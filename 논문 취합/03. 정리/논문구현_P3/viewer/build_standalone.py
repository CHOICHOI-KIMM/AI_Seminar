# -*- coding: utf-8 -*-
"""자립판 빌더 — micropitting_viewer.html (단일 파일, file:// 더블클릭 실행).

원리 (작업결과 §자립판): fetch 를 전부 제거한다 —
  - wasm  : base64 내장 → initSync({module}) 동기 인스턴스화 (fetch 없음)
  - glue  : export 제거 후 인라인
  - worker: 본문을 **verbatim** 인라인해 가짜 worker 객체로 구동 (메인스레드; SSOT=worker.js)
  - plot.js / vc_data.json: 인라인
자립판은 이 스크립트의 **순수 생성물** — 수기 편집 금지 (1층 SSOT).

사용:
  python build_standalone.py          # 생성
  python build_standalone.py --check  # 신선도 검사 (내장 해시 vs 현 소스, 불일치 exit 1) — 3층
"""
import base64
import datetime
import hashlib
import io
import os
import re
import sys

# Windows 콘솔(cp949)에서 유니코드 출력 안전화
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "micropitting_viewer.html")
SRCS = ["index.html", "plot.js", "worker.js", "vc_data.json",
        os.path.join("pkg", "micropitting_wasm.js"),
        os.path.join("pkg", "micropitting_wasm_bg.wasm")]


def read(p, binary=False):
    with open(os.path.join(HERE, p), "rb" if binary else "r",
              encoding=None if binary else "utf-8") as f:
        return f.read()


def input_hash():
    h = hashlib.sha256()
    for p in SRCS:
        h.update(read(p, binary=True))
    return h.hexdigest()[:12]


def build():
    html = read("index.html")
    plot = read("plot.js")
    worker = read("worker.js")
    vc = read("vc_data.json").strip()
    glue = read(os.path.join("pkg", "micropitting_wasm.js"))
    wasm_b64 = base64.b64encode(read(os.path.join("pkg", "micropitting_wasm_bg.wasm"), binary=True)).decode()

    ih = input_hash()
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")

    # ── glue: export 구문 제거 (동일 모듈 스코프 인라인) ──
    glue = glue.replace("export function ", "function ")
    glue = re.sub(r"^export default .*$", "", glue, flags=re.M)
    glue = re.sub(r"^export \{[^}]*\};?\s*$", "", glue, flags=re.M)
    # 주의: `instance.exports` 같은 속성명은 무해 — **구문**(행 시작 export)만 검사
    assert not re.search(r"^export ", glue, flags=re.M), "glue 에 미처리 export 구문 잔존"

    # ── plot.js: export 제거 ──
    plot = plot.replace("export function ", "function ")

    # ── worker.js 본문: import 블록 제거 + init() → (이미 initSync 완료) ──
    w = re.sub(r"^import init, \{[^}]*\} from \"\./pkg/micropitting_wasm\.js\";\s*$",
               "", worker, flags=re.M | re.S)
    w = re.sub(r"^import init, \{.*?\} from \"\./pkg/micropitting_wasm\.js\";",
               "", w, flags=re.S)  # 멀티라인 import 형
    assert "import init" not in w, "worker import 미제거"
    w2 = re.sub(r"^const ready = init\(\).*$",
                'const ready = Promise.resolve(); wstep("initDone");', w, flags=re.M)
    assert w2 != w, "worker ready 패치 실패"
    w = w2

    # ── 1) <title> ──
    html = html.replace("<title>Micropitting P3 뷰어 — M1~M6 정적 체인</title>",
                        "<title>Micropitting P3 뷰어 (자립판) — M1~M6 정적 체인</title>")
    # 헤더 sub 에 빌드 정보
    old = 'JS 물리식 0건 · 스프린트판(P3_HTML_spike)'
    assert old in html
    html = html.replace(old, f'JS 물리식 0건 · <b>자립판</b> 빌드 {now} · src {ih}', 1)

    # ── 2) classic file:// 가드 → 자립판은 file:// 가 정상 경로이므로 제거 ──
    m = re.search(r"<script>\n// file:// 가드.*?</script>\n", html, flags=re.S)
    assert m, "file:// 가드 블록 못 찾음"
    html = html.replace(m.group(0), "", 1)

    # ── 3) plot.js import → 인라인 ──
    old = 'import { linePlot, heatmap } from "./plot.js";'
    assert old in html
    html = html.replace(old, "// ═ plot.js 인라인 (빌드 생성물) ═\n" + plot, 1)

    # ── 4) 신선도 가드(fetch 기반) → 빌드 정보 배너 (자립판 3층: 해시는 --check 가 검사) ──
    m = re.search(r"// ── 신선도 가드.*?catch \(e\) \{ if \(String\(e\)\.includes\(\"stale\"\)\) throw e; \}",
                  html, flags=re.S)
    assert m, "신선도 가드 블록 못 찾음"
    html = html.replace(m.group(0),
                        f'// 자립판: fetch 기반 신선도 가드 불가 → 빌드 정보 표시(검사는 build_standalone.py --check)\n'
                        f'const __BUILD_INFO__ = {{ time: "{now}", hash: "{ih}" }};\n'
                        f'boot.textContent = `자립판 로딩… (빌드 ${{__BUILD_INFO__.time}} · src ${{__BUILD_INFO__.hash}})`;', 1)

    # ── 5) Worker → 인라인 가짜 worker (메인스레드; worker.js 본문 verbatim = SSOT 유지) ──
    old = 'const worker = new Worker("./worker.js", { type: "module" });'
    assert old in html
    engine = (
        "// ═ 자립판 인라인 엔진 (빌드 생성물 — 수기 편집 금지) ═\n"
        "// glue(export 제거) + wasm base64 + worker.js 본문 verbatim. fetch 0건.\n"
        + glue + "\n"
        + 'const __WASM_B64 = "' + wasm_b64 + '";\n'
        + "function __wasmBytes() { const b = atob(__WASM_B64); const u = new Uint8Array(b.length);\n"
        + "  for (let i = 0; i < b.length; i++) u[i] = b.charCodeAt(i); return u; }\n"
        + "initSync({ module: __wasmBytes() }); // 동기 인스턴스화 — fetch·비동기 컴파일 없음\n"
        + "const worker = (() => {\n"
        + "  const selfObj = {};\n"
        + "  (function (self) {\n" + w + "\n  })(selfObj);\n"
        + "  const fake = {};\n"
        + "  // setTimeout 양보: 무거운 동기 계산 전에 '계산 중' 버튼 페인트 기회 부여\n"
        + "  fake.postMessage = (data) => setTimeout(() => selfObj.onmessage({ data }), 15);\n"
        + "  selfObj.postMessage = (msg) => fake.onmessage && fake.onmessage({ data: msg });\n"
        + "  return fake;\n"
        + "})();"
    )
    html = html.replace(old, engine, 1)

    # ── 6) vc_data fetch → 인라인 ──
    old = 'const d = await (await fetch("./vc_data.json")).json();'
    assert old in html
    html = html.replace(old, "const d = __VC_DATA__;", 1)
    html = html.replace("// ── ② 검증 탭 ──",
                        "// ── ② 검증 탭 ──\nconst __VC_DATA__ = " + vc + ";", 1)

    # ── 해시 마커 (--check 용) ──
    html += f"\n<!-- STANDALONE_SRC_HASH:{ih} -->\n"

    with io.open(OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(html)
    print(f"자립판 생성: {os.path.basename(OUT)} ({os.path.getsize(OUT)//1024} KB) · src {ih} · {now}")


def check():
    if not os.path.exists(OUT):
        print("[check] 자립판 부재 → 빌드 필요")
        return 1
    cur = input_hash()
    m = re.search(r"STANDALONE_SRC_HASH:([0-9a-f]+)", read(os.path.basename(OUT)))
    emb = m.group(1) if m else "(마커 없음)"
    if emb != cur:
        print(f"[check] STALE — 자립판 {emb} ≠ 현 소스 {cur} → python build_standalone.py 재실행")
        return 1
    print(f"[check] OK — 자립판 = 현 소스 ({cur})")
    return 0


if __name__ == "__main__":
    sys.exit(check() if "--check" in sys.argv else build() or 0)

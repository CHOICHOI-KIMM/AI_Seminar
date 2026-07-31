"""
§5-9.3 표 자동 재생성
=======================
fullres_per_point.csv 를 읽어 모델 × 케이스 집계표를 다시 쓴다.
C0 는 lub_per_bin.csv(부록 5) 에서 같은 DLC 를 뽑아 쓴다 — 재해석 불필요.

집계는 **회전수 가중평균**(rev = |n|/60 · Δt) 이며 UW 기준이다(§5-2 방침).
표 위치는 절 제목 + 정규식으로 찾고 패턴에 헤더 내용을 넣지 않는다
(헤더 조각을 패턴에 남기면 열 변경 시 영구 실패한다 — 260730 사고).
"""
import csv
import io
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(HERE, "부록5_윤활조건")
SRC = os.path.join(DIR, "fullres_per_point.csv")
BIN = os.path.join(DIR, "lub_per_bin.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
DLC = "DLC2.4.2-a-s1"
SECT = "### 5-9.3 결과"
MODELS = ("A1", "B1")
CASES = (("C0", "빈 rpm + 빈 하중"),
         ("C1", "**점 rpm** + 빈 하중"),
         ("C2", "**점 rpm + 점 하중**"))
NPT = 3001
HDR = ("| 모델 | 케이스 | 구성 | n | rpm | κ | λ_in | a_ISO | "
       "Δκ vs C0 | Δa_ISO vs C0 |")
SEP = "|:-:|:-:|---" + "|--:" * 7 + "|"
PAT = r"^\| 모델 \| 케이스.*?(?=\n\n)"
VARS = ("rpm", "kappa", "lambda_in", "a_iso")


def rd(p):
    if not os.path.isfile(p):
        return []
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def wmean(g, key):
    n = w = 0.0
    for r in g:
        try:
            v, ww = float(r[key]), float(r["rev"])
        except (TypeError, ValueError):
            continue
        n += v * ww
        w += ww
    return n / w if w else None


def collect():
    """(model, case) -> dict(n, rpm, kappa, lambda_in, a_iso)"""
    acc = {}
    for r in rd(BIN):
        if r["DLC"] == DLC and r["brg"] == "UW" and r["model"] in MODELS:
            acc.setdefault((r["model"], "C0"), []).append(r)
    for r in rd(SRC):
        if r["brg"] == "UW":
            acc.setdefault((r["model"], r["case"]), []).append(r)
    return {k: dict(n=len(g), **{v: wmean(g, v) for v in VARS})
            for k, g in acc.items()}


def build():
    acc = collect()
    lines = [HDR, SEP]
    done = 0
    for m in MODELS:
        base = acc.get((m, "C0"))
        for c, desc in CASES:
            a = acc.get((m, c))
            if not a or a["kappa"] is None:
                lines.append(f"| {m} | {c} | {desc} | · | · | · | · | · | · | · |")
                continue
            if c != "C0":
                done += a["n"]
            dk = da = "—"
            if base and base["kappa"] and c != "C0":
                dk = f"{100*(a['kappa']/base['kappa']-1):+.2f}%"
                da = f"{100*(a['a_iso']/base['a_iso']-1):+.2f}%"
            tail = "" if c == "C0" and a["n"] >= 15 else (
                "" if a["n"] >= NPT or c == "C0" else " ⏳")
            lines.append(
                f"| {m} | **{c}** | {desc} | {a['n']:,}{tail} | "
                f"{a['rpm']:.4f} | {a['kappa']:.4f} | {a['lambda_in']:.4f} | "
                f"{a['a_iso']:.4f} | {dk} | {da} |")
    return "\n".join(lines), done, len(MODELS) * 2 * NPT


def main():
    tbl, done, total = build()
    s = io.open(DOC, encoding="utf-8").read()
    base = s.index(SECT)
    m = re.search(PAT, s[base:], re.S | re.M)
    if not m:
        raise RuntimeError("§5-9.3 표를 찾지 못했다")
    s = s[:base + m.start()] + tbl + s[base + m.end():]
    txt = (f"*(수행 중 {done:,}/{total:,}점 — 500점마다 자동 갱신)*"
           if done < total else f"*(완료 {total:,}점)*")
    i = s.index(SECT)
    s = s[:i] + re.sub(r"^\*\((대기|수행|완료)[^\n]*\)\*$", txt, s[i:], count=1,
                       flags=re.M)
    io.open(DOC, "w", encoding="utf-8").write(s)
    return done, total


if __name__ == "__main__":
    d, t = main()
    print(f"[문서 갱신] §5-9.3  {d:,}/{t:,}점")

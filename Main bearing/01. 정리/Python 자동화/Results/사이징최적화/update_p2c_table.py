"""
P2 Phase 3 — 문서 §8-6.1(C) · §8-6.2(D) 표 자동 재생성
============================================================
베어링 단독 질량 기준 최경량 6건(§8-4.4.1·§8-4.4.2 상위 3)의 피로 결과.
fatigue_summary.csv 를 읽어 집합별 표를 다시 쓴다.
run_p2_fatigue.py 3 이 설계 1건 완료마다 호출한다. 미완료는 `·`.

열 구성은 §8-5.5.1·§8-5.5.2 와 동일하게 두되, 베어링 질량이 이 절의
정렬 기준이므로 그 열만 굵게 둔다.

표 위치는 절 제목 + 정규식으로 찾고, 패턴에 헤더 내용을 넣지 않는다
(헤더 조각을 패턴에 남기면 열 변경 시 영구 실패 — 260730 사고).
탐색 범위는 다음 절까지로 제한한다(260802 사고).
"""
import csv
import io
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import sizing_geom as sg   # noqa: E402
DIR = os.path.join(HERE, "P2_피로수명_Phase3")
CONST = os.path.join(DIR, "p2c_constants.csv")
TGT = os.path.join(DIR, "p2c_targets.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SECT = {"C": "#### 8-6.1", "D": "#### 8-6.2"}
LIMIT = 0.5
# 기준선 — P2 Phase 1 재사용. 제원은 v1.3, 질량은 §4-3 MASTA 실측.
BASE = dict(D_pw_mm=3330.9, alpha=19.0, D_we_mm=110.51, L_we_mm=238.048,
            z1=0.5, z2=3.0, Z=87, L_eff_m=3.616662,
            mass_brg=5600.5, mass_shaft=43225.8, sigma=3424.2,
            D30_UW=6.9249, D30_DW=1.1421, D30_Sys=7.7297, life_Sys=3.8811)
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | B [mm] | C [mm] | T [mm] | α [°] | "
       "D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | L_eff [m] | "
       "**베어링** [t] | 샤프트 [t] | 합계 [t] | σ_max [MPa] | "
       "**ΣD30_UW** | **ΣD30_DW** | ΣD30_Sys | life_Sys [yr] | **판정** |")
SEP = "|--:" * 23 + "|:-:|"

def _next_head(s, base):
    """base 다음 제목 줄의 위치. 없으면 문서 끝.
    레벨을 고정하면 절이 문서 마지막일 때 실패한다(260803)."""
    m = re.search(r"^#{1,6} ", s[base + 5:], re.M)
    return base + 5 + m.start() if m else len(s)

# 편집기가 표를 정렬하면 `| # |` 이 `|  # |` 로 패딩된다 — 여백 허용(260805)
PAT = r"^\|\s*#\s*\|\s*D_pw.*?(?=\n\n)"


def load(p):
    if not os.path.isfile(p):
        return []
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def build():
    res = {r["design"]: r for r in load(os.path.join(DIR, "fatigue_summary.csv"))}
    tgt = {r["rank_mass"]: r for r in load(TGT)}
    out, done = {}, 0
    for tag in ("C", "D"):
        lines = [HDR, SEP]
        rows = [c for c in load(CONST) if c["rank_mass"].startswith(tag)]
        for c in rows:
            k = c["rank_mass"]
            t = tgt.get(k, {})
            r = res.get(k)
            if r:
                done += 1
                du, dd = float(r["D30_UW"]), float(r["D30_DW"])
                ds = float(r["D30_Sys"])
                ok = du <= LIMIT and ds <= LIMIT
                cells = (f"**{du:.4f}**", f"{dd:.4f}", f"{ds:.4f}",
                         f"{float(r['life_Sys_yr']):,.1f}",
                         "**합격**" if ok else "**불합격**")
            else:
                cells = ("·",) * 5
            dpw, al = float(c["D_pw_mm"]), float(c["alpha"])
            dwe, lwe = float(c["D_we_mm"]), float(c["L_we_mm"])
            g = sg.bearing(dpw / 1e3, al, dwe / 1e3, lwe / 1e3)
            mb = float(t["mass_brg_kg"]) / 1000 if t else 0.0
            ms = float(t["mass_shaft_kg"]) / 1000 if t else 0.0
            lines.append(
                f"| {k} | {dpw:,.0f} | {g['bore']*1e3:,.0f} | "
                f"{g['outer_diameter']*1e3:,.0f} | "
                f"{g['inner_ring_width']*1e3:,.1f} | "
                f"{g['outer_ring_width']*1e3:,.1f} | {g['width']*1e3:,.1f} | "
                f"{al:.0f} | {dwe:.0f} | {lwe:.0f} | {lwe/dwe:.3f} | "
                f"{float(c['z1']):.1f} | {float(c['z2']):.1f} | {c['Z']} | "
                f"{float(c['L_eff_m']):.3f} | **{mb:.2f}** | {ms:.1f} | "
                f"{2*mb+ms:.1f} | "
                f"{(float(t['sigma_max_MPa']) if t else 0):,.1f} | "
                + " | ".join(cells) + " |")

        b = BASE                       # 기준선 — 표 최하단
        gb = sg.bearing(b["D_pw_mm"] / 1e3, b["alpha"],
                        b["D_we_mm"] / 1e3, b["L_we_mm"] / 1e3)
        mb, ms = b["mass_brg"] / 1000, b["mass_shaft"] / 1000
        lines.append(
            f"| **기준선** | {b['D_pw_mm']:,.0f} | {gb['bore']*1e3:,.0f} | "
            f"{gb['outer_diameter']*1e3:,.0f} | "
            f"{gb['inner_ring_width']*1e3:,.1f} | "
            f"{gb['outer_ring_width']*1e3:,.1f} | {gb['width']*1e3:,.1f} | "
            f"{b['alpha']:.0f} | {b['D_we_mm']:.0f} | {b['L_we_mm']:.0f} | "
            f"{b['L_we_mm']/b['D_we_mm']:.3f} | {b['z1']:.1f} | {b['z2']:.1f} | "
            f"{b['Z']} | {b['L_eff_m']:.3f} | **{mb:.2f}** | {ms:.1f} | "
            f"{2*mb+ms:.1f} | {b['sigma']:,.1f} | **{b['D30_UW']:.4f}** | "
            f"{b['D30_DW']:.4f} | {b['D30_Sys']:.4f} | {b['life_Sys']:,.1f} | "
            f"**불합격** |")
        out[tag] = "\n".join(lines)
    return out, done, len(load(CONST))


def swap(s, sect, tbl):
    base = s.index(sect)
    nxt = _next_head(s, base)
    m = re.search(PAT, s[base:nxt], re.S | re.M)
    if not m:
        raise RuntimeError(f"{sect} 표를 찾지 못했다 — 자리표 표가 있어야 한다")
    return s[:base + m.start()] + tbl + s[base + m.end():]


def main():
    tbls, done, total = build()
    s = io.open(DOC, encoding="utf-8").read()
    for tag in ("D", "C"):            # 뒤에서 앞으로 — 인덱스 밀림 방지
        s = swap(s, SECT[tag], tbls[tag])
    txt = (f"*(수행 중 {done}/{total} — 설계 완료마다 자동 갱신)*" if done < total
           else f"*(완료 {total}/{total})*")
    i = s.index("### 8-6. P2 —")
    s = s[:i] + re.sub(r"^\*\((대기|수행|완료)[^\n]*\)\*$", txt, s[i:], count=1,
                       flags=re.M)
    io.open(DOC, "w", encoding="utf-8").write(s)
    return done, total


if __name__ == "__main__":
    d, t = main()
    print(f"[문서 갱신] §8-6  {d}/{t} 완료")

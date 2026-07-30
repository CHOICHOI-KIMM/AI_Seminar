"""
P2 Phase 2 — 문서 §8-5.5.1(A) · §8-5.5.2(B) 표 자동 재생성
============================================================
fatigue_summary.csv 를 읽어 집합별 표를 다시 쓴다.
run_p2_fatigue.py 2 가 설계 1건 완료마다 호출한다. 미완료는 `·`.

표 위치는 절 제목 + 정규식으로 찾고, 패턴에 헤더 내용을 넣지 않는다
(헤더 조각을 패턴에 남기면 열 변경 시 영구 실패한다 — 260730 사고).
"""
import csv
import io
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import sizing_geom as sg   # noqa: E402
DIR = os.path.join(HERE, "P2_피로수명_Phase2")
CONST = os.path.join(DIR, "p2b_constants.csv")
TGT = os.path.join(DIR, "p2b_targets.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SECT = {"A": "##### 8-5.5.1", "B": "##### 8-5.5.2"}
LIMIT = 0.5
# 기준선 — P2 Phase 1 재사용(§8-5.3). 제원은 v1.3, 질량은 §4-3 MASTA 실측.
BASE = dict(D_pw_mm=3330.9, alpha=19.0, D_we_mm=110.51, L_we_mm=238.048,
            z1=0.5, z2=3.0, Z=87, L_eff_m=3.616662,
            mass_brg=5600.5, mass_shaft=43225.8, sigma=3424.2,
            D30_UW=6.9249, D30_DW=1.1421, D30_Sys=7.7297, life_Sys=3.8811)
# §8-2.5.1 · §8-3.6.2 · §8-4.3.2 와 동일 제원열 + 피로 4열 (24열)
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | B [mm] | C [mm] | T [mm] | α [°] | "
       "D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | L_eff [m] | "
       "질량 [t] | 베어링 [t] | 샤프트 [t] | σ_max [MPa] | "
       "**ΣD30_UW** | **ΣD30_DW** | ΣD30_Sys | life_Sys [yr] | **판정** |")
SEP = "|--:" * 23 + "|:-:|"
PAT = r"^\| # \| D_pw.*?(?=\n\n)"


def load(p):
    if not os.path.isfile(p):
        return []
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def build():
    res = {r["design"]: r for r in load(os.path.join(DIR, "fatigue_summary.csv"))}
    tgt = {r["rank_mass"]: r for r in load(TGT)}
    out, done = {}, 0
    for tag in ("A", "B"):
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
            lines.append(
                f"| {k} | {dpw:,.0f} | {g['bore']*1e3:,.0f} | "
                f"{g['outer_diameter']*1e3:,.0f} | "
                f"{g['inner_ring_width']*1e3:,.1f} | "
                f"{g['outer_ring_width']*1e3:,.1f} | {g['width']*1e3:,.1f} | "
                f"{al:.0f} | {dwe:.0f} | {lwe:.0f} | {lwe/dwe:.3f} | "
                f"{float(c['z1']):.1f} | {float(c['z2']):.1f} | {c['Z']} | "
                f"{float(c['L_eff_m']):.3f} | "
                f"{(float(t['mass_total_kg'])/1000 if t else 0):.1f} | "
                f"{(float(t['mass_brg_kg'])/1000 if t else 0):.1f} | "
                f"{(float(t['mass_shaft_kg'])/1000 if t else 0):.1f} | "
                f"{(float(t['sigma_max_MPa']) if t else 0):,.1f} | "
                + " | ".join(cells) + " |")

        # 기준선 — 표 최하단 (P2 Phase 1 재사용)
        b = BASE
        gb = sg.bearing(b["D_pw_mm"] / 1e3, b["alpha"],
                        b["D_we_mm"] / 1e3, b["L_we_mm"] / 1e3)
        mt = 2 * b["mass_brg"] + b["mass_shaft"]
        lines.append(
            f"| **기준선** | {b['D_pw_mm']:,.0f} | {gb['bore']*1e3:,.0f} | "
            f"{gb['outer_diameter']*1e3:,.0f} | "
            f"{gb['inner_ring_width']*1e3:,.1f} | "
            f"{gb['outer_ring_width']*1e3:,.1f} | {gb['width']*1e3:,.1f} | "
            f"{b['alpha']:.0f} | {b['D_we_mm']:.0f} | {b['L_we_mm']:.0f} | "
            f"{b['L_we_mm']/b['D_we_mm']:.3f} | {b['z1']:.1f} | {b['z2']:.1f} | "
            f"{b['Z']} | {b['L_eff_m']:.3f} | {mt/1000:.1f} | "
            f"{b['mass_brg']/1000:.1f} | {b['mass_shaft']/1000:.1f} | "
            f"{b['sigma']:,.1f} | **{b['D30_UW']:.4f}** | {b['D30_DW']:.4f} | "
            f"{b['D30_Sys']:.4f} | {b['life_Sys']:,.1f} | **불합격** |")
        out[tag] = "\n".join(lines)
    return out, done, len(load(CONST))


def swap(s, sect, tbl):
    base = s.index(sect)
    m = re.search(PAT, s[base:], re.S | re.M)
    if not m:
        raise RuntimeError(f"{sect} 표를 찾지 못했다")
    return s[:base + m.start()] + tbl + s[base + m.end():]


def main():
    tbls, done, total = build()
    s = io.open(DOC, encoding="utf-8").read()
    for tag in ("B", "A"):        # 뒤에서 앞으로 — 인덱스 밀림 방지
        s = swap(s, SECT[tag], tbls[tag])
    txt = (f"*(수행 중 {done}/{total} — 설계 완료마다 자동 갱신)*" if done < total
           else f"*(완료 {total}/{total})*")
    i = s.index("#### 8-5.5 결과")
    s = s[:i] + re.sub(r"^\*\((수행|완료)[^\n]*\)\*$", txt, s[i:], count=1,
                       flags=re.M)
    io.open(DOC, "w", encoding="utf-8").write(s)
    return done, total


if __name__ == "__main__":
    d, t = main()
    print(f"[문서 갱신] §8-5.5  {d}/{t} 완료")

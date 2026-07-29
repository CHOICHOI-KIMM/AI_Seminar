"""
P2 Phase 1 — 문서 §8-3.6.2 결과표 자동 재생성
==============================================
fatigue_summary.csv 를 읽어 사이징 최적화 문서의 §8-3.6.2 표를 다시 쓴다.
설계 1건이 끝날 때마다 run_p2_fatigue.py 가 호출한다. 미완료 설계는 `·` 로 남는다.

표 위치는 **절 제목으로 찾는다** — 헤더 문자열에 의존하면 열을 늘릴 때
실행 중인 프로세스(구 모듈)가 표를 못 찾아 죽는다. (260729 사고)

열 구성은 §8-2.5.1 최경량 표와 동일하게 d·D·세장비를 포함한다.
d·D 는 sizing_geom(SSOT)으로 산출하므로 기준선도 같은 규칙으로 나온다.
"""
import csv
import io
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import sizing_geom as sg   # noqa: E402

DIR = os.path.join(HERE, "P2_피로수명_Phase1")
CONST = os.path.join(HERE, "P1_극한응력_Phase2", "p2_constants.csv")
FEAS = os.path.join(HERE, "P1_극한응력_Phase2", "p1_feasible.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SECT = "##### 8-3.6.2"
LIMIT = 0.5
HDR = ("| # | D_pw | d | D | α | D_we | L_we | 세장비 | z1 | z2 | 질량 | σ_max | "
       "**ΣD30_UW** | **ΣD30_DW** | life_Sys | **판정** |")
SEP = "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|"


def load(path):
    if not os.path.isfile(path):
        return []
    with open(path, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def build():
    res = {r["design"]: r for r in load(os.path.join(DIR, "fatigue_summary.csv"))}
    feas = {r["rank_mass"]: r for r in load(FEAS)}
    specs = load(CONST)
    lines = [HDR, SEP]
    done = 0
    for c in specs:
        tag = c["rank_mass"]
        lbl = "기준선" if tag == "base" else tag
        dpw, al = float(c["D_pw_mm"]), float(c["alpha"])
        dwe, lwe = float(c["D_we_mm"]), float(c["L_we_mm"])
        g = sg.bearing(dpw / 1e3, al, dwe / 1e3, lwe / 1e3)
        f = feas.get(tag, {})
        mass = (f"{float(f['mass_total_kg'])/1000:.1f} t" if f.get("mass_total_kg")
                else "54.4 t")
        sig = (f"{float(f['sigma_max_MPa']):,.1f}" if f.get("sigma_max_MPa")
               else "3,424.2")
        r = res.get(tag)
        if r:
            done += 1
            du, dd = float(r["D30_UW"]), float(r["D30_DW"])
            ok = du <= LIMIT and float(r["D30_Sys"]) <= LIMIT
            cells = (f"**{du:.4f}**", f"{dd:.4f}",
                     f"{float(r['life_Sys_yr']):,.1f} yr",
                     "**합격**" if ok else "**불합격**")
        else:
            cells = ("·", "·", "·", "·")
        lines.append(
            f"| {lbl} | {dpw:,.0f} | {g['bore']*1e3:,.0f} | "
            f"{g['outer_diameter']*1e3:,.0f} | {al:.0f} | {dwe:.0f} | {lwe:.0f} | "
            f"{lwe/dwe:.3f} | {float(c['z1']):.1f} | {float(c['z2']):.1f} | "
            f"{mass} | {sig} | " + " | ".join(cells) + " |")
    return "\n".join(lines), done, len(specs)


def main():
    tbl, done, total = build()
    s = io.open(DOC, encoding="utf-8").read()
    base = s.index(SECT)
    # 절 안의 첫 표를 찾아 통째로 교체 (헤더 문자열 비의존)
    m = re.search(r"^\| # \| D_pw \|.*?(?=\n\n)", s[base:], re.S | re.M)
    if not m:
        raise RuntimeError("§8-3.6.2 표를 찾지 못했다")
    i, j = base + m.start(), base + m.end()
    s = s[:i] + tbl + s[j:]
    # 진행 표시줄 — 위치가 아니라 패턴으로 찾는다
    txt = (f"*(진행 {done}/{total} — 설계 완료 시마다 갱신)*" if done < total
           else f"*(완료 {total}/{total})*")
    s = re.sub(r"^\*\((진행|완료|수행)[^\n]*\)\*$", txt, s, count=1, flags=re.M)
    io.open(DOC, "w", encoding="utf-8").write(s)
    return done, total


if __name__ == "__main__":
    d, t = main()
    print(f"[문서 갱신] §8-3.6.2  {d}/{t} 완료")

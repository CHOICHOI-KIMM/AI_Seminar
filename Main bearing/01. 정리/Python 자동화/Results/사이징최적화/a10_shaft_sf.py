# -*- coding: utf-8 -*-
"""§10-11.2.2 — #1 · #103 · #210 의 DIN 743 축 피로안전율

하중 기준은 **극한 LC 가 아니다.** 부록 7·8 이 정한 대로
`DLC1.2-k-s5` 의 **최약 빈 #11** 대표하중(`부록7_샤프트/worst_bin.csv`)을
쓴다 — 두께 규칙 `W` = 1.393e9 mm^3 를 유도한 바로 그 하중이며(§7-6.7.5),
극한 LC `Myz_max` 는 실운전 대비 과대하고 두께 규칙의 근거 하중도 아니라
쓰지 않는다(§8-6.2).

형상은 §10-11.2.2 기록 그대로(현행 `element_offset` · 양쪽 DIN)라 표의
다른 열과 상태가 같다. 안전율은 `ShaftSystemDeflection.safety_factors.
items[]` 에서 전 항목을 읽고 **최소값과 그 항목명**을 남긴다.
"""
import csv
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "shaft_sf")
RANKS = (1, 103, 210)


def main(ranks):
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    import masta_fatigue as mf
    BIN = os.path.join(HERE, "부록7_샤프트", "worst_bin.csv")
    rec = next(csv.DictReader(open(BIN, encoding="utf-8-sig")))
    rec = {k: float(v) for k, v in rec.items() if k != "DLC"}
    print(f"[하중] DLC1.2-k-s5 최약 빈 #{int(rec['bin'])} · "
          f"{rec['rpm']:.4f} rpm · Mx {rec['Mx']/1e3:.1f} · "
          f"My {rec['My']/1e3:.1f} · Mz {rec['Mz']/1e3:.1f} kNm")

    rig = L.Rig()
    asm = rig.asm
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(q for q in asm.all_parts_of_type_power_load()
               if "input" in str(q).lower())
    lc0 = next(c for c in rig.dp.static_loads if c.name == "Load Case 1")
    ds0 = lc0.design_state_load_case_group
    rig.lc = lc0
    rows = []
    for rk in ranks:
        rig.build(P[rk])
        rig.set_din(0.0)                  # §10-11.2.2 기록 상태
        dup = lc0.duplicate(ds0, f"sf_{rk}")
        mf.set_loads(dup, pl, ipl, rec)
        duty = rig.dp.add_duty_cycle(f"sfd_{rk}")
        duty.add_static_load(dup)
        csd = duty.analysis_of(rig.AT.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        sub = list(list(csd.results_for(rig.sh))[0]
                   .component_analysis_cases)[0]
        items = []
        for it in sub.safety_factors.items:
            try:
                items.append((str(it.description),
                              float(it.safety_factor),
                              float(it.minimum_required_safety_factor)))
            except Exception:
                pass
        for x in (dup, duty):
            try:
                x.delete()
            except Exception:
                pass
        if not items:
            print(f"  #{rk}: 안전율 항목 없음")
            continue
        nm, sf, req = min(items, key=lambda t: t[1])
        r = P[rk]
        rows.append(dict(rank=rk, D_mm=r["D_mm"], bore_mm=r["bore_mm"],
                         sf_min=round(sf, 3), sf_item=nm,
                         sf_required=req, n_items=len(items)))
        print(f"\n#{rk}  D {r['D_mm']} · d {r['bore_mm']} · 항목 "
              f"{len(items)}개")
        for d, v, q in sorted(items, key=lambda t: t[1]):
            mark = " ←최소" if d == nm else ""
            print(f"    {d:44s} {v:9.3f}  (요구 {q:.2f}){mark}")
    with open(os.path.join(OUT, "shaft_sf.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {OUT}")


if __name__ == "__main__":
    main([int(x) for x in sys.argv[1:]] or list(RANKS))

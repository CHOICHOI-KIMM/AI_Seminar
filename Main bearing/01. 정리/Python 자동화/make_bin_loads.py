"""
부록 8: dt별 빈(bin) 평균 하중 산출 → 단일 CSV (MASTA 불필요)
=============================================================
구간 [t_i, t_i+dt) 내 원 시계열(Δt₀=0.1 s) 하중 6분력·rpm 을 산술평균.
전 dt 를 한 파일에 누적 → Results/bin_loads_all_dt.csv

열: dt_s, bin_index, t_start_s, t_end_s, n_points, rpm_avg,
    force_x_N, force_y_N, axial_load_N, moment_x_Nm, moment_y_Nm, Moment_z_Nm, rev_N
※ 하중 부호·성분은 §4.2 좌표변환 적용 후 MASTA 인가값과 동일 (단위 N, N·m)
"""
import csv
import os

from c1_pin import DLC, hub, parse_dlc

DTS = [60, 20, 10, 6, 4, 2, 1, 0.6, 0.4, 0.3, 0.2, 0.1]
DT0 = 0.1
OUT = os.path.join("Results", "bin_loads_all_dt.csv")
HDR = ["dt_s", "bin_index", "t_start_s", "t_end_s", "n_points", "rpm_avg",
       "force_x_N", "force_y_N", "axial_load_N",
       "moment_x_Nm", "moment_y_Nm", "Moment_z_Nm", "rev_N"]
KEYS = ["FX", "FY", "FZ", "MX", "MY", "MZ"]


def bins_of(data, dt):
    """→ [(i0, i1)] 마지막 잔여 구간은 마지막 빈에 편입."""
    k = int(round(dt / DT0))
    n = len(data)
    nb = max(1, n // k)
    out = [(b * k, (b + 1) * k) for b in range(nb)]
    if out[-1][1] < n:                       # 잔여 편입
        out[-1] = (out[-1][0], n)
    return out


def main():
    data = parse_dlc(DLC)
    H = [hub(r) for r in data]
    os.makedirs("Results", exist_ok=True)
    rows = 0
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(HDR)
        for dt in DTS:
            for bi, (i0, i1) in enumerate(bins_of(data, dt)):
                k = i1 - i0
                avg = [sum(H[i][x] for i in range(i0, i1)) / k for x in KEYS]
                rpm = sum(data[i]["rpm"] for i in range(i0, i1)) / k
                t0 = round(i0 * DT0, 4)
                span = round(k * DT0, 4)
                w.writerow(["%g" % dt, bi, f"{t0:.4f}", f"{t0+span:.4f}", k,
                            f"{rpm:.6f}"] + [f"{v:.6f}" for v in avg]
                           + [f"{rpm / 60.0 * span:.8f}"])
                rows += 1
            print(f"  dt={dt:<5g} 빈 {bi+1:5d}개")
    print(f"\n[저장] {OUT}   총 {rows:,}행")

    # 검산: dt별 회전수 총합이 일치해야 함(= 전체 회전수)
    tot = {}
    for r in csv.DictReader(open(OUT, encoding="utf-8-sig")):
        tot[r["dt_s"]] = tot.get(r["dt_s"], 0.0) + float(r["rev_N"])
    ref = tot["0.1"]
    print(f"\n[검산] 총 회전수 (기준 dt=0.1: {ref:.6f} Rev)")
    for k, v in tot.items():
        print(f"  dt={k:<5} ΣN = {v:.6f} Rev   편차 {(v/ref-1)*100:+.4f}%")


if __name__ == "__main__":
    main()

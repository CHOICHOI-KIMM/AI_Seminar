"""
DLC별 대표하중 CSV 생성 (원본 좌표계)
=====================================
111 DLC 각각에 대해 dt=20 s 로 재구간화하고, 부록 4 의 중앙타겟 k 를 적용한
μ+kσ 대표하중을 원본 좌표계(raw.csv 와 동일)로 기록한다.

  · 6자유도 전부 μ+kσ  (Fx, Fy, Fz, Mx, My, Mz)  ← 260729 결정: 일관성
    (주: 실제 피로해석 인가 토크는 Mx 빈평균이었음 — 본 CSV 는 대표하중 정의 통일본)
  · rpm 은 빈 평균, rev = |rpm|/60 × dt
  · 단위 : F [kN] · M [kNm]  (raw.csv 원단위 유지)
  · 열   : index, t_s, rpm, rev, Fx, Fy, Fz, Mx, My, Mz  (A~J)
  · UW/DW 구분 없음 — 허브 하중 1행/빈
"""
import csv
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
KSRC = os.path.join(HERE, "부록4_screening_dt20", "per_dlc.csv")
OUTDIR = os.path.join(HERE, "대표하중_원본좌표")
DT0, DT = 0.1, 20.0
DOF = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")      # 전 성분 μ+kσ
COLS = ["index", "t_s", "rpm", "rev"] + list(DOF)


def load_raw(name):
    with open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig") as f:
        return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(f)]


def bin_reps(data, dt, k):
    kp = int(round(dt / DT0))
    n = len(data)
    nb = max(n // kp, 1)
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges and edges[-1][1] < n:                 # 잔여점은 마지막 빈에 흡수
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rpm = sum(data[i]["rpm"] for i in range(i0, i1)) / m
        row = dict(index=bi, t_s=round(data[i0]["t"], 3), rpm=round(rpm, 5),
                   rev=round(abs(rpm) / 60.0 * (m * DT0), 5))
        for key in DOF:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            row[key] = round(mu + math.copysign(1.0, mu) * k * math.sqrt(var), 3)
        out.append(row)
    return out


def main():
    os.makedirs(OUTDIR, exist_ok=True)
    with open(KSRC, encoding="utf-8-sig") as f:
        kmap = {r["DLC"]: float(r["k"]) for r in csv.DictReader(f)}
    idx = []
    for i, (name, k) in enumerate(sorted(kmap.items()), 1):
        raw = load_raw(name)
        rows = bin_reps(raw, DT, k)
        path = os.path.join(OUTDIR, f"{name}.csv")
        with open(path, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=COLS)
            w.writeheader()
            w.writerows(rows)
        idx.append(dict(DLC=name, k=k, dt_s=DT, n_raw=len(raw), n_bins=len(rows),
                        T_s=round((len(raw) - 1) * DT0, 1), file=f"{name}.csv"))
        print(f"  [{i:3d}/{len(kmap)}] {name:22} k={k:5.2f}  "
              f"{len(raw):5d}pt → {len(rows):3d}빈")
    with open(os.path.join(OUTDIR, "_index.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(idx[0]))
        w.writeheader()
        w.writerows(idx)
    tot = sum(r["n_bins"] for r in idx)
    print(f"\n[완료] {len(idx)}개 파일 · 총 {tot}빈 → {OUTDIR}")


if __name__ == "__main__":
    main()

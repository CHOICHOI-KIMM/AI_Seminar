"""
DLC별 해석 · 0단계 — 전 DLC 시계열 사전 파싱 + 기초통계 (1회 실행)
==================================================================
각 DLC 폴더에 raw.csv(시계열) + stats.csv(기초통계) 저장, 전체 메타를 dlc_meta.csv 로 집계.
파서·열정의는 검증된 §3/§4 (masta_fatigue.parse_dlc 와 동일 규칙, MASTA 불필요).
"""
import csv
import glob
import math
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

DLC_DIR = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중"
FHRS = os.path.join(DLC_DIR, "FatigueHours.txt")
COLS = ["t", "rpm", "Mx", "My", "Mz", "Fx", "Fy", "Fz"]


def parse_dlc(path):
    rows = []
    for ln in open(path, encoding="latin-1").readlines()[4:]:
        p = ln.split()
        if len(p) < 8:
            continue
        try:
            rows.append([float(x) for x in p[:8]])
        except ValueError:
            continue
    return np.array(rows)


def parse_sf(s):
    import re
    m = re.match(r"^([0-9]+\.[0-9]+)\.(E[+-]?[0-9]+)$", s.strip(), re.I)
    return float(m.group(1) + m.group(2)) if m else float(s)


def load_hours():
    out = {}
    for ln in open(FHRS, encoding="utf-8").readlines()[1:]:
        p = ln.rstrip("\n").split("\t")
        if len(p) >= 4:
            out[p[0]] = (parse_sf(p[1]), float(p[2]), float(p[3]))
    return out


def main():
    hours = load_hours()
    files = sorted(glob.glob(os.path.join(DLC_DIR, "*.$150")))
    print(f"[대상] {len(files)}개 DLC (FatigueHours {len(hours)}행)")
    meta = []
    for f in files:
        name = os.path.basename(f)[:-5]
        A = parse_dlc(f)
        d = os.path.join(HERE, name)
        os.makedirs(d, exist_ok=True)
        # raw.csv
        with open(os.path.join(d, "raw.csv"), "w", newline="", encoding="utf-8-sig") as fo:
            w = csv.writer(fo)
            w.writerow(COLS)
            w.writerows(A.tolist())
        # 통계
        n = len(A)
        dt0 = round(float(A[1, 0] - A[0, 0]), 6) if n > 1 else float("nan")
        rpm = A[:, 1]
        rpm_mean = float(rpm.mean())
        t3p = 20.0 / rpm_mean if rpm_mean > 0 else float("inf")
        dt_rule = 5.0 / rpm.max() if rpm.max() > 0 else float("inf")   # 최고 rpm 보수 적용
        low = int((rpm < 1.0).sum())
        st = {"DLC": name, "n_pts": n, "dt0_s": dt0,
              "rpm_mean": rpm_mean, "rpm_min": float(rpm.min()),
              "rpm_max": float(rpm.max()),
              "rpm_CV_pct": float(rpm.std() / rpm_mean * 100) if rpm_mean else float("nan"),
              "T3P_s": t3p, "dt_rule_max_s": dt_rule, "n_rpm_lt1": low,
              "ScaleFactor": hours.get(name, (float("nan"),) * 3)[0],
              "hours_30yr": hours.get(name, (float("nan"),) * 3)[1]}
        for i, c in enumerate(COLS[2:], start=2):    # Mx..Fz
            x = A[:, i]
            mu, sd = float(x.mean()), float(x.std())
            st[f"{c}_mean"] = mu
            st[f"{c}_std"] = sd
            st[f"{c}_CV_pct"] = abs(sd / mu * 100) if mu else float("inf")
        with open(os.path.join(d, "stats.csv"), "w", newline="", encoding="utf-8-sig") as fo:
            w = csv.writer(fo)
            w.writerow(list(st.keys()))
            w.writerow(list(st.values()))
        meta.append(st)
        if len(meta) % 20 == 0:
            print(f"  … {len(meta)}/{len(files)}")
    with open(os.path.join(HERE, "dlc_meta.csv"), "w", newline="", encoding="utf-8-sig") as fo:
        w = csv.DictWriter(fo, fieldnames=list(meta[0].keys()))
        w.writeheader()
        w.writerows(meta)
    print(f"[완료] {len(meta)}개 DLC — raw.csv/stats.csv + dlc_meta.csv")
    # 요약
    ms = [m for m in meta if not math.isnan(m["ScaleFactor"])]
    print(f"  ScaleFactor 매칭: {len(ms)}/{len(meta)}")
    lows = [m["DLC"] for m in meta if m["n_rpm_lt1"] > 0]
    print(f"  저속점(rpm<1) 포함 DLC: {len(lows)}개 {lows[:6]}")
    rng = [m["rpm_mean"] for m in meta]
    print(f"  rpm_mean 범위: {min(rng):.3f} ~ {max(rng):.3f}")


if __name__ == "__main__":
    main()

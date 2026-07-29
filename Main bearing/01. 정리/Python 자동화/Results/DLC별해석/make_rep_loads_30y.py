"""
대표하중 30년 기준 변환 (LRD 형태)
===================================
대표하중_원본좌표/ 의 DLC별 CSV 를 읽어, rev 열에 FatigueHours.xlsx 의
Detail Scale Factor(= 해당 시계열의 30년 반복 횟수)를 곱해 30년 누적
회전수로 바꾼 파일을 하위 폴더에 생성한다.

  · rev  ← rev × SF        (소수 3자리) · 열 이름은 rev_30y 로 명시
  · t_s  삭제              → 9열
  · 하중 6열 헤더에 단위 병기 (외부 전달용) : Fx [kN] … Mz [kNm]
  · 파일명 {DLC}_30y.csv
  · 하중값(Fx~Mz)·rpm 은 원본 그대로 (μ+kσ 대표하중)
"""
import csv
import os

import openpyxl

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "대표하중_원본좌표")
DST = os.path.join(SRC, "대표하중_30년기준")
XLSX = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중"
        r"\FatigueHours.xlsx")
# (원본 CSV 열 이름, 출력 헤더) — 하중 6열만 단위 병기
COLMAP = [("index", "index"), ("rpm", "rpm"), ("rev", "rev_30y"),
          ("Fx", "Fx [kN]"), ("Fy", "Fy [kN]"), ("Fz", "Fz [kN]"),
          ("Mx", "Mx [kNm]"), ("My", "My [kNm]"), ("Mz", "Mz [kNm]")]
COLS = [h for _, h in COLMAP]


def read_sf():
    ws = openpyxl.load_workbook(XLSX, data_only=True)["FatigueHours"]
    out = {}
    for r in ws.iter_rows(min_row=2, values_only=True):
        if r[0] and r[1] is not None:
            out[str(r[0]).strip()] = (float(r[1]), float(r[2]))   # SF, 30yr h
    return out


def main():
    os.makedirs(DST, exist_ok=True)
    sf = read_sf()
    names = sorted(f[:-4] for f in os.listdir(SRC)
                   if f.endswith(".csv") and not f.startswith("_"))
    missing = [n for n in names if n not in sf]
    if missing:
        raise SystemExit(f"[중단] SF 미확보 DLC {len(missing)}건: {missing[:5]}")

    idx, grand = [], 0.0
    for i, n in enumerate(names, 1):
        s, h30 = sf[n]
        with open(os.path.join(SRC, n + ".csv"), encoding="utf-8-sig") as f:
            rows = list(csv.DictReader(f))
        out, tot = [], 0.0
        for r in rows:
            rev30 = float(r["rev"]) * s
            tot += rev30
            out.append({h: (round(rev30, 3) if s_ == "rev" else r[s_])
                        for s_, h in COLMAP})
        with open(os.path.join(DST, f"{n}_30y.csv"), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=COLS)
            w.writeheader()
            w.writerows(out)
        grand += tot
        idx.append(dict(DLC=n, scale_factor=s, hours_30y=round(h30, 3),
                        n_bins=len(out), rev_30y_total=round(tot, 3),
                        file=f"{n}_30y.csv"))
        print(f"  [{i:3d}/{len(names)}] {n:22} SF={s:>8,.0f}  "
              f"{len(out):3d}빈  Σrev={tot:>14,.0f}")

    with open(os.path.join(DST, "_index.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(idx[0]))
        w.writeheader()
        w.writerows(idx)
    print(f"\n[완료] {len(idx)}개 파일 · 30년 총 회전수 {grand:,.0f} rev")
    print(f"       → {DST}")


if __name__ == "__main__":
    main()

"""
DLC별해석_총괄.xlsx (초안) — 전 DLC 핵심 결과 + 최적 (dt, k)
데이터: dlc_master_summary.csv + dlc_meta.csv (재실행 시 최신 반영)
"""
import csv
import os

from openpyxl import Workbook
from openpyxl.styles import Alignment, Border, Font, PatternFill, Side
from openpyxl.utils import get_column_letter

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "DLC별해석_총괄.xlsx")
HDR_FILL = PatternFill("solid", fgColor="D9E1F2")
WARN_FILL = PatternFill("solid", fgColor="FDEBD0")
BOLD = Font(bold=True)
TH = Side(style="thin", color="BFBFBF")
BD = Border(left=TH, right=TH, top=TH, bottom=TH)
BLUE, RED = Font(color="1F60C4"), Font(color="C00000")


def fnum(v):
    try:
        return float(v)
    except (TypeError, ValueError):
        return None


def main():
    master = {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_master_summary.csv"), encoding="utf-8-sig"))}
    meta = {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}

    wb = Workbook(); ws = wb.active; ws.title = "총괄"
    ws.append(["DLC별 해석 총괄 (초안) — 1단계 스크리닝 기준. "
               "수명·손상 절대값은 핀지지 해석 경로(절대편향 내포), 상대지표·최적조합 탐색용. "
               "2단계 MASTA에서 대체 예정"])
    ws.cell(1, 1).font = Font(bold=True, size=11, color="C00000")
    hdr = ["DLC", "ScaleFactor", "rpm 평균", "rpm CV%", "T_3P [s]", "dt 상한(5/rpm_max)",
           "M_X CV%", "저속점(rpm<1)", "κ클립 점수",
           "UW 30년손상(scr)", "DW 30년손상(scr)",
           "UW 수명(scr, yr)", "DW 수명(scr, yr)", "Sys 수명(scr, yr)",
           "최적 dt [s]", "최적 k", "ε_UW@best", "ε_DW@best", "ε_Sys@best", "비고"]
    ws.append(hdr)
    for c in range(1, len(hdr) + 1):
        cell = ws.cell(2, c); cell.font = BOLD; cell.fill = HDR_FILL
        cell.alignment = Alignment(horizontal="center", wrap_text=True); cell.border = BD

    r = 3
    for name in sorted(master):
        m, mt = master[name], meta.get(name, {})
        valid = m.get("valid") == "1"
        flags = []
        if int(float(m.get("n_rpm_lt1", 0) or 0)) > 0:
            flags.append("저속")
        if int(float(m.get("n_kappa_clip", 0) or 0)) > 0:
            flags.append("κ클립")
        if not valid:
            flags.append("참값손상≈0(스킵)")
        if valid and m.get("best_dt_s") in ("", None, "None"):
            flags.append("합격조합없음")
        vals = [name, fnum(m["ScaleFactor"]), fnum(m["rpm_mean"]), fnum(m["rpm_CV_pct"]),
                fnum(m["T3P_s"]), fnum(m["dt_rule_max_s"]),
                fnum(mt.get("Mz_CV_pct")), int(float(m.get("n_rpm_lt1", 0) or 0)),
                int(float(m.get("n_kappa_clip", 0) or 0)),
                fnum(m.get("D30_UW_scr")), fnum(m.get("D30_DW_scr")),
                fnum(m.get("life_UW_scr_yr")), fnum(m.get("life_DW_scr_yr")),
                fnum(m.get("life_Sys_scr_yr")),
                fnum(m.get("best_dt_s")), fnum(m.get("best_k")),
                fnum(m.get("eps_UW_at_best")), fnum(m.get("eps_DW_at_best")),
                fnum(m.get("eps_Sys_at_best")), ", ".join(flags)]
        for c, v in enumerate(vals, 1):
            cell = ws.cell(r, c, v if v is not None else "–")
            cell.border = BD
            if c in (17, 18, 19) and isinstance(v, float):
                cell.value = v / 100.0
                cell.number_format = "+0.00%;-0.00%"
                cell.font = BLUE if v < 0 else RED
        for c, fmt in ((2, "#,##0"), (3, "0.000"), (4, "0.0"), (5, "0.00"), (6, "0.00"),
                       (7, "0.0"), (10, "0.0000"), (11, "0.0000"),
                       (12, "#,##0.0"), (13, "#,##0.0"), (14, "#,##0.0"),
                       (15, "0.0"), (16, "0.00")):
            ws.cell(r, c).number_format = fmt
        if flags:
            ws.cell(r, 20).fill = WARN_FILL
        r += 1
    # ── 전 DLC 합산 (MASTA 기반 양식 — 2단계 배치 완료 후 데이터 입력) ──
    r += 1
    ws.cell(r, 1, "■ 전 DLC 합산 (MASTA 기반) — 2단계 배치 완료 후 입력")
    ws.cell(r, 1).font = Font(bold=True, size=11)
    ws.cell(r, 1).fill = PatternFill("solid", fgColor="EAECEE")
    part = None
    pp = os.path.join(HERE, "fleet_masta_partial.csv")
    if os.path.exists(pp):
        part = list(csv.DictReader(open(pp, encoding="utf-8-sig")))[0]
    lbl = (f"Σ D30 (상위 {part['n_dlc']} DLC 부분합, MASTA 참값)" if part
           else "Σ D30 (전 111 DLC, MASTA)")
    r += 1
    ws.cell(r, 1, lbl).font = BOLD
    for c, key in ((10, "sumD30_UW"), (11, "sumD30_DW")):
        cell = ws.cell(r, c, float(part[key]) if part else "–")
        cell.border = BD
        if part:
            cell.number_format = "0.0000"
    ws.cell(r, 1).border = BD
    r += 1
    ws.cell(r, 1, "수명 [yr] = 30/ΣD30 · Sys=와이블(e=9/8)"
            + (" — 부분합 기준" if part else "")).font = BOLD
    for c, key in ((12, "life_UW"), (13, "life_DW"), (14, "life_Sys")):
        cell = ws.cell(r, c, float(part[key]) if part else "–")
        cell.border = BD
        cell.fill = PatternFill("solid", fgColor="FFF2CC")
        if part:
            cell.number_format = "#,##0.00"
            cell.font = Font(bold=True, color="C00000")
    ws.cell(r, 1).border = BD

    ws.freeze_panes = "B3"
    ws.auto_filter.ref = f"A2:{get_column_letter(len(hdr))}{r-1}"
    for i, w in enumerate([15, 11, 9, 8, 9, 10, 8, 9, 9, 12, 12, 12, 12, 12,
                           9, 8, 10, 10, 10, 22], 1):
        ws.column_dimensions[get_column_letter(i)].width = w
    wb.save(OUT)
    print(f"[저장] {OUT}  ({r-3}개 DLC)")


if __name__ == "__main__":
    main()

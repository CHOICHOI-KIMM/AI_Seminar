"""
§10-11.1 실행 요약표 재생성 — ΔHV 를 **백분율**로
====================================================
`s3_genlog.csv` 만 읽어 §10-11.1 마커를 다시 쓴다. 본런이 도는 중에는
`a10_s3_run.py` 의 콜백이 10세대마다 덮어쓰므로, **종료 후에 부른다.**

절대 ΔHV 는 목적의 물리 단위(m · t · t)가 섞인 부피라 크기 자체에 의미가 없다.
**직전 표시 행 대비 백분율**로 보아야 수렴이 읽힌다.
"""
import csv
import io
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = "A10:GENLOG"
STEP = 10


def main(step=STEP):
    G = list(csv.DictReader(open(os.path.join(OUT, "s3_genlog.csv"),
                                 encoding="utf-8-sig")))
    show = [g for g in G if int(g["gen"]) % step == 0
            or int(g["gen"]) == int(G[-1]["gen"])]
    body = ["", "| 세대 | 프론트 | HV | **ΔHV** | **ΔHV / HV** | "
            "**최소 `D`** [mm] | **최소 베어링** [t] | **최소 총질량** [t] |",
            "|--:|--:|--:|--:|--:|--:|--:|--:|"]
    prev = None
    for g in show:
        hv = float(g["hv"])
        d = "—" if prev is None else f"{hv-prev:+,.1f}"
        pct = "—" if prev is None else f"**{100*(hv-prev)/prev:+.3f}%**"
        body.append(f"| {g['gen']} | {g['n_front']} | {hv:,.1f} | {d} | {pct} | "
                    f"**{float(g['f1_min'])*1e3:,.0f}** | "
                    f"**{float(g['f2_min']):.2f}** | "
                    f"**{float(g['f3_min']):.2f}** |")
        prev = hv
    body += ["", f"*{step}세대 간격 · `ΔHV` 는 **직전 표시 행** 대비 · HV 기준점 "
             "(5.5 m, 45 t, 250 t) · 셋 다 목적이다*"]

    s = io.open(DOC, encoding="utf-8").read()
    a, b = f"<!-- {MARK} -->", f"<!-- /{MARK} -->"
    pat = re.compile(re.escape(a) + r".*?" + re.escape(b), re.S)
    assert pat.search(s), MARK
    blk = a + "\n" + "\n".join(body) + "\n" + b
    out = pat.sub(lambda _m: blk, s, count=1)
    io.open(DOC, "w", encoding="utf-8").write(out)

    hv = [float(g["hv"]) for g in G]
    last = max((i for i in range(1, len(hv))
                if 100 * (hv[i] - hv[i - 1]) / hv[i - 1] > 0.05), default=None)
    print(f"[문서] §10-11.1 {len(show)}행 · 최종 HV {hv[-1]:,.2f}")
    print(f"  세대당 증분이 0.05% 를 넘은 마지막 세대: "
          f"{last+1 if last else '—'} / {len(hv)}")
    print(f"  마지막 10세대 누적 {100*(hv[-1]-hv[-11])/hv[-11]:+.3f}%")
    return len(show)


if __name__ == "__main__":
    sys.exit(0 if main(int(sys.argv[1]) if len(sys.argv) > 1 else STEP) else 1)

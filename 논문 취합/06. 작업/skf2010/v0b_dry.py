"""V0b — M1 건접촉 단위검사: 단일 정현파의 Westergaard 해석해 대조 (계획서 §6 V0b · Johnson).

간극 g(x) = Δ(1 − cos 2πx/λ) (y 방향 균일), 주기 패치 λ × λ.
  p* = π E* Δ / λ   (완전접촉에 필요한 평균압력)
  부분접촉 p̄ < p*:  sin²(πa/λ) = p̄/p*,
                    p(x) = 2p̄ cos(πx/λ) √(sin²(πa/λ) − sin²(πx/λ)) / sin²(πa/λ),  |x| < a
  완전접촉 p̄ ≥ p*:  p(x) = p̄ + p* cos(2πx/λ)
소성 검사: p_lim 을 탄성 최대압력의 60 % 로 두고 상한·평균하중 유지 확인.
완전접촉(p̄ ≥ p*)은 원문(Stanley–Kato §6) 적용범위 밖이라 판정하지 않고 비수렴 여부만 기록한다(사용자 결정 2026-10-09).
압력 대조는 해석 접촉경계 ±1 격자를 뺀 표본점으로 판정한다(경계는 접촉반폭 항목이 판정 · 사용자 결정 2026-10-09).
표본 밀도 시험: 파장당 128 · 256 · 512 점에서 최대 오차가 경계점(√ 특이)에서 √Δx 로 줄어드는지 기록한다.
"""
import numpy as np

import dry_contact as DC

E_EFF, DELTA, LAM, NX, NY = 219e9, 1.0e-6, 100e-6, 128, 8
TOL_P = 0.02          # 표본점 압력 오차 / p* — Stanley–Kato Fig 1 수준(16점/파장에서 '매우 좋음')
RESULTS = []


def rec(tid, label, obs, ok):
    RESULTS.append((tid, label, obs, ok))


def westergaard(x, pbar, pstar):
    if pbar >= pstar:
        return pbar + pstar * np.cos(2 * np.pi * x / LAM)
    s2a = pbar / pstar
    s2 = np.sin(np.pi * x / LAM) ** 2
    out = np.zeros_like(x)
    m = s2 < s2a
    out[m] = 2 * pbar * np.cos(np.pi * x[m] / LAM) * np.sqrt(s2a - s2[m]) / s2a
    return out


def main():
    x = (np.arange(NX) - NX // 2) * LAM / NX          # x = 0 이 봉우리
    g = np.tile(DELTA * (1 - np.cos(2 * np.pi * x / LAM)), (NY, 1))
    pstar = np.pi * 0.5 * E_EFF * DELTA / LAM
    print(f"p* = {pstar/1e9:.3f} GPa · 파장당 {NX} 점")
    for ratio in (0.1, 0.3, 0.5, 0.8):
        pbar = ratio * pstar
        r = DC.solve(g, LAM, LAM * NY / NX, E_EFF, pbar)
        p = r["p"][0]
        pa = westergaard(x, pbar, pstar)
        err_all = np.max(np.abs(p - pa)) / pstar
        a_an = LAM / np.pi * np.arcsin(np.sqrt(ratio))
        inner = np.abs(np.abs(x) - a_an) > LAM / NX        # 접촉경계 ±1 격자 제외(사용자 결정 2026-10-09)
        err = np.max(np.abs(p - pa)[inner]) / pstar
        load = abs(np.mean(r["p"]) / pbar - 1)
        a_num = 0.5 * np.sum(p > 0) * LAM / NX
        wtxt = f"반폭 수치 {a_num*1e6:.2f} / 해석 {a_an*1e6:.2f} μm (격자 {LAM/NX*1e6:.2f})"
        wok = abs(a_num - a_an) <= LAM / NX
        rec("V0b", f"p̄/p* = {ratio}: 압력 대조", f"경계±1격자 제외 최대오차 {err:.4f} p* (전체 {err_all:.4f}) · 반복 {r['iters']} · 수렴 {r['converged']}",
            err <= TOL_P and r["converged"])
        rec("V0b", f"p̄/p* = {ratio}: 하중·접촉폭", f"하중오차 {load:.1e} · {wtxt}", load < 1e-9 and wok)
    # 완전접촉 — 적용범위 밖(판정 없음)
    r = DC.solve(g, LAM, LAM * NY / NX, E_EFF, 1.2 * pstar, itmax=2000)
    print(f"[적용범위 밖] p̄/p* = 1.2 완전접촉: 수렴 {r['converged']} · full_contact {r['full_contact']} · "
          f"p 범위 {r['p'].min()/1e9:.3f}–{r['p'].max()/1e9:.3f} GPa (해석 {(1.2-1)*pstar/1e9:.3f}–{2.2*pstar/1e9:.3f})")
    # 표본 밀도 시험(기록)
    for ratio in (0.5,):
        errs = []
        for nx in (128, 256, 512):
            xx = (np.arange(nx) - nx // 2) * LAM / nx
            gg = np.tile(DELTA * (1 - np.cos(2 * np.pi * xx / LAM)), (4, 1))
            rr = DC.solve(gg, LAM, LAM * 4 / nx, E_EFF, ratio * pstar)
            errs.append(np.max(np.abs(rr["p"][0] - westergaard(xx, ratio * pstar, pstar))) / pstar)
        print(f"[표본 밀도] p̄/p* = {ratio}: 최대오차 " + " · ".join(f"N{n} {e:.4f}" for n, e in zip((128, 256, 512), errs))
              + f" · 비 {errs[0]/errs[1]:.2f} · {errs[1]/errs[2]:.2f} (√2 = 1.41)")
    # 소성 상한
    pbar = 0.3 * pstar
    pmax_el = westergaard(np.array([0.0]), pbar, pstar)[0]
    plim = 0.6 * pmax_el
    r = DC.solve(g, LAM, LAM * NY / NX, E_EFF, pbar, p_lim=plim)
    rec("V0b", "완전소성: 상한·평균하중", f"max p/p_lim {r['p'].max()/plim:.6f} · 하중오차 "
        f"{abs(np.mean(r['p'])/pbar-1):.1e} · 소성점 {r['plastic']:.1%} · 반복 {r['iters']} · 수렴 {r['converged']}",
        r["p"].max() <= plim * (1 + 1e-12) and abs(np.mean(r["p"]) / pbar - 1) < 1e-9 and r["converged"])
    print(f"{'ID':<6}{'항목':<30}{'관측':<70}판정")
    for t, l, o, ok in RESULTS:
        print(f"{t:<6}{l:<30}{o:<70}{'PASS' if ok else 'FAIL'}")
    n = sum(ok for *_, ok in RESULTS)
    print(f"{n}/{len(RESULTS)} PASS")
    return n == len(RESULTS)


if __name__ == "__main__":
    raise SystemExit(0 if main() else 1)

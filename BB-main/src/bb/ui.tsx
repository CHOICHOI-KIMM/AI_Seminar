// BB 공용 UI (Plan Phase 4-4 P4-4c · 글로벌 CLAUDE.md 「UI 구성: 공용 컴포넌트 우선」)
//
// BB 뷰마다 인라인으로 반복되던 안내·경고·오류 박스, 카드, 알림 목록, 로딩 화면을 모았다.
// BB 뷰는 공용 UI 와 `DetailTable` 을 **이 파일에서만** import 한다(단일 진입점).
//
// ⚠ 이 파일은 **컴포넌트만** export 한다 — 상수·로직은 `shared.ts` 몫이다
//    (`react-refresh/only-export-components`).
// ⚠ 실제로 쓰는 컴포넌트만 둔다. 쓰는 곳 없는 컴포넌트는 죽은 코드다.

import type { ReactNode } from 'react';
import type { Alert } from './generated/Alert';
import type { AlertLevel } from './generated/AlertLevel';

export { DetailTable } from '../components/shared/DetailTable';

/** 「프리셋을 불러오는 중…」 — 입력이 아직 없을 때의 전체 화면. */
export function Loading() {
  return (
    <div className="flex items-center justify-center h-full">
      <p className="text-text-canvas text-sm">프리셋을 불러오는 중…</p>
    </div>
  );
}

/** 안내 박스 (파랑). */
export function InfoBox({ children }: { children: ReactNode }) {
  return (
    <div className="p-2.5 rounded border bg-blue-500/10 border-blue-400/30 text-blue-100 text-[12px] leading-relaxed">
      {children}
    </div>
  );
}

/** 경고 박스 (호박색). */
export function WarnBox({ children }: { children: ReactNode }) {
  return (
    <div className="p-2 rounded border bg-amber-500/10 border-amber-400/30 text-amber-200 text-[12px]">
      {children}
    </div>
  );
}

/** 오류 박스 (빨강) — 제목 + Rust 메시지 원문. */
export function ErrorBox({ title, message }: { title: string; message: string }) {
  return (
    <div className="p-2 rounded border bg-red-500/10 border-red-400/30 text-red-200">
      <p className="text-[13px] font-medium">{title}</p>
      <p className="text-xs opacity-80 whitespace-pre-wrap break-words">{message}</p>
    </div>
  );
}

/** 카드(패널). `className` 은 기본 클래스 뒤에 덧붙인다 (예: `space-y-3`). */
export function Panel({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={`rounded border border-white/10 bg-black/20 p-3${className ? ` ${className}` : ''}`}>
      {children}
    </div>
  );
}

// 아이콘은 `AlertPanel` 과 같은 `i` · `!` · `!!` (기존 패턴 우선).
const ALERT_COLORS: Record<AlertLevel, string> = {
  Info: 'bg-blue-500/10 border-blue-400/30 text-blue-200',
  Warning: 'bg-amber-500/10 border-amber-400/30 text-amber-200',
  Critical: 'bg-red-500/10 border-red-400/30 text-red-200',
};
const ALERT_ICONS: Record<AlertLevel, string> = { Info: 'i', Warning: '!', Critical: '!!' };

/** 알림 목록 — 제목 + 알림들. 비어 있으면 「경고 없음」. */
export function AlertList({ title, alerts }: { title: string; alerts: Alert[] }) {
  return (
    <div>
      <h4 className="text-sm font-medium text-text-light mb-2">{title}</h4>
      {alerts.length === 0 ? (
        <p className="text-[13px] text-text-canvas/60">경고 없음</p>
      ) : (
        alerts.map((a, i) => (
          <div key={i} className={`flex items-start gap-2 p-2 rounded border mb-1 ${ALERT_COLORS[a.level]}`}>
            <span className="text-[13px] font-medium mt-0.5">{ALERT_ICONS[a.level]}</span>
            <div className="min-w-0">
              <p className="text-[13px] font-medium">{a.code}</p>
              <p className="text-xs opacity-80">{a.message}</p>
            </div>
          </div>
        ))
      )}
    </div>
  );
}

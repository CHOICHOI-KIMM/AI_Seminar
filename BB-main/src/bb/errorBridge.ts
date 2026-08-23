// BB — 웹뷰 오류 → Rust 로그 브리지 (Plan §3.6.5.3)
//
// `npm run tauri dev` 터미널에는 **Rust 패닉과 Vite 빌드 오류만** 나온다.
// 웹뷰 안의 JS 런타임 오류(`undefined.xxx` 같은 것)는 보이지 않는다.
// 그것이 보이지 않으면 검증 사다리 ④(런타임 헬스체크)가 성립하지 않는다.
//
// Rust 쪽 `tauri-plugin-log` 는 이미 `lib.rs` 에서 초기화 중이고
// 기본 target 에 Stdout 이 들어 있다 — 즉 여기서 넘긴 것이 dev 터미널에 찍힌다.

import { error as logError, warn as logWarn } from '@tauri-apps/plugin-log';

let installed = false;
/** 브리지 자신이 낸 오류를 다시 브리지에 넣지 않기 위한 재진입 가드 */
let forwarding = false;

function forward(kind: 'error' | 'warn', text: string): void {
  if (forwarding) return;
  forwarding = true;
  const emit = kind === 'error' ? logError : logWarn;
  // 로그 전송 자체가 실패해도(권한 미부여 등) 앱을 죽이지 않는다
  void emit(`[webview] ${text}`)
    .catch(() => {})
    .finally(() => {
      forwarding = false;
    });
}

function stringify(value: unknown): string {
  if (value instanceof Error) return `${value.name}: ${value.message}\n${value.stack ?? ''}`;
  if (typeof value === 'string') return value;
  try {
    return JSON.stringify(value);
  } catch {
    return String(value);
  }
}

/**
 * `window.onerror` · `unhandledrejection` · `console.error` 를 Rust 로그로 넘긴다.
 * `main.tsx` 에서 **1회만** 호출한다 (StrictMode 이중 실행에 대비해 멱등).
 */
export function installErrorBridge(): void {
  if (installed) return;
  installed = true;

  window.addEventListener('error', (ev: ErrorEvent) => {
    forward('error', `window.onerror ${ev.filename}:${ev.lineno}:${ev.colno} ${stringify(ev.error ?? ev.message)}`);
  });

  window.addEventListener('unhandledrejection', (ev: PromiseRejectionEvent) => {
    forward('error', `unhandledrejection ${stringify(ev.reason)}`);
  });

  const originalError = console.error.bind(console);
  console.error = (...args: unknown[]) => {
    originalError(...args);
    forward('error', `console.error ${args.map(stringify).join(' ')}`);
  };

  const originalWarn = console.warn.bind(console);
  console.warn = (...args: unknown[]) => {
    originalWarn(...args);
    forward('warn', `console.warn ${args.map(stringify).join(' ')}`);
  };
}

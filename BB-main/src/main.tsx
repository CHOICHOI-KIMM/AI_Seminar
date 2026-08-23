import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
// P4-S1-5 (Plan §3.6.5.3): 웹뷰 오류를 Rust 로그로 넘기고,
// VITE_BB_HEALTHCHECK=1 일 때만 자동 스모크를 1회 돌린다.
import { installErrorBridge } from './bb/errorBridge'
import { runHealthcheckIfEnabled } from './bb/healthcheck'

installErrorBridge()
runHealthcheckIfEnabled()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)

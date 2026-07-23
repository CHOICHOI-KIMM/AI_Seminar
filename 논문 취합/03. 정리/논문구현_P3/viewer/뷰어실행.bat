@echo off
rem Micropitting P3 뷰어 실행 — 로컬 서버 + 브라우저 (file:// 불가 → http 필수)
cd /d "%~dp0"
start "p3-viewer-server" /min python -m http.server 8741 --bind 127.0.0.1
timeout /t 1 /nobreak >nul
start "" http://127.0.0.1:8741/index.html

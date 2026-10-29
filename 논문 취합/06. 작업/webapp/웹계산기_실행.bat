@echo off
rem SKF 2010 M2 web calculator - double-click to start server and open browser (close this window to stop)
cd /d "%~dp0"
python server.py --open
if errorlevel 1 pause

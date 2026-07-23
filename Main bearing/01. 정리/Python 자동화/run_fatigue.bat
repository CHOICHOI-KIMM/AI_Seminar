@echo off
rem ---------------------------------------------------------------
rem  ASCII-only batch. Korean messages are printed by Python (UTF-8)
rem  to avoid cmd.exe multibyte parsing issues.
rem ---------------------------------------------------------------
chcp 65001 >nul
cd /d "%~dp0"
set PYTHONUTF8=1
set PYTHONIOENCODING=utf-8

echo ==============================================================
echo   MASTA Bearing Fatigue Batch   (config: fatigue_config.xlsx)
echo ==============================================================

if not exist "fatigue_config.xlsx" (
  echo   [INFO] fatigue_config.xlsx not found - running with defaults.
  echo          To create it:  python make_config_xlsx.py
  echo --------------------------------------------------------------
)

python -X utf8 masta_fatigue.py
if errorlevel 1 goto err

echo --------------------------------------------------------------
echo   DONE. Check the generated CSV / xlsx files.
goto end

:err
echo --------------------------------------------------------------
echo   [ERROR] Run failed. See messages above.
echo     - Are MODEL_PATH / DLC_FILE correct in fatigue_config.xlsx ?
echo     - Is the output CSV/xlsx still open in Excel ?
echo     - Is the model still open in MASTA GUI ?
echo     - Is Python on PATH ?  (try:  python --version)

:end
echo.
pause

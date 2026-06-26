#requires -Version 5.1
# TRB-main 개발 서버 기동 헬퍼
# - VS 2022 BuildTools의 vcvars64.bat 환경을 현재 PowerShell 세션에 import
# - 이후 npm run tauri dev 실행
# 사용: ./dev.ps1

$ErrorActionPreference = "Stop"

$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if (-not (Test-Path $vcvars)) {
    throw "vcvars64.bat not found at: $vcvars"
}

Write-Host "Loading VS 2022 BuildTools environment..." -ForegroundColor Cyan
$tmp = New-TemporaryFile
cmd /c "`"$vcvars`" >nul 2>&1 && set" | Out-File -FilePath $tmp.FullName -Encoding ASCII
Get-Content $tmp.FullName | ForEach-Object {
    if ($_ -match "^([^=]+)=(.*)$") {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}
Remove-Item $tmp.FullName -Force

Write-Host "  VSINSTALLDIR = $env:VSINSTALLDIR" -ForegroundColor DarkGray
Write-Host "  MSVC linker  = $(Get-Command link.exe -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source)" -ForegroundColor DarkGray
Write-Host ""

Write-Host "Starting Tauri dev server..." -ForegroundColor Cyan
npm run tauri dev

@echo off
setlocal EnableExtensions
REM Adds this directory to your user PATH (for a manual release extract).
REM Run from inside the extracted "lyangpiler" folder (where lyangpiler.exe lives).

set "INSTALL_DIR=%~dp0"
if "%INSTALL_DIR:~-1%"=="\" set "INSTALL_DIR=%INSTALL_DIR:~0,-1%"

echo Adding Lyangpiler to user PATH:
echo   %INSTALL_DIR%
echo.

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$d = $env:INSTALL_DIR; $p = [Environment]::GetEnvironmentVariable('PATH','User'); if (-not $p) { $p = '' }; if ($p -notlike ('*' + $d + '*')) { [Environment]::SetEnvironmentVariable('PATH', ($p.TrimEnd(';') + ';' + $d), 'User'); Write-Host 'Done. Open a new terminal.' -ForegroundColor Green } else { Write-Host 'Already on user PATH.' -ForegroundColor Green }"

echo.
echo Then run: lyangpiler --help
pause

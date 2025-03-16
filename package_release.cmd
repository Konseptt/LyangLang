@echo off
REM Create a distributable package of Lyangpiler
REM This script creates a zip file with everything needed to run Lyangpiler on another PC

echo Creating Lyangpiler release package...

REM Set directory variables
set SCRIPT_DIR=%~dp0
set RELEASE_DIR=%SCRIPT_DIR%release
set PACKAGE_DIR=%SCRIPT_DIR%lyangpiler-package
set VERSION=0.1.0

REM First, build the release version if it doesn't exist
if not exist "%RELEASE_DIR%\lyangpiler.exe" (
    echo Building release version...
    cargo build --release
    if %ERRORLEVEL% NEQ 0 (
        echo Failed to build release version!
        exit /b 1
    )
)

REM Create and clean package directory
if exist "%PACKAGE_DIR%" rmdir /s /q "%PACKAGE_DIR%"
mkdir "%PACKAGE_DIR%"

REM Copy executable and required files
echo Copying files to package...
copy "%RELEASE_DIR%\lyangpiler.exe" "%PACKAGE_DIR%\"
copy "%SCRIPT_DIR%\lyangpiler.cmd" "%PACKAGE_DIR%\"
copy "%SCRIPT_DIR%\example.nbh" "%PACKAGE_DIR%\"
copy "%SCRIPT_DIR%\README.md" "%PACKAGE_DIR%\"

REM Create a simplified launcher that doesn't need cargo
echo Creating standalone launcher...
(
echo @echo off
echo REM Lyangpiler VM standalone launcher
echo REM Usage: lyangpiler.cmd ^<filename.nbh^>
echo.
echo setlocal
echo.
echo set SCRIPT_DIR=%%~dp0
echo.
echo REM Check if required parameter is provided
echo if "%%1"=="" (
echo     echo Usage: lyangpiler.cmd ^<filename.nbh^>
echo     exit /b 1
echo ^)
echo.
echo REM Run the executable with VM mode
echo "%%SCRIPT_DIR%%lyangpiler.exe" "%%1" --vm
) > "%PACKAGE_DIR%\lyangpiler.cmd"

REM Create a simple install script to add to PATH (optional)
echo Creating Windows installer...
(
echo @echo off
echo REM Simple installer for Lyangpiler
echo REM This adds the current directory to PATH
echo.
echo echo Installing Lyangpiler...
echo.
echo REM Get the current directory
echo set INSTALL_DIR=%%~dp0
echo set INSTALL_DIR=%%INSTALL_DIR:~0,-1%%
echo.
echo REM Add to PATH using PowerShell
echo powershell -Command "[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ';%%INSTALL_DIR%%', 'User')"
echo.
echo echo Lyangpiler has been installed!
echo echo You can now run 'lyangpiler example.nbh' from any directory.
echo echo.
echo echo Press any key to exit...
echo pause ^>nul
) > "%PACKAGE_DIR%\install.cmd"

REM Create Unix version of the launcher
echo Creating Unix launcher...
(
echo #!/bin/bash
echo # Lyangpiler VM standalone launcher for Unix-like systems
echo # Usage: ./lyangpiler ^<filename.nbh^>
echo.
echo SCRIPT_DIR="$^(cd "$(dirname "$0")" && pwd)"
echo.
echo # Check if required parameter is provided
echo if [ -z "$1" ]; then
echo     echo "Usage: ./lyangpiler <filename.nbh>"
echo     exit 1
echo fi
echo.
echo # Check if the executable exists and has correct permissions
echo if [ ! -x "$SCRIPT_DIR/lyangpiler" ]; then
echo     chmod +x "$SCRIPT_DIR/lyangpiler"
echo fi
echo.
echo # Run the executable with VM mode
echo "$SCRIPT_DIR/lyangpiler" "$1" --vm
) > "%PACKAGE_DIR%\lyangpiler"

REM Create a zip file of the package
echo Creating ZIP archive...
cd "%SCRIPT_DIR%"
powershell -Command "Compress-Archive -Path '%PACKAGE_DIR%\*' -DestinationPath 'lyangpiler-v%VERSION%.zip' -Force"

echo.
echo Release package created: lyangpiler-v%VERSION%.zip
echo.
echo This package can be extracted on any Windows PC and run without needing Rust installed.
echo To install, extract the ZIP and run install.cmd to add to PATH (optional).
echo.
echo Press any key to exit...
pause >nul
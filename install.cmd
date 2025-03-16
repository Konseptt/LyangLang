@echo off
REM LyangLang Installer for Windows
REM This script installs LyangLang and makes it available in PATH

setlocal enabledelayedexpansion

set VERSION=0.1.0
set INSTALL_DIR=%USERPROFILE%\.lyangpiler

REM Colors for Windows console output
set RED=[91m
set GREEN=[92m
set YELLOW=[93m
set BLUE=[94m
set BOLD=[1m
set NC=[0m

echo %BLUE%%BOLD%
echo   _                           _                   
echo  ^| ^|                         ^| ^|                  
echo  ^| ^|    _   _  __ _ _ __   __^| ^|_ __   __ _ _ __  
echo  ^| ^|   ^| ^| ^| ^|/ _` ^| '_ \ / _` ^| '_ \ / _` ^| '_ \ 
echo  ^| ^|___^| ^|_^| ^| (_^| ^| ^| ^| ^| (_^| ^| ^|_) ^| (_^| ^| ^| ^| ^|
echo  ^|______\__, ^|\__,_^|_^| ^|_^|\__,_^| .__/ \__,_^|_^| ^|_^|
echo          __/ ^|                 ^| ^|     __/ ^|      
echo         ^|___/                  ^|_^|    ^|___/       
echo %NC%
echo %YELLOW%LyangLang Programming Language Installer v%VERSION%%NC%
echo.

REM Check if Rust is installed
where rustc >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo %YELLOW%Rust is not installed. It's required to build LyangLang.%NC%
    echo Would you like to install Rust now? (y/n)
    set /p install_rust=
    
    if /i "!install_rust!"=="y" (
        echo %BLUE%Installing Rust...%NC%
        echo Downloading rustup-init.exe...
        powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe"
        rustup-init.exe -y
        del rustup-init.exe
        
        REM Refresh the PATH to include Rust binaries
        set PATH=%PATH%;%USERPROFILE%\.cargo\bin
    ) else (
        echo %RED%Rust is required to build LyangLang. Installation aborted.%NC%
        exit /b 1
    )
)

REM Create installation directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo %BLUE%Building LyangLang...%NC%

REM In a real scenario, we would download from a remote repository
REM For now, we'll build from the current directory
if exist ".git" (
    echo %GREEN%Building from current directory...%NC%
    cargo build --release
    
    REM Copy binaries and scripts
    copy /Y "target\release\lyangpiler.exe" "%INSTALL_DIR%\" >nul
    copy /Y "oimugchal.cmd" "%INSTALL_DIR%\" >nul
) else (
    echo %RED%Error: Not a git repository. Please run this script from the LyangLang repository root.%NC%
    exit /b 1
)

REM Create a launcher script in the installation directory
echo @echo off> "%INSTALL_DIR%\lyangpiler-launcher.cmd"
echo REM LyangLang launcher>> "%INSTALL_DIR%\lyangpiler-launcher.cmd"
echo cd /d "%INSTALL_DIR%">> "%INSTALL_DIR%\lyangpiler-launcher.cmd"
echo oimugchal.cmd %%*>> "%INSTALL_DIR%\lyangpiler-launcher.cmd"

REM Add to PATH
echo %YELLOW%Adding LyangLang to your PATH...%NC%
powershell -Command "[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ';%INSTALL_DIR%', 'User')"

REM Create desktop shortcuts
echo %BLUE%Creating shortcuts...%NC%
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\LyangLang.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\lyangpiler-launcher.cmd'; $Shortcut.Save()"

REM Create Start Menu entry
set START_MENU=%APPDATA%\Microsoft\Windows\Start Menu\Programs\LyangLang
if not exist "%START_MENU%" mkdir "%START_MENU%"
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%START_MENU%\LyangLang.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\lyangpiler-launcher.cmd'; $Shortcut.Save()"

echo.
echo %GREEN%%BOLD%LyangLang has been successfully installed!%NC%
echo.
echo You can now use %BOLD%oimugchal%NC% from any location to create and run LyangLang programs.
echo.
echo NOTE: You may need to restart your command prompt or terminal for PATH changes to take effect.
echo.
echo Try it out:
echo   %BOLD%oimugchal new my-project%NC%
echo   %BOLD%cd my-project%NC%
echo   %BOLD%oimugchal run%NC%
echo.
echo For more information, run: %BOLD%oimugchal help%NC%
echo.
echo Press any key to exit...
pause >nul
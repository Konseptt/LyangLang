@echo off
REM Lyangpiler VM launcher for Windows
REM Usage: lyangpiler.cmd <filename.nbh>

setlocal

set SCRIPT_DIR=%~dp0
set EXEC_PATH=%SCRIPT_DIR%target\release\lyangpiler.exe

REM Check if required parameter is provided
if "%1"=="" (
    echo Usage: lyangpiler.cmd ^<filename.nbh^>
    exit /b 1
)

REM Check if compiled executable exists
if exist "%EXEC_PATH%" (
    REM Use the compiled executable
    "%EXEC_PATH%" "%1" --vm
) else (
    REM Executable not found, use cargo run
    echo Notice: Compiled executable not found, using cargo run instead.
    echo You can build the executable with: cargo build --release
    cargo run -- "%1" --vm
)
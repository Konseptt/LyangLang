@echo off
setlocal EnableDelayedExpansion

REM Add the current directory to PATH temporarily
set "PATH=%PATH%;%~dp0"

if "%~1"=="" (
    cargo run
) else (
    REM Use proper path handling with %~1
    set "input_file=%~f1"
    if not exist "!input_file!" (
        echo Error: File "!input_file!" not found
        exit /b 1
    )
    copy /Y "!input_file!" "example.nbh" >nul
    cargo run
)

endlocal
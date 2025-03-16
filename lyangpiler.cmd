@echo off
REM Lyangpiler VM launcher for Windows
REM Usage: lyangpiler.cmd <filename.nbh>

if "%1"=="" (
    echo Usage: lyangpiler.cmd ^<filename.nbh^>
    exit /b 1
)

cargo run -- "%1" --vm
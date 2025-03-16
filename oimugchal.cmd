@echo off
REM oimugchal - LyangLang package manager for Windows
REM Usage: oimugchal [command] [options]

setlocal enabledelayedexpansion

set VERSION=0.1.0
set LYANGPILER_DIR=%~dp0

REM Define colors for Windows console
set RED=[91m
set GREEN=[92m
set YELLOW=[93m
set BLUE=[94m
set BOLD=[1m
set NC=[0m

REM Print banner
:print_banner
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
echo %YELLOW%Lyangpiler Package Manager v%VERSION%%NC%
echo.
exit /b

REM Print help message
:print_help
echo %BOLD%Usage:%NC% oimugchal [command] [options]
echo.
echo %BOLD%Commands:%NC%
echo   %GREEN%new%NC% ^<project-name^>        Create a new LyangLang project
echo   %GREEN%build%NC%                     Build the project
echo   %GREEN%run%NC% [file.nbh]            Run a LyangLang file (default: example.nbh)
echo   %GREEN%vm%NC% [file.nbh]             Run with the Lyangpiler VM (default: example.nbh)
echo   %GREEN%release%NC%                   Build for release
echo   %GREEN%install%NC%                   Install the compiled binary
echo   %GREEN%clean%NC%                     Clean build artifacts
echo   %GREEN%test%NC%                      Run tests
echo   %GREEN%version%NC%                   Show version information
echo   %GREEN%help%NC%                      Show this help message
echo.
echo %BOLD%Examples:%NC%
echo   oimugchal new hello-world
echo   oimugchal run example.nbh
echo   oimugchal vm example.nbh
echo.
exit /b

REM Create a new project
:create_new_project
if "%~1"=="" (
    echo %RED%Error: Project name required%NC%
    echo Usage: oimugchal new ^<project-name^>
    exit /b 1
)

set PROJECT_NAME=%~1

if exist "%PROJECT_NAME%" (
    echo %RED%Error: Directory '%PROJECT_NAME%' already exists%NC%
    exit /b 1
)

echo %BLUE%Creating new LyangLang project: %BOLD%%PROJECT_NAME%%NC%

mkdir "%PROJECT_NAME%"
cd "%PROJECT_NAME%"

REM Create project structure
mkdir src

REM Create main file
echo // Main file for %PROJECT_NAME% project> src\main.nbh
echo.>> src\main.nbh
echo // Print a welcome message>> src\main.nbh
echo bol mug "Namaste, world!">> src\main.nbh
echo.>> src\main.nbh
echo // Get user input>> src\main.nbh
echo bol mug "Timro naam k ho?">> src\main.nbh
echo oi mug bhan userName>> src\main.nbh
echo.>> src\main.nbh
echo // Greet the user>> src\main.nbh
echo bol mug "Namaste, " + userName + "!">> src\main.nbh

REM Create project configuration file
echo [project]> lyangfile.toml
echo name = "%PROJECT_NAME%">> lyangfile.toml
echo version = "0.1.0">> lyangfile.toml
echo author = "">> lyangfile.toml
echo.>> lyangfile.toml
echo [dependencies]>> lyangfile.toml
echo # List your dependencies here>> lyangfile.toml
echo # example = "1.0.0">> lyangfile.toml
echo.>> lyangfile.toml
echo [build]>> lyangfile.toml
echo main = "src/main.nbh">> lyangfile.toml

echo %GREEN%Project created successfully!%NC%
echo.
echo Get started with:
echo   %BOLD%cd %PROJECT_NAME%%NC%
echo   %BOLD%oimugchal run%NC%
echo.
exit /b 0

REM Main command processing
if "%~1"=="" (
    call :print_banner
    call :print_help
    exit /b 0
)

set COMMAND=%~1
shift

if "%COMMAND%"=="new" (
    call :create_new_project %1
) else if "%COMMAND%"=="build" (
    cd "%LYANGPILER_DIR%" && cargo build
) else if "%COMMAND%"=="run" (
    if "%~1"=="" (
        set FILE=example.nbh
    ) else (
        set FILE=%~1
    )
    cd "%LYANGPILER_DIR%" && cargo run -- "!FILE!"
) else if "%COMMAND%"=="vm" (
    if "%~1"=="" (
        set FILE=example.nbh
    ) else (
        set FILE=%~1
    )
    cd "%LYANGPILER_DIR%" && cargo run -- "!FILE!" --vm
) else if "%COMMAND%"=="release" (
    cd "%LYANGPILER_DIR%" && cargo build --release
) else if "%COMMAND%"=="install" (
    cd "%LYANGPILER_DIR%" && cargo install --path .
) else if "%COMMAND%"=="clean" (
    cd "%LYANGPILER_DIR%" && cargo clean
) else if "%COMMAND%"=="test" (
    cd "%LYANGPILER_DIR%" && cargo test
) else if "%COMMAND%"=="version" (
    call :print_banner
    echo %GREEN%LyangLang Version: %VERSION%%NC%
) else if "%COMMAND%"=="help" (
    call :print_banner
    call :print_help
) else (
    echo %RED%Unknown command: %COMMAND%%NC%
    call :print_help
    exit /b 1
)

exit /b 0
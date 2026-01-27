@echo off
echo ================================================
echo   Building TODO AI for Windows
echo ================================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs
    pause
    exit /b 1
)

echo [1/2] Building debug version...
cargo build
if %errorlevel% neq 0 (
    echo ERROR: Debug build failed
    pause
    exit /b 1
)

echo.
echo [2/2] Building release version (optimized)...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Release build failed
    pause
    exit /b 1
)

echo.
echo ================================================
echo   Build Complete!
echo ================================================
echo.
echo Binaries location:
echo   Debug:   %CD%\target\debug\todo_tui.exe
echo   Release: %CD%\target\release\todo_tui.exe
echo.
echo To run:
echo   Debug:   target\debug\todo_tui.exe
echo   Release: target\release\todo_tui.exe
echo.
echo To install system-wide, run: install.bat
echo.
pause
@echo off
setlocal enabledelayedexpansion

echo ================================================
echo   Installing TODO AI for Windows
echo ================================================
echo.

REM Check for admin rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo WARNING: Not running as Administrator
    echo Installation will be to user directory
    echo.
    set "INSTALL_DIR=%USERPROFILE%\AppData\Local\Programs\TodoAI"
) else (
    set "INSTALL_DIR=%ProgramFiles%\TodoAI"
)

REM Build release version
echo [1/2] Building release version...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Build failed
    pause
    exit /b 1
)

REM Create install directory
echo [2/2] Installing to %INSTALL_DIR%...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy binary
copy /Y target\release\todo_tui.exe "%INSTALL_DIR%\todo-ai.exe"
if %errorlevel% neq 0 (
    echo ERROR: Failed to copy binary
    pause
    exit /b 1
)

echo.
echo ================================================
echo   Installation Complete!
echo ================================================
echo.
echo Installed to: %INSTALL_DIR%\todo-ai.exe
echo.
echo To run from anywhere:
echo   1. Search "Environment Variables" in Windows
echo   2. Edit "Path" variable
echo   3. Add: %INSTALL_DIR%
echo   4. Open new Command Prompt
echo   5. Run: todo-ai
echo.
echo Or run directly: "%INSTALL_DIR%\todo-ai.exe"
echo.
pause
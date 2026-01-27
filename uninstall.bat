@echo off
echo ================================================
echo   Uninstalling TODO AI
echo ================================================
echo.

set "FOUND=0"

REM Check user directory
if exist "%USERPROFILE%\AppData\Local\Programs\TodoAI\todo-ai.exe" (
    echo Removing from user directory...
    rmdir /S /Q "%USERPROFILE%\AppData\Local\Programs\TodoAI"
    set "FOUND=1"
)

REM Check Program Files
if exist "%ProgramFiles%\TodoAI\todo-ai.exe" (
    echo Removing from Program Files...
    rmdir /S /Q "%ProgramFiles%\TodoAI"
    set "FOUND=1"
)

if "%FOUND%"=="0" (
    echo TODO AI not found in standard locations
    pause
    exit /b 1
)

echo.
echo ================================================
echo   Uninstall Complete!
echo ================================================
echo.
echo NOTE: Your data was NOT removed:
echo   - tasks.db in project directory
echo   - Config files
echo.
pause
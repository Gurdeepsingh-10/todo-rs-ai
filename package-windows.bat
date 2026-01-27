@echo off
echo Creating Windows installer...

REM Build release
cargo build --release

REM Create distribution folder
set "DIST_DIR=todo-ai-windows"
if exist "%DIST_DIR%" rmdir /S /Q "%DIST_DIR%"
mkdir "%DIST_DIR%"

REM Copy files
copy target\release\todo_tui.exe "%DIST_DIR%\todo-ai.exe"
copy README.md "%DIST_DIR%\"
copy install.bat "%DIST_DIR%\"

REM Create a simple installer script
(
echo @echo off
echo echo Installing TODO AI...
echo if not exist "%%ProgramFiles%%\TodoAI\" mkdir "%%ProgramFiles%%\TodoAI\"
echo copy todo-ai.exe "%%ProgramFiles%%\TodoAI\"
echo echo Installation complete!
echo echo Add %%ProgramFiles%%\TodoAI to your PATH to run 'todo-ai' from anywhere
echo pause
) > "%DIST_DIR%\INSTALL.bat"

REM Create zip
powershell Compress-Archive -Path "%DIST_DIR%\*" -DestinationPath "todo-ai-windows.zip" -Force

echo Done! Package: todo-ai-windows.zip
pause
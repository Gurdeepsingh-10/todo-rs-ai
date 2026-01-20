@echo off
echo Installing TODO AI for Windows...

REM Build release version
cargo build --release

REM Create Program Files directory
if not exist "%ProgramFiles%\TodoAI\" mkdir "%ProgramFiles%\TodoAI\"

REM Copy binary
copy target\release\todo_tui.exe "%ProgramFiles%\TodoAI\todo-ai.exe"

echo Installation complete!
echo Add %ProgramFiles%\TodoAI to your PATH to run 'todo-ai' from anywhere
pause
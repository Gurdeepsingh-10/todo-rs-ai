@echo off
echo Building TODO AI for Windows...

REM Debug build
cargo build

REM Release build
cargo build --release

echo Build complete!
echo Debug binary: target\debug\todo_tui.exe
echo Release binary: target\release\todo_tui.exe
pause
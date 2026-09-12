@echo off
cargo build --release || EXIT /B 1
copy /Y target\release\oppai.exe .\oppai.exe >nul
echo Build complete: oppai.exe

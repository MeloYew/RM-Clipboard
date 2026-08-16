@echo off
setlocal
cd /d "%~dp0"

where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust is required only to build RM Clipboard.
    echo Install it from https://rustup.rs/ and run this file again.
    exit /b 1
)

cargo build --release
if errorlevel 1 exit /b 1

copy /y "target\release\rm-clipboard.exe" "RMClipboard.exe" >nul
echo Built: %CD%\RMClipboard.exe

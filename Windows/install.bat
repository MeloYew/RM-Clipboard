@echo off
setlocal EnableExtensions
cd /d "%~dp0"

set "APP_DIR=%LOCALAPPDATA%\RM Clipboard"
set "MESSAGE_DIR=%USERPROFILE%\Documents\Support Messages\RM"
set "STARTUP_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup"
set "EXE_SOURCE=%~dp0RMClipboard.exe"
set "DOWNLOAD_URL=https://github.com/MeloYew/RM-Clipboard/releases/latest/download/RMClipboard.exe"

echo Installing RM Clipboard...
if not exist "%APP_DIR%" mkdir "%APP_DIR%"
if not exist "%MESSAGE_DIR%" mkdir "%MESSAGE_DIR%"

if not exist "%EXE_SOURCE%" (
    echo Downloading the latest native Windows executable...
    powershell -NoProfile -ExecutionPolicy Bypass -Command ^
      "Invoke-WebRequest -UseBasicParsing -Uri '%DOWNLOAD_URL%' -OutFile '%APP_DIR%\RMClipboard.exe'"
    if errorlevel 1 (
        echo Download failed. Download RMClipboard.exe from the GitHub Releases page.
        exit /b 1
    )
) else (
    copy /y "%EXE_SOURCE%" "%APP_DIR%\RMClipboard.exe" >nul
)

echo Importing missing RM message templates...
robocopy "%~dp0templates\RM" "%MESSAGE_DIR%" /E /XC /XN /XO >nul
if errorlevel 8 (
    echo Could not import the RM message templates.
    exit /b 1
)

echo Creating the Windows startup shortcut...
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$s=(New-Object -COM WScript.Shell).CreateShortcut('%STARTUP_DIR%\RM Clipboard.lnk');" ^
  "$s.TargetPath='%APP_DIR%\RMClipboard.exe';$s.WorkingDirectory='%APP_DIR%';$s.Save()"
if errorlevel 1 exit /b 1

taskkill /IM RMClipboard.exe /F >nul 2>nul
start "" "%APP_DIR%\RMClipboard.exe"

echo.
echo RM Clipboard is installed.
echo Messages: %MESSAGE_DIR%
echo Shortcut: Ctrl+Alt+Shift+M
exit /b 0

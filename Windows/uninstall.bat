@echo off
setlocal
taskkill /IM RMClipboard.exe /F >nul 2>nul
del /q "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\RM Clipboard.lnk" 2>nul
rmdir /s /q "%LOCALAPPDATA%\RM Clipboard" 2>nul
echo RM Clipboard was removed. Your message files were preserved in:
echo %USERPROFILE%\Documents\Support Messages\RM

# RM Clipboard for Windows 11

The Windows edition is a small native Win32 application written in Rust. It
runs silently in the background and registers **Win+Shift+M** as a global
shortcut. No Python runtime, AutoHotkey, or third-party UI framework is needed
on coworkers' computers.

It creates and reads messages from:

```text
C:\Users\<username>\Documents\Support Messages\RM\
```

## Install a release

Download this repository, then double-click `Windows\install.bat`. The installer:

1. creates the RM message directory;
2. copies only missing templates, preserving customized files;
3. downloads `RMClipboard.exe` from the latest GitHub release when it is not
   already beside the installer;
4. installs it under `%LOCALAPPDATA%\RM Clipboard\`;
5. creates a shortcut in the current user's Windows Startup folder; and
6. starts it immediately.

Press **Win+Shift+M**, double-click a folder to open it, or double-click a
message to copy it. Press Escape or close the window to hide it. The global
shortcut remains active in the background.

## Build from source

Install the stable Rust toolchain from <https://rustup.rs/>, then run:

```bat
cd Windows
build.bat
```

GitHub Actions also builds the native executable. Pushing a version tag such
as `v0.1.0` publishes `RMClipboard.exe` on the repository Releases page.

## Uninstall

Run `Windows\uninstall.bat`. It removes the application and Startup shortcut
but deliberately preserves all messages under Documents.

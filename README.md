# RM Clipboard

RM Clipboard is a fast support-message launcher for Linux and Windows. Both
versions use the same portable message library:

```text
~/Documents/Support Messages/RM/
```

Each `.txt` file becomes a button. Directories become categories, and clicking
a message copies its contents to the clipboard for use in support chats.

## Repository layout

```text
RM-Clipboard/
├── linux/
│   ├── install.sh
│   ├── rm_clipboard.py
│   └── templates/RM/
├── windows/
│   └── README.md
└── README.md
```

## Linux version

The current Linux version targets X11 with i3 and GTK 3. Its installer:

- installs Python, PyGObject, GTK 3 and i3 dependencies on Arch Linux;
- installs the launcher at `~/.local/bin/rm-clipboard`;
- creates `~/Documents/Support Messages/RM/` when it is missing;
- imports all bundled RM folders and `.txt` files;
- never overwrites an existing customized message;
- binds **Super+Shift+M** to launch RM Clipboard; and
- reloads i3 when it is already running.

Install it with:

```bash
git clone https://github.com/MeloYew/RM-Clipboard.git
cd RM-Clipboard/linux
chmod +x install.sh
./install.sh
```

Press **Super+Shift+M** after installation.

## Updating message templates

Edit the repository copies under `linux/templates/RM/` when changing the
defaults distributed to new users. Running the installer again adds new files
but deliberately leaves existing user messages untouched.

## Windows version

The Windows 11 launcher and installer will live under `windows/` and will use
the same `Documents/Support Messages/RM/` layout. It is not implemented yet.

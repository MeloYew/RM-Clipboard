# RM Clipboard

RM Clipboard is split into independent Linux and Windows versions. Both create
an `RM` folder inside the current user's Documents directory and import the
same portable message library.

```text
Linux:   /home/<username>/Documents/Support Messages/RM/
Windows: C:\Users\<username>\Documents\Support Messages\RM\
```

Each `.txt` file becomes a button. Directories become categories, and clicking
a message copies its contents to the clipboard for use in support chats.

## Repository layout

```text
RM-Clipboard/
├── Linux/
│   ├── install.sh
│   ├── rm_clipboard.py
│   └── templates/RM/
├── Windows/
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
cd RM-Clipboard/Linux
chmod +x install.sh
./install.sh
```

Press **Super+Shift+M** after installation.

## Updating message templates

Edit the repository copies under `Linux/templates/RM/` when changing the
defaults distributed to new users. Running the installer again adds new files
but deliberately leaves existing user messages untouched.

## Windows version

The Windows 11 launcher and installer will live under `Windows/` and create:

```text
C:\Users\<username>\Documents\Support Messages\RM\
```

It is not implemented yet.

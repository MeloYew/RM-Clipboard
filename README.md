# RM Clip

Eu precisava de uma ferramenta para agilizar o envio de mensagem para os clientes.
Inicialmente fiz o RM CLip para linux, depois recriei para windows.

Em ambos o sistema ele cria uma pasta "RM" dentro do folder de documentos do user
e importa as mensagems que eu envio.

```texto
Linux:   /home/<username>/Documents/Support Messages/RM/
Windows: C:\Users\<username>\Documents\Support Messages\RM\
```

Each `.txt` file becomes a button. Directories become categories, and clicking
a message copies its contents to the clipboard for use in support chats.

Cada `.txt` vira um botao. Diretorios viram as cateorias, e clicar nas mensagens
copiam elas para a clipboard, ai vc so cola depois.

## layout

```text
RM-Clipboard/
├── Linux/
│   ├── install.sh
│   ├── rm_clipboard.py
│   └── templates/RM/
├── Windows/
│   ├── install.bat
│   ├── uninstall.bat
│   ├── Cargo.toml
│   ├── src/
│   └── templates/RM/
└── README.md
```

## Linux 

So vai funcionar em x11 i3 GTK, e:

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

Para Windows 11, meti o louco e aprendi rust pra fazer uma versao que roda nativamente.
Nao tem runtime no pc, comeca sozinho com o windows, e registra **WIN+SHIFT+M** globalmente.
Ele cria e usa:

```text
C:\Users\<username>\Documents\Support Messages\RM\
```

Baixe e abra `Windows\install.bat`. Veja
[`Windows/README.md`](Windows/README.md) pra instalar, building, and
uninstallation.

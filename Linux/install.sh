#!/usr/bin/env bash
# Install RM Clipboard for Linux/X11/i3.
set -Eeuo pipefail

repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
launcher_source="${repo_dir}/rm_clipboard.py"
templates_dir="${repo_dir}/templates/RM"
launcher_target="${HOME}/.local/bin/rm-clipboard"
messages_dir="${HOME}/Documents/Support Messages/RM"
i3_config="${HOME}/.config/i3/config"
binding='bindsym $mod+Shift+m exec --no-startup-id ~/.local/bin/rm-clipboard'

log() { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }
die() { printf 'ERROR: %s\n' "$*" >&2; exit 1; }

[[ ${EUID} -ne 0 ]] || die "Run this installer as your regular user, not root."
[[ -f "$launcher_source" ]] || die "Missing launcher: ${launcher_source}"
[[ -d "$templates_dir" ]] || die "Missing RM templates: ${templates_dir}"

if command -v pacman >/dev/null 2>&1; then
    log "Installing Linux dependencies"
    sudo pacman -S --needed python python-gobject gtk3 i3-wm
else
    command -v python3 >/dev/null 2>&1 || die "Python 3 is required."
    command -v i3-msg >/dev/null 2>&1 || die "i3 is required for the keyboard shortcut."
    python3 -c 'import gi; gi.require_version("Gtk", "3.0")' 2>/dev/null || \
        die "PyGObject and GTK 3 are required. Install them with your distribution package manager."
fi

log "Installing the RM Clipboard launcher"
mkdir -p "${HOME}/.local/bin"
install -m 755 "$launcher_source" "$launcher_target"

log "Creating the shared RM message directory"
mkdir -p "$messages_dir"
# Archive/no-clobber preserves messages the user has already customized while
# adding every missing directory and starter template from this repository.
cp -an "${templates_dir}/." "${messages_dir}/"

log "Configuring Super+Shift+M in i3"
mkdir -p "$(dirname "$i3_config")"
touch "$i3_config"
if grep -Fq 'bindsym $mod+Shift+m ' "$i3_config"; then
    sed -i 's|^bindsym \$mod+Shift+m .*$|bindsym $mod+Shift+m exec --no-startup-id ~/.local/bin/rm-clipboard|' "$i3_config"
else
    printf '\n# RM Clipboard support-message launcher\n%s\n' "$binding" >> "$i3_config"
fi

if command -v i3-msg >/dev/null 2>&1 && i3-msg -t get_version >/dev/null 2>&1; then
    i3-msg reload >/dev/null
    printf 'Reloaded the active i3 configuration.\n'
fi

log "RM Clipboard is installed"
printf 'Messages: %s\nLauncher: %s\nShortcut: Super+Shift+M\n' \
    "$messages_dir" "$launcher_target"

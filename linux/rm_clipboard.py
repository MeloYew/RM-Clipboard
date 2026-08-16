#!/usr/bin/env python3

import re
import signal
from pathlib import Path

import gi

gi.require_version("Gtk", "3.0")
gi.require_version("Gdk", "3.0")
from gi.repository import Gdk, GLib, Gtk  # noqa: E402


MESSAGES_DIR = Path.home() / "Documents" / "Support Messages" / "RM"
ITEMS_PER_COLUMN = 9


def natural_key(path):
    return [int(part) if part.isdigit() else part.casefold()
            for part in re.split(r"(\d+)", path.name)]


class MessageLauncher(Gtk.Window):
    def __init__(self):
        super().__init__(type=Gtk.WindowType.POPUP)
        self.set_name("overlay")
        self.set_decorated(False)
        self.set_skip_taskbar_hint(True)
        self.set_skip_pager_hint(True)
        self.set_keep_above(True)
        self.set_position(Gtk.WindowPosition.CENTER_ALWAYS)
        self.set_app_paintable(True)

        screen = self.get_screen()
        visual = screen.get_rgba_visual()
        if visual:
            self.set_visual(visual)

        self.connect("key-press-event", self.on_key_press)
        self.connect("button-press-event", self.on_background_click)
        self.connect("destroy", Gtk.main_quit)

        self.columns = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=14)
        self.columns.set_halign(Gtk.Align.CENTER)
        self.columns.set_valign(Gtk.Align.CENTER)
        self.add(self.columns)

        self.apply_style()
        self.add_column(MESSAGES_DIR, 0)

    def apply_style(self):
        css = b"""
        #overlay { background-color: rgba(8, 11, 18, 0.72); }
        .branch {
            background-color: rgba(22, 27, 38, 0.97);
            border: 1px solid #58657a;
            border-radius: 12px;
            padding: 12px;
        }
        .title { color: #9aa9bd; font-size: 12px; font-weight: bold; }
        button {
            min-width: 170px;
            min-height: 38px;
            padding: 5px 14px;
            color: #eef3fa;
            background-image: none;
            background-color: #30394a;
            border: 0;
            border-radius: 8px;
            font-weight: bold;
        }
        button:hover { background-color: #586f98; }
        button.folder { background-color: #3d4b62; }
        button.folder:hover { background-color: #627ba8; }
        .empty { color: #8995a6; padding: 12px; }
        """
        provider = Gtk.CssProvider()
        provider.load_from_data(css)
        Gtk.StyleContext.add_provider_for_screen(
            self.get_screen(), provider, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

    def add_column(self, directory, depth):
        children = self.columns.get_children()
        for child in children[depth:]:
            self.columns.remove(child)

        panel = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=8)
        # A horizontal Gtk.Box stretches children along its vertical axis by
        # default. Keep each branch at its own natural height instead, so a
        # long list in a later column does not make earlier columns look tall.
        panel.set_valign(Gtk.Align.CENTER)
        panel.get_style_context().add_class("branch")

        title = Gtk.Label(label=directory.name if depth else "RM CLIPBOARD")
        title.get_style_context().add_class("title")
        panel.pack_start(title, False, False, 2)

        entries = []
        if directory.is_dir():
            entries = sorted(
                (item for item in directory.iterdir()
                 if item.is_dir() or item.suffix.casefold() == ".txt"),
                key=natural_key,
            )

        entry_columns = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=8)
        panel.pack_start(entry_columns, False, False, 0)

        current_column = None
        for index, item in enumerate(entries):
            if index % ITEMS_PER_COLUMN == 0:
                current_column = Gtk.Box(
                    orientation=Gtk.Orientation.VERTICAL, spacing=8
                )
                entry_columns.pack_start(current_column, False, False, 0)

            label = item.stem if item.is_file() else item.name
            button = Gtk.Button(label=(label + "  ›") if item.is_dir() else label)
            if item.is_dir():
                button.get_style_context().add_class("folder")
                button.connect("clicked", self.open_folder, item, depth + 1)
            else:
                button.connect("clicked", self.copy_message, item)
            current_column.pack_start(button, False, False, 0)

        if not entries:
            empty = Gtk.Label(label="No folders or .txt files")
            empty.get_style_context().add_class("empty")
            entry_columns.pack_start(empty, False, False, 0)

        self.columns.pack_start(panel, False, False, 0)
        self.show_all()

    def open_folder(self, _button, directory, depth):
        self.add_column(directory, depth)

    def copy_message(self, _button, message_file):
        self.copy_files([message_file])

    def copy_files(self, message_files):
        try:
            message = "\n".join(
                message_file.read_text(encoding="utf-8").strip()
                for message_file in message_files
            )
        except (OSError, UnicodeError) as error:
            dialog = Gtk.MessageDialog(
                transient_for=self,
                flags=0,
                message_type=Gtk.MessageType.ERROR,
                buttons=Gtk.ButtonsType.CLOSE,
                text="Could not read message",
            )
            dialog.format_secondary_text(str(error))
            dialog.run()
            dialog.destroy()
            return

        clipboard = Gtk.Clipboard.get(Gdk.SELECTION_CLIPBOARD)
        clipboard.set_text(message, -1)
        self.clipboard = clipboard
        self.hide()

        # X11 clipboard data belongs to the process that copied it. Keep this
        # process invisibly alive until another app takes ownership (or for a
        # maximum of ten minutes), since this desktop has no clipboard manager.
        GLib.timeout_add(200, self.watch_clipboard)
        GLib.timeout_add_seconds(600, Gtk.main_quit)

    def watch_clipboard(self):
        self.clipboard.connect("owner-change", lambda *_args: Gtk.main_quit())
        return False

    def on_key_press(self, _window, event):
        if event.keyval == Gdk.KEY_Escape:
            Gtk.main_quit()
            return True
        return False

    def on_background_click(self, _window, event):
        if event.window == self.get_window():
            Gtk.main_quit()


def main():
    MESSAGES_DIR.mkdir(parents=True, exist_ok=True)
    signal.signal(signal.SIGTERM, lambda *_args: Gtk.main_quit())
    window = MessageLauncher()
    window.fullscreen()
    window.show_all()
    Gtk.main()


if __name__ == "__main__":
    main()

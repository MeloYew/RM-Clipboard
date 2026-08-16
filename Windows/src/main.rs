#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::DataExchange::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::Ole::CF_UNICODETEXT;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const HOTKEY_ID: i32 = 1;
const LIST_ID: usize = 101;
const BACK_ID: usize = 102;
const WIDTH: i32 = 620;
const HEIGHT: i32 = 620;

struct AppState {
    root: PathBuf,
    current: PathBuf,
    entries: Vec<PathBuf>,
    hwnd: HWND,
    title: HWND,
    list: HWND,
    back: HWND,
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn rm_directory() -> PathBuf {
    let profile = env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .unwrap_or_default();
    profile
        .join("Documents")
        .join("Support Messages")
        .join("RM")
}

fn display_name(path: &Path) -> String {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    if path.is_dir() {
        format!("[Folder]  {name}")
    } else {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
}

unsafe fn state_from(hwnd: HWND) -> Option<&'static mut AppState> {
    let pointer = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
    pointer.as_mut()
}

unsafe fn set_text(hwnd: HWND, value: &str) {
    let value = wide(value);
    SetWindowTextW(hwnd, value.as_ptr());
}

unsafe fn refresh(state: &mut AppState) {
    state.entries.clear();
    if let Ok(items) = fs::read_dir(&state.current) {
        state.entries = items
            .filter_map(Result::ok)
            .map(|item| item.path())
            .filter(|path| {
                path.is_dir()
                    || path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
            })
            .collect();
    }
    state.entries.sort_by_key(|path| {
        (
            !path.is_dir(),
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase(),
        )
    });

    SendMessageW(state.list, LB_RESETCONTENT, 0, 0);
    for path in &state.entries {
        let label = wide(&display_name(path));
        SendMessageW(state.list, LB_ADDSTRING, 0, label.as_ptr() as isize);
    }

    let relative = state
        .current
        .strip_prefix(&state.root)
        .unwrap_or(Path::new(""));
    let title = if relative.as_os_str().is_empty() {
        "RM CLIPBOARD".to_string()
    } else {
        format!("RM CLIPBOARD  /  {}", relative.display())
    };
    set_text(state.title, &title);
    EnableWindow(state.back, (state.current != state.root) as i32);
}

unsafe fn copy_to_clipboard(hwnd: HWND, text: &str) -> bool {
    let data: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    if OpenClipboard(hwnd) == 0 {
        return false;
    }
    EmptyClipboard();
    let bytes = data.len() * std::mem::size_of::<u16>();
    let memory = GlobalAlloc(GMEM_MOVEABLE, bytes);
    if memory.is_null() {
        CloseClipboard();
        return false;
    }
    let target = GlobalLock(memory) as *mut u16;
    if target.is_null() {
        GlobalFree(memory);
        CloseClipboard();
        return false;
    }
    std::ptr::copy_nonoverlapping(data.as_ptr(), target, data.len());
    GlobalUnlock(memory);
    if SetClipboardData(CF_UNICODETEXT as u32, memory as HANDLE).is_null() {
        GlobalFree(memory);
        CloseClipboard();
        return false;
    }
    CloseClipboard();
    true
}

unsafe fn activate_selected(state: &mut AppState) {
    let selected = SendMessageW(state.list, LB_GETCURSEL, 0, 0);
    if selected == LB_ERR as isize {
        return;
    }
    let Some(path) = state.entries.get(selected as usize).cloned() else {
        return;
    };
    if path.is_dir() {
        state.current = path;
        refresh(state);
        return;
    }
    match fs::read_to_string(&path) {
        Ok(message) if copy_to_clipboard(state.hwnd, message.trim()) => {
            ShowWindow(state.hwnd, SW_HIDE);
        }
        _ => {
            let body = wide("The selected message could not be copied.");
            let title = wide("RM Clipboard");
            MessageBoxW(
                state.hwnd,
                body.as_ptr(),
                title.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_CREATE => {
            let create = &*(lparam as *const CREATESTRUCTW);
            let state = create.lpCreateParams as *mut AppState;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as isize);
            (*state).hwnd = hwnd;

            let static_class = wide("STATIC");
            let list_class = wide("LISTBOX");
            let button_class = wide("BUTTON");
            let empty = wide("");
            let back_label = wide("Back");
            (*state).title = CreateWindowExW(
                0,
                static_class.as_ptr(),
                empty.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                18,
                16,
                WIDTH - 120,
                28,
                hwnd,
                null_mut(),
                null_mut(),
                null(),
            );
            (*state).back = CreateWindowExW(
                0,
                button_class.as_ptr(),
                back_label.as_ptr(),
                WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32,
                WIDTH - 96,
                10,
                70,
                32,
                hwnd,
                BACK_ID as *mut std::ffi::c_void,
                null_mut(),
                null(),
            );
            (*state).list = CreateWindowExW(
                WS_EX_CLIENTEDGE,
                list_class.as_ptr(),
                empty.as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_VSCROLL | LBS_NOTIFY as u32,
                18,
                54,
                WIDTH - 52,
                HEIGHT - 112,
                hwnd,
                LIST_ID as *mut std::ffi::c_void,
                null_mut(),
                null(),
            );
            refresh(&mut *state);
            0
        }
        WM_HOTKEY => {
            if let Some(state) = state_from(hwnd) {
                state.current = state.root.clone();
                refresh(state);
                let screen_width = GetSystemMetrics(SM_CXSCREEN);
                let screen_height = GetSystemMetrics(SM_CYSCREEN);
                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    (screen_width - WIDTH) / 2,
                    (screen_height - HEIGHT) / 2,
                    WIDTH,
                    HEIGHT,
                    SWP_SHOWWINDOW,
                );
                SetForegroundWindow(hwnd);
                SetFocus(state.list);
            }
            0
        }
        WM_COMMAND => {
            let control_id = (wparam & 0xffff) as usize;
            let notification = ((wparam >> 16) & 0xffff) as u16;
            if let Some(state) = state_from(hwnd) {
                if control_id == BACK_ID && notification == BN_CLICKED as u16 {
                    if let Some(parent) = state.current.parent() {
                        if parent.starts_with(&state.root) {
                            state.current = parent.to_path_buf();
                            refresh(state);
                        }
                    }
                } else if control_id == LIST_ID && notification == LBN_DBLCLK as u16 {
                    activate_selected(state);
                }
            }
            0
        }
        WM_CLOSE => {
            ShowWindow(hwnd, SW_HIDE);
            0
        }
        WM_DESTROY => {
            UnregisterHotKey(hwnd, HOTKEY_ID);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

fn main() {
    unsafe {
        let root = rm_directory();
        let _ = fs::create_dir_all(&root);
        let mut state = Box::new(AppState {
            root: root.clone(),
            current: root,
            entries: Vec::new(),
            hwnd: null_mut(),
            title: null_mut(),
            list: null_mut(),
            back: null_mut(),
        });

        let instance = GetModuleHandleW(null());
        let class_name = wide("RMClipboardWindow");
        let window_title = wide("RM Clipboard");
        let mut class: WNDCLASSW = std::mem::zeroed();
        class.style = CS_HREDRAW | CS_VREDRAW;
        class.lpfnWndProc = Some(window_proc);
        class.hInstance = instance;
        class.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        class.hbrBackground = (COLOR_WINDOW as isize + 1) as HBRUSH;
        class.lpszClassName = class_name.as_ptr();
        if RegisterClassW(&class) == 0 {
            return;
        }

        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW,
            class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH,
            HEIGHT,
            null_mut(),
            null_mut(),
            instance,
            state.as_mut() as *mut AppState as *const _,
        );
        if hwnd.is_null() {
            return;
        }

        if RegisterHotKey(
            hwnd,
            HOTKEY_ID,
            MOD_WIN | MOD_SHIFT | MOD_NOREPEAT,
            b'M' as u32,
        ) == 0
        {
            let body = wide("Win+Shift+M is already used by another program.");
            MessageBoxW(
                hwnd,
                body.as_ptr(),
                window_title.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
            DestroyWindow(hwnd);
            return;
        }

        let _state = Box::into_raw(state);
        let mut message: MSG = std::mem::zeroed();
        while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
            if message.message == WM_KEYDOWN && message.wParam == VK_ESCAPE as usize {
                ShowWindow(hwnd, SW_HIDE);
                continue;
            }
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

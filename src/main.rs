#![windows_subsystem = "windows"]
mod engine;

use std::sync::Mutex;
use lazy_static::lazy_static;
use windows::Win32::Foundation::{RECT, LPARAM, WPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, SetWindowPos, SendMessageW, WM_SETREDRAW, 
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx, GetMessageW, 
    DispatchMessageW, MSG, WH_KEYBOARD_LL, KBDLLHOOKSTRUCT, WM_KEYDOWN, 
    WM_SYSKEYDOWN, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOZORDER, 
    SWP_NOCOPYBITS, SWP_NOSENDCHANGING, HHOOK, GetWindowRect,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, 
    VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9,
};
use windows::Win32::Graphics::Gdi::{
    MonitorFromWindow, MONITOR_DEFAULTTONEAREST, GetMonitorInfoW, MONITORINFO, 
    RedrawWindow, RDW_INVALIDATE, RDW_UPDATENOW, RDW_ALLCHILDREN,
};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};

// --- App State ---

struct AppState {
    last_hwnd: isize,
    cycle_count: usize,
    last_key: u32,
}

lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(AppState {
        last_hwnd: 0,
        cycle_count: 0,
        last_key: 0,
    });
}

// --- Snapshot Action ---

fn perform_snap(key: u32) {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() { return; }
        
        // 1. Update State
        let mut state = STATE.lock().unwrap();
        if state.last_hwnd == hwnd.0 as isize && state.last_key == key {
            state.cycle_count += 1;
        } else {
            state.last_hwnd = hwnd.0 as isize;
            state.last_key = key;
            state.cycle_count = 0;
        }
        let cycle = state.cycle_count;

        // 2. Gather Context
        let mut current_rect = RECT::default();
        let _ = GetWindowRect(hwnd, &mut current_rect);

        let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() { return; }
        let work_area = monitor_info.rcWork;

        // 3. Functional Calculation (No Side Effects)
        let target = engine::calculate_snap(key, cycle, current_rect, work_area);

        // 4. Border Compensation (Invisible resize borders adjustment)
        let mut extended_rect = RECT::default();
        let _ = DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut extended_rect as *mut RECT as *mut _,
            std::mem::size_of::<RECT>() as u32,
        );

        let border_left = extended_rect.left - current_rect.left;
        let border_top = extended_rect.top - current_rect.top;
        let border_right = current_rect.right - extended_rect.right;
        let border_bottom = current_rect.bottom - extended_rect.bottom;

        let final_left = target.left - border_left;
        let final_top = target.top - border_top;
        let final_width = (target.right - target.left) + border_left + border_right;
        let final_height = (target.bottom - target.top) + border_top + border_bottom;

        // 5. Execution (Side Effects Isolated)
        let _ = SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(0)), Some(LPARAM(0)));
        let _ = SetWindowPos(
            hwnd, 
            None, 
            final_left, 
            final_top, 
            final_width, 
            final_height, 
            SWP_NOSENDCHANGING | SWP_NOCOPYBITS | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED
        );
        let _ = SendMessageW(hwnd, WM_SETREDRAW, Some(WPARAM(1)), Some(LPARAM(0)));
        let _ = RedrawWindow(Some(hwnd), None, None, RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);
    }
}

// --- Keyboard Hook ---

static mut HOOK_HANDLE: Option<HHOOK> = None;

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let msg = wparam.0 as u32;
        if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
            let kbd = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let ctrl = (GetAsyncKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0;
            let alt = (GetAsyncKeyState(VK_MENU.0 as i32) as u16 & 0x8000) != 0;

            if ctrl && alt {
                let nkey = match kbd.vkCode {
                    x if x == VK_NUMPAD1.0 as u32 => Some(1),
                    x if x == VK_NUMPAD2.0 as u32 => Some(2),
                    x if x == VK_NUMPAD3.0 as u32 => Some(3),
                    x if x == VK_NUMPAD4.0 as u32 => Some(4),
                    x if x == VK_NUMPAD5.0 as u32 => Some(5),
                    x if x == VK_NUMPAD6.0 as u32 => Some(6),
                    x if x == VK_NUMPAD7.0 as u32 => Some(7),
                    x if x == VK_NUMPAD8.0 as u32 => Some(8),
                    x if x == VK_NUMPAD9.0 as u32 => Some(9),
                    _ => None,
                };

                if let Some(k) = nkey {
                    perform_snap(k);
                    return LRESULT(1); // Block
                }
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

fn main() {
    unsafe {
        let h_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0).expect("Hook Failed");
        HOOK_HANDLE = Some(h_hook);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            DispatchMessageW(&msg);
        }

        if let Some(h) = HOOK_HANDLE {
            let _ = UnhookWindowsHookEx(h);
        }
    }
}

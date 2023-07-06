// use crate::macros::WindowBound;
use crate::sonny::WindowBound;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowInfo};

#[cfg(target_os = "windows")]
pub fn get_window_size() -> WindowBound {
    unsafe {
        let handle = GetForegroundWindow();
        let mut win_info = Default::default();
        GetWindowInfo(handle, &mut win_info);
        WindowBound {
            top: win_info.rcClient.top,
            left: win_info.rcClient.left,
            right: win_info.rcClient.right,
            bottom: win_info.rcClient.bottom,
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_window_size() -> WindowBound {
    println!("Not implemented for this os.");
    WindowBound::default()
}

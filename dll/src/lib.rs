#![cfg(target_os = "windows")]

// logging
use once_cell::sync::OnceCell;
use tracing_appender::non_blocking::WorkerGuard;
use std::fs::create_dir_all;
// winapi/c interactions
use std::os::raw::c_void;
use windows::Win32::Foundation::*;
// DLlMain
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
// GetClipboardData
mod set_clipboard_data;
// SetClipboardData
mod get_clipboard_data;

// logging
static GUARD: OnceCell<WorkerGuard> = OnceCell::new();
fn setup_logging() -> WorkerGuard {
    // log path => AppData\Local\ClipMon
    let log_path = dirs::cache_dir().unwrap().join("ClipMon");
    create_dir_all(&log_path).unwrap();
    let file_appender = tracing_appender::rolling::daily(log_path, "clipmon.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    guard
}

#[no_mangle]
unsafe extern "system" fn DllMain(_hinst: HANDLE, reason: u32, _reserved: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        GUARD.set(setup_logging()).unwrap();
        tracing::info!("Dll is injected");
        get_clipboard_data::hook_get_clipboard_data();
        set_clipboard_data::hook_set_clipboard_data();
        tracing::info!("The hooks are setup");
    }
    BOOL::from(true)
}

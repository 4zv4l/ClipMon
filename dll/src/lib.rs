#![cfg(target_os = "windows")]

// logging
use once_cell::sync::OnceCell;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
// winapi/c interactions
use std::os::raw::c_void;
use windows::Win32::Foundation::*;
// DLlMain
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
// GetClipboardData
mod set_clipboard_data;
// SetClipboardData
mod get_clipboard_data;

// logging
static GUARD: OnceCell<WorkerGuard> = OnceCell::new();
fn setup_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("", "clipmon.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .finish()
            .with(fmt::Layer::default().with_writer(file_writer))
    ).expect("Unable to set global tracing subscriber");
    guard
}

#[no_mangle]
unsafe extern "system" fn DllMain(_hinst: HANDLE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            GUARD.set(setup_logging()).unwrap();
            tracing::info!("Dll is injected");
            get_clipboard_data::hook_get_clipboard_data();
            set_clipboard_data::hook_set_clipboard_data();
            tracing::info!("The hooks are setup");
        },
        DLL_PROCESS_DETACH => {},
        _ => {},
    };
    BOOL::from(true)
}

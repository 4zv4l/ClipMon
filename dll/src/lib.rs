#![cfg(target_os = "windows")]

// logging
use once_cell::sync::OnceCell;
use tracing_appender::non_blocking::WorkerGuard;
// hooking
use retour::static_detour;
use libloading::Library;
// winapi/c interactions
use std::os::raw::c_void;
use windows::Win32::Foundation::*;
// DLlMain
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
// clipboard handling
use clipboard_win::{formats, get_clipboard};

// GetClipboardData
type FnGetClipboardData = unsafe extern "system" fn (uFormat: u32) -> HANDLE;
static_detour! {static HookGetClipboardData: unsafe extern "system" fn (u32) -> HANDLE;}
fn hook_get_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnGetClipboardData>(b"GetClipboardData\0").unwrap().into_raw() };
    unsafe {HookGetClipboardData.initialize(*address, detour_get_clipboard_data).expect("init").enable().expect("enable");}
}
fn detour_get_clipboard_data(uformat: u32) -> HANDLE {
    let content: String = get_clipboard(formats::Unicode).unwrap_or("".into());
    tracing::info!("content => {content}");
    let ret = unsafe { HookGetClipboardData.call(uformat) };
    ret
}

// SetClipboardData
type FnSetClipboardData = unsafe extern "system" fn (uFormat: u32, hMem: HANDLE) -> HANDLE;
static_detour! {static HookSetClipboardData: unsafe extern "system" fn (u32, HANDLE) -> HANDLE;}
fn hook_set_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnSetClipboardData>(b"SetClipboardData\0").unwrap().into_raw() };
    unsafe {HookSetClipboardData.initialize(*address, detour_set_clipboard_data).expect("init").enable().expect("enable");}
}
fn detour_set_clipboard_data(uformat: u32, hmem: HANDLE) -> HANDLE {
    let ret = unsafe { HookSetClipboardData.call(uformat, hmem) };
    ret
}

// logging
static GUARD: OnceCell<WorkerGuard> = OnceCell::new();
fn setup_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("", "clipmon.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    _guard
}

#[no_mangle]
unsafe extern "system" fn DllMain(_hinst: HANDLE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            GUARD.set(setup_logging()).unwrap();
            tracing::info!("Dll is injected");
            hook_get_clipboard_data();
            hook_set_clipboard_data();
            tracing::info!("The hooks are setup");
        },
        DLL_PROCESS_DETACH => {},
        _ => {},
    };
    return BOOL::from(true);
}

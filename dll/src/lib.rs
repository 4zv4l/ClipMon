#![cfg(target_os = "windows")]

// logging
use once_cell::sync::OnceCell;
use tracing_appender::non_blocking::WorkerGuard;
// hooking
use retour::static_detour;
use libloading::Library;
// winapi/c interactions
use std::os::raw::c_void;
use windows::{s,core::PCSTR, Win32::Foundation::*};
// DLlMain
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

type FnMessageBoxA = unsafe extern "system" fn (HWND, PCSTR, PCSTR, u32) -> i32;
static GUARD: OnceCell<WorkerGuard> = OnceCell::new();

static_detour! {
    static HookMessageBox: unsafe extern "system" fn(HWND, PCSTR, PCSTR, u32) -> i32;
}

fn detour_messagebox(hwnd: HWND, text: PCSTR, _caption: PCSTR, msgbox_style: u32) -> i32 {
    tracing::info!("Before MessageBox");
    let ret = unsafe { HookMessageBox.call(hwnd, text, s!("Detoured"), msgbox_style) };
    tracing::info!("After MessageBox");
    ret
}

fn hook_message_box() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnMessageBoxA>(b"MessageBoxA\0").unwrap().into_raw() };
    unsafe {HookMessageBox.initialize(*address, detour_messagebox).expect("init").enable().expect("enable");}
}

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
            hook_message_box();
            tracing::info!("The hook is setup");
        },
        DLL_PROCESS_DETACH => {},
        _ => {},
    };
    return BOOL::from(true);
}

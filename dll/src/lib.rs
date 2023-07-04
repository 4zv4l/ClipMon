#![cfg(target_os = "windows")]

// hooking
use retour::static_detour;
use libloading::Library;
// winapi/c interactions
use std::os::raw::c_void;
use windows::{s,core::PCSTR, Win32::Foundation::*};
// DLlMain
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};


type FnMessageBoxA = unsafe extern "system" fn (HWND, PCSTR, PCSTR, u32) -> i32;

static_detour! {
    static HookMessageBox: unsafe extern "system" fn(HWND, PCSTR, PCSTR, u32) -> i32;
}

fn detour_messagebox(hwnd: HWND, text: PCSTR, _caption: PCSTR, msgbox_style: u32) -> i32 {
    unsafe { HookMessageBox.call(hwnd, text, s!("Detoured"), msgbox_style) }
}

fn hook_message_box() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnMessageBoxA>(b"MessageBoxA\0").unwrap().into_raw() };
    unsafe {HookMessageBox.initialize(*address, detour_messagebox).expect("init").enable().expect("enable");}
}

#[no_mangle]
unsafe extern "system" fn DllMain(_hinst: HANDLE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            println!("Hello ! I am injected !!!");
            hook_message_box();
            println!("Hook is made !");
        },
        DLL_PROCESS_DETACH => {
            println!("Hello ! I being detached :(");
        },
        _ => {},
    };
    return BOOL::from(true);
}

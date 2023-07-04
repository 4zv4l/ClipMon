// clipboard handling
use std::ptr::null;
use std::ffi::CString;
use windows::Win32::System::Memory::{GlobalLock,GlobalUnlock};
use clipboard_win::{formats, set_clipboard};
// hooking
use retour::static_detour;
use libloading::Library;
use windows::Win32::Foundation::{HANDLE,HGLOBAL};

// setup hooking
type FnSetClipboardData = unsafe extern "system" fn (uFormat: u32, hMem: HANDLE) -> HANDLE;
static_detour! {static HookSetClipboardData: unsafe extern "system" fn (u32, HANDLE) -> HANDLE;}
pub fn hook_set_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnSetClipboardData>(b"SetClipboardData\0").unwrap().into_raw() };
    unsafe {HookSetClipboardData.initialize(*address, detour_set_clipboard_data).expect("init").enable().expect("enable");}
}

fn handle_to_string(hmem: HANDLE) -> String {
    let c_string = unsafe {GlobalLock::<HGLOBAL>(std::mem::transmute(hmem)) as *const i8};
    if c_string == null() { return "".into() }
    let c_string = unsafe { CString::from_raw(c_string as *mut i8) }.into_string().unwrap();
    unsafe {GlobalUnlock::<HGLOBAL>(std::mem::transmute(hmem))};
    c_string
}

// hooked function
fn detour_set_clipboard_data(uformat: u32, hmem: HANDLE) -> HANDLE {
    tracing::info!("checking");
    match uformat {
        // if text then check
        formats::CF_UNICODETEXT|formats::CF_TEXT => {
            let content = handle_to_string(hmem);
            tracing::info!(":: {content} ::");
            unsafe { HookSetClipboardData.call(uformat, hmem) }
        },
        // else error and log
        _ => {
            tracing::warn!("Illegal format {uformat}");
            HANDLE(0)
        }
    }
}

// clipboard handling
use clipboard_win::{formats, set_clipboard};
// hooking
use retour::static_detour;
use libloading::Library;
use windows::Win32::Foundation::HANDLE;

// setup hooking
type FnSetClipboardData = unsafe extern "system" fn (uFormat: u32, hMem: HANDLE) -> HANDLE;
static_detour! {static HookSetClipboardData: unsafe extern "system" fn (u32, HANDLE) -> HANDLE;}
pub fn hook_set_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnSetClipboardData>(b"SetClipboardData\0").unwrap().into_raw() };
    unsafe {HookSetClipboardData.initialize(*address, detour_set_clipboard_data).expect("init").enable().expect("enable");}
}

// hooked function
fn detour_set_clipboard_data(uformat: u32, hmem: HANDLE) -> HANDLE {
    tracing::info!("SetClipboardData");
    unsafe { HookSetClipboardData.call(uformat, hmem) }
}

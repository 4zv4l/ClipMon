// clipboard handling
use clipboard_win::{formats, get_clipboard};
// hooking
use retour::static_detour;
use libloading::Library;
use windows::Win32::Foundation::HANDLE;

// setup hooking
type FnGetClipboardData = unsafe extern "system" fn (uFormat: u32) -> HANDLE;
static_detour! {static HookGetClipboardData: unsafe extern "system" fn (u32) -> HANDLE;}
pub fn hook_get_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnGetClipboardData>(b"GetClipboardData\0").unwrap().into_raw() };
    unsafe {HookGetClipboardData.initialize(*address, detour_get_clipboard_data).expect("init").enable().expect("enable");}
}

// hooked function
fn detour_get_clipboard_data(uformat: u32) -> HANDLE {
    tracing::info!("GetClipboardData :: Hooked");
    let content: String = get_clipboard(formats::Unicode).unwrap_or("".into());
    tracing::info!("GetClipboardData :: {content}");
    unsafe { HookGetClipboardData.call(uformat) }
}

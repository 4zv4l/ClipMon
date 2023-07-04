// clipboard handling
use clipboard_win::{formats, get_clipboard, Getter};
// hooking
use retour::static_detour;
use libloading::Library;
use windows::Win32::Foundation::HANDLE;

// setup hooking
type FnGetClipboardData = unsafe extern "system" fn (uFormat: u32) -> HANDLE;
static_detour! { static HookGetClipboardData: unsafe extern "system" fn (u32) -> HANDLE; }
pub fn hook_get_clipboard_data() {
    let user32 = unsafe { Library::new("user32.dll").unwrap() };
    let address = unsafe { user32.get::<FnGetClipboardData>(b"GetClipboardData\0").unwrap().into_raw() };
    unsafe { HookGetClipboardData.initialize(*address, detour_get_clipboard_data).expect("init").enable().expect("enable"); }
}

// hooked function
fn detour_get_clipboard_data(uformat: u32) -> HANDLE {
    tracing::info!("checking");
    // if handle is 0 just skip it
    let ret = unsafe { HookGetClipboardData.call(uformat) };
    if ret == HANDLE(0) { return HANDLE(0) }

    // disable the hook to avoid infinite loop
    unsafe {HookGetClipboardData.disable().unwrap();}

    match uformat {
        // if text then check and log
        formats::CF_UNICODETEXT|formats::CF_TEXT => {
            let content = get_clipboard::<String, formats::Unicode>(formats::Unicode).unwrap();
            tracing::info!("'{content}'");
            unsafe { HookGetClipboardData.enable().unwrap(); }
            ret
        },
        // else error, if files then show the copied paths
        _ => {
            tracing::warn!("Illegal format {uformat}");
            if uformat == formats::CF_HDROP {
                let mut files = Vec::<String>::with_capacity(5);
                if formats::FileList.read_clipboard(&mut files).is_err() { tracing::error!("couldn't read paths") }
                tracing::warn!("=> {:?}", files);
            }
            unsafe { HookGetClipboardData.enable().unwrap(); }
            HANDLE(0)
        }
    }
}

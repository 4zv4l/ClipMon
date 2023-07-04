#![cfg(target_os = "windows")]

use dll_syringe::{Syringe, process::OwnedProcess};

fn main() {
    const TARGET: &str = "loop_messagebox.exe";

    match OwnedProcess::find_first_by_name(TARGET) {
        Some(process) => {
            // create a new syringe for the target process
            let syringe = Syringe::for_process(process);

            // inject the payload into the target process
            let _injected_payload = syringe.inject("clipmon.dll").unwrap();
            println!("Dll has been injected :)");
        },
        None => {
            return eprintln!("couldn't find {TARGET}");
        }
    }

}

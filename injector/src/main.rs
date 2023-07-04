#![cfg(target_os = "windows")]

use dll_syringe::{Syringe, process::OwnedProcess};
use std::process::Command;
use std::os::windows::process::CommandExt;

fn start_process(proc: &str) -> Result<std::process::Child, std::io::Error> {
    const DEBUG_PROCESS: u32 = 0x00000001;
    const DETACHED_PROCESS: u32 = 0x00000008;
    Command::new(proc).creation_flags(DEBUG_PROCESS|DETACHED_PROCESS).spawn()
}

fn main() {
    const TARGET_EXE: &str = "loop_messagebox.exe";
    const TARGET: &str = "loop_messagebox";
    const DLL: &str = "clipmon.dll";
    
    println!("Creating {TARGET_EXE}");
    let proc = start_process(TARGET_EXE).unwrap();
    println!("{TARGET_EXE} is Created => {}", proc.id());
    match OwnedProcess::find_first_by_name(TARGET) {
        Some(process) => {
            println!("found {:?}", process);
            // create a new syringe for the target process
            let syringe = Syringe::for_process(process);

            println!("Injecting Dll...");
            let _injected_payload = syringe.inject(DLL).unwrap();
            println!("Dll has been injected :)");
        },
        None => {
            return eprintln!("couldn't find {TARGET}");
        }
    }
}

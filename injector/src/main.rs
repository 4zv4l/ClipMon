#![cfg(target_os = "windows")]

use dll_syringe::{Syringe, process::OwnedProcess};
use std::process::Command;
use std::os::windows::process::CommandExt;

extern "system" { fn DebugActiveProcessStop(dwProcessId: u32) -> bool; }

fn start_process(proc: &str) -> Result<std::process::Child, std::io::Error> {
    const DEBUG_PROCESS: u32 = 0x00000001;
    let proc = Command::new(proc).creation_flags(DEBUG_PROCESS).spawn()?;
    unsafe { DebugActiveProcessStop(proc.id()); }
    Ok(proc)
}

fn main() {
    const TARGET: &str = "loop_messagebox.exe";
    const DLL: &str = "clipmon.dll";
    
    println!("Creating {TARGET}");
    let mut proc = start_process(TARGET).unwrap();
    println!("{TARGET} is Created => {}", proc.id());
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
    std::thread::sleep(std::time::Duration::from_secs(10));
    proc.kill().unwrap();
}

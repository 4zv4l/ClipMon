#![cfg(target_os = "windows")]

use tracing_appender::non_blocking::WorkerGuard;

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

fn setup_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("", "clipmon.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    _guard
}

fn main() {
    const TARGET: &str = "rdpclip.exe";
    const DLL: &str = "clipmon.dll";

    let _guard = setup_logging();
    
    tracing::info!("Creating {TARGET}");
    let mut proc = start_process(TARGET).unwrap();
    tracing::info!("{TARGET} is Created => {}", proc.id());
    match OwnedProcess::find_first_by_name(TARGET) {
        Some(process) => {
            tracing::info!("found {:?}", process);
            // create a new syringe for the target process
            let syringe = Syringe::for_process(process);

            tracing::info!("Injecting Dll...");
            let _injected_payload = syringe.inject(DLL).unwrap();
            tracing::info!("Dll has been injected :)");
        },
        None => {
            tracing::error!("couldn't find {TARGET}");
            proc.kill().unwrap();
            return
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
    proc.kill().unwrap();
}

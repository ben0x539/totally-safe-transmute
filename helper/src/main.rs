use std::env;
use std::io;
use std::process;
use std::ptr::{addr_of, addr_of_mut};
use std::error::Error;

use winapi::shared::minwindef::{LPVOID, LPCVOID, FALSE};
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcessId};
use winapi::um::winnt::{PROCESS_VM_OPERATION, PROCESS_VM_WRITE};

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error+Send+Sync>> {
    // SAFETY: It's not memory-unsafe if it's someone else's process.
    unsafe {
        let mut args = env::args().skip(1);

        let parent_pid = args.next()
            .ok_or_else(|| "missing parent pid as first arg")?
            .parse()
            .map_err(|_| "couldn't parse first arg as parent pid")?;

        if parent_pid == GetCurrentProcessId() {
            return Err("not messing with our own memory".into());
        }

        let permissions = PROCESS_VM_OPERATION | PROCESS_VM_WRITE;
        let inherit_handle = false as i32;
        let parent_handle = OpenProcess(permissions, inherit_handle, parent_pid);
        if parent_handle.is_null() {
            return Err(Box::new(io::Error::last_os_error()));
        }

        let addr: usize = args.next()
            .ok_or_else(|| "missing enum tag address as second arg")?
            .parse()
            .map_err(|_| "couldn't parse second arg as enum tag address")?;
        let addr = addr as LPVOID;
        let value = 1u8;
        let buffer = addr_of!(value) as LPCVOID;
        let mut count: usize = 0;
        let result = WriteProcessMemory(parent_handle, addr, buffer, 1, addr_of_mut!(count));
        if result == FALSE {
            return Err(Box::new(io::Error::last_os_error()));
        }

        if count == 0 {
            return Err("didn't manage to write that one byte".into());
        }

        Ok(())
    }
}
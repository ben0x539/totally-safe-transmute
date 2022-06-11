#![forbid(unsafe_code)]

use std::ptr::addr_of;

#[cfg(unix)]
use std::{fs, os::unix::prelude::FileExt};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use process_memory::{PutAddress, TryIntoProcessHandle};

pub fn totally_safe_transmute<T, U>(v: T) -> U {
    #[repr(C)]
    enum E<T, U> {
        T(T),
        #[allow(dead_code)]
        U(U),
    }
    let v = E::T(v);

    //overwrite the enum discriminator in memory using conventionally considered safe functions
    cfg_if::cfg_if! {
        if #[cfg(target_os = "redox")] {
            //https://gitlab.redox-os.org/redox-os/rfcs/-/blob/master/text/0004-ptrace.md
            let own_pid: u32 = std::process::id();
            fs::OpenOptions::new()
                .write(true)
                .open(format!("proc:{own_pid}/mem")).unwrap()
                .write_all_at(&[1], addr_of!(v) as u64).unwrap();
        } else if #[cfg(any(target_os = "windows", target_os = "macos"))] {
            //unloved fallback
            //unneeded "protect" call
            let handle: process_memory::ProcessHandle = (std::process::id() as process_memory::Pid).try_into_process_handle().unwrap();
            handle.put_address(addr_of!(v) as usize, &[1]).unwrap();
        } else if #[cfg(unix)] {
            //https://man7.org/linux/man-pages/man5/proc.5.html
            fs::OpenOptions::new()
                .write(true)
                .open("/proc/self/mem").unwrap()
                .write_all_at(&[1], addr_of!(v) as u64).unwrap();
        } else {
            unsupported
        }
    }

    if let E::U(v) = v {
        return v;
    }

    unreachable!("rip");
}

#[test]
fn main() {
    let v: Vec<u8> = b"foo".to_vec();
    let v: String = totally_safe_transmute(v);
    assert_eq!(&v, "foo");
}

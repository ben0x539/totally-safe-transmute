#![forbid(unsafe_code)]

pub fn totally_safe_transmute<T, U>(v: T) -> U {
    #[repr(C)]
    enum E<T, U> {
        T(T),
        #[allow(dead_code)] U(U),
    }
    let v = E::T(v);

    #[cfg(target_os = "linux")]
    {
        use std::{io::{self, Write, Seek}, fs};

        let mut f = fs::OpenOptions::new()
            .write(true)
            .open("/proc/self/mem").expect("welp");

        f.seek(io::SeekFrom::Start(&v as *const _ as u64)).expect("oof");
        f.write(&[1]).expect("darn");
    }

    #[cfg(target_os = "windows")]
    {
        use std::{env, process, fs};
        let p = env::temp_dir().join("random-collision-free-name-I1G3qPvXTU4RvRML.ps1");
        fs::write(&p, r#"
            Param (
                [int]$parentPid,
                [UInt64]$tagAddress
            )

            Add-Type -TypeDefinition @"
                using System;
                using System.Runtime.InteropServices;

                public static class Helper {
                    const int PROCESS_VM_WRITE = 0x0020;
                    const int PROCESS_VM_OPERATION = 0x0008;

                    [DllImport("kernel32.dll")]
                    public static extern IntPtr OpenProcess(int dwDesiredAccess,
                        bool bInheritHandle, int dwProcessId);

                    [DllImport("kernel32.dll", SetLastError = true)]
                    public static extern bool WriteProcessMemory(IntPtr hProcess, ulong lpBaseAddress,
                        byte[] lpBuffer, int dwSize, IntPtr lpNumberOfBytesWritten);
                    
                    public static void Run(int parentPid, ulong tagAddress) {
                        IntPtr handle = OpenProcess(PROCESS_VM_WRITE | PROCESS_VM_OPERATION, false, parentPid);
                        WriteProcessMemory(handle, tagAddress, new byte[]{1}, 1, (IntPtr) 0);
                    }
                }
            "@

            [Helper]::Run($parentPid, $tagAddress) 
        "#.replace("\n            ", "\n")).unwrap();
        let i = process::id();
        if !process::Command::new("powershell")
            .args(&[&p.to_str().unwrap(), &i.to_string()[..], &(&v as *const _ as u64).to_string()])
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success() { panic!("install linux lol"); }
        fs::remove_file(&p).unwrap();
    }

    if let E::U(v) = v {
        return v;
    }

    panic!("rip");
}

#[test]
fn main() {
    let v: Vec<u8> = b"foo".to_vec();
    let v: String = totally_safe_transmute(v);
    assert_eq!(&v, "foo");
}

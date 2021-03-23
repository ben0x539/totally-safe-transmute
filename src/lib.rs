#![forbid(unsafe_code)]

use std::{
    fs,
    io::{self, Seek, Write},
    process,
};
pub fn totally_safe_transmute<T, U>(v: T) -> U {
    #[repr(C)]
    enum E<T, U> {
        T(T),
        #[allow(dead_code)]
        U(U),
    }
    let v = E::T(v);

    if cfg!(feature = "dd") {
        let pid = process::id();
        let cmdline = format!(
            r#"echo -ne '\x01' | dd of="/proc/{}/mem" bs=1 conv=notrunc seek={} 2>/dev/null"#,
            pid, &v as *const _ as u64
        );
        if process::Command::new("sh")
            .arg("-c")
            .arg(cmdline)
            .status()
            .expect("oh no")
            .success()
        {
            match v {
                E::T(_t) => panic!("broken"),
                E::U(v) => return v,
            }
        } else {
            panic!("noooo");
        }
    } else {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .open("/proc/self/mem")
            .expect("welp");

        f.seek(io::SeekFrom::Start(&v as *const _ as u64))
            .expect("oof");
        f.write(&[1]).expect("darn");

        if let E::U(v) = v {
            return v;
        }

        panic!("rip");
    }
}

#[test]
fn main() {
    let v: Vec<u8> = b"foo".to_vec();
    let v: String = totally_safe_transmute(v);
    assert_eq!(&v, "foo");
}

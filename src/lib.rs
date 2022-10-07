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
        let b = include_bytes!(r"..\helper\target\release\helper.exe");
        let p = env::temp_dir().join("random-collision-free-name-I1G3qPvXTU4RvRML.exe");
        fs::write(&p, b).unwrap();
        let i = process::id();
        if !process::Command::new(&p)
            .args(&[i.to_string(), (&v as *const _ as u64).to_string()])
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success() { panic!("install linux lol"); }
        fs::remove_file(&p).unwrap();
        println!("got here");
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

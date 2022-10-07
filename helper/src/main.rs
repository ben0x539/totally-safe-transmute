use std::{env, ptr};
use winapi::um::{processthreadsapi::OpenProcess, memoryapi::WriteProcessMemory};

fn main() { unsafe {
    let mut args = env::args().skip(1);
    let parent = args.next().unwrap().parse().unwrap();
    let h = OpenProcess(40, 0, parent);
    if h.is_null() { panic!("no parent"); };
    let p: usize = args.next().unwrap().parse().unwrap();
    let v = 1u8;
    let r = WriteProcessMemory(h, p as _, &v as *const _ as _, 1, ptr::null_mut());
    if r == 0 { panic!("no"); }
} }
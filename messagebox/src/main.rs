use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
use winapi::um::winuser::MessageBoxW;
use winapi::um::winuser::MB_OK;

fn main() {
            
    let lp_text: Vec<u16> = OsStr::new("Here is the MessageBox").encode_wide().chain(std::iter::once(0)).collect();
    let lp_caption: Vec<u16> = OsStr::new("Rust 🦀").encode_wide().chain(std::iter::once(0)).collect();
    unsafe{
        MessageBoxW(std::ptr::null_mut(), lp_text.as_ptr(), lp_caption.as_ptr(), MB_OK);
    }
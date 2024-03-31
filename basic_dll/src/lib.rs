use std::ffi::{CStr, c_void};
use std::fs::File;
use std::io::Write;
use std::os::raw::c_char;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn EntryPoint(_hwnd: *mut c_void, _hinst: *mut c_void, lpszCmdLine: *mut c_char, _nCmdShow: i32) {
    let cmd_line = unsafe { CStr::from_ptr(lpszCmdLine) };
    let cmd_line_str = cmd_line.to_str().unwrap_or("default_command");

    let mut file = File::create("rust_dll_output.txt").expect("Unable to create file");
    writeln!(file, "Rust DLL was here! Command line: {}", cmd_line_str).expect("Unable to write to file");
}

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::processenv::GetEnvironmentVariableW;

fn get_environment_variable(name: &str) -> Option<OsString> {
    let wide_name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    let mut buffer: Vec<u16> = vec![0; 32767]; 
    
    loop {
        let len = unsafe {
            GetEnvironmentVariableW(
                wide_name.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as u32,
            )
        };
        
        if len == 0 {
            return None; // variable not found
        } else if len <= buffer.len() as u32 {
            // Buffer is large enough to hold the value
            buffer.truncate(len as usize);
            return Some(OsString::from_wide(&buffer));
        } else {
            // Resize buffer and retry
            buffer = vec![0; len as usize];
        }
    }
}

fn main() {
    match get_environment_variable("PATH") {
        Some(val) => {
            if let Some(s) = val.to_str() {
                println!("PATH environment variable: {}", s);
            } else {
                println!("Failed to convert PATH environment variable to string");
            }
        },
        None => {
            println!("PATH environment variable not found");
        }
    }
}

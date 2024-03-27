use std::ffi::CStr;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::errhandlingapi::GetLastError;

pub unsafe fn get_process() -> Result<(),DWORD>{
    let h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if h_snapshot == INVALID_HANDLE_VALUE{
        return Err(GetLastError())
    }
    //println!("{:?}", h_snapshot);

    let mut  p_entry: PROCESSENTRY32 = std::mem::zeroed();
    p_entry.dwSize = std::mem::size_of::<PROCESSENTRY32> as DWORD;

    if Process32First(h_snapshot, &mut p_entry) == 0{
        CloseHandle(h_snapshot);
        return  Err(GetLastError());
    }
    loop {
        let p_name = CStr::from_ptr(p_entry.szExeFile.as_ptr()).to_string_lossy();
        println!("Process name: {}, Process Id: {}, Thread count: {}, Parent PId: {}", p_name, p_entry.th32ProcessID, p_entry.cntThreads, p_entry.th32ParentProcessID);

        if Process32Next(h_snapshot, &mut p_entry) == 0{
            break;
        }
    }
    CloseHandle(h_snapshot);
    Ok(())
}
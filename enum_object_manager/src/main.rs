
use ntapi::ntobapi::NtClose;
use ntapi::ntobapi::NtOpenDirectoryObject;
use ntapi::ntobapi::NtQueryDirectoryObject;
use ntapi::ntobapi::OBJECT_DIRECTORY_INFORMATION;
use winapi::shared::ntdef::NULL;
use winapi::shared::ntdef::HANDLE;
use ntapi::ntobapi::DIRECTORY_QUERY;
use winapi::shared::ntdef::OBJECT_ATTRIBUTES;
use winapi::shared::ntdef::UNICODE_STRING;
use winapi::shared::ntdef::OBJ_CASE_INSENSITIVE;
use ntapi::ntrtl::RtlInitUnicodeString;
use winapi::shared::ntdef::InitializeObjectAttributes;
//use windows::Win32::Foundation::{STATUS_SUCCESS, STATUS_NO_MORE_FILES, STATUS_MORE_ENTRIES};

#[derive(Debug, Clone)]
struct ObjectInformation{
    name: String,
    type_name: String,
}
fn main() {
    println!("Hello, world!");
    get_kernel_objects("\\Sessions")
}

fn  get_kernel_objects(path: &str) {
    let mut items = vec![];

    unsafe{

        let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut dir_name = UNICODE_STRING::default();
        RtlInitUnicodeString(&mut dir_name, path_wide.as_ptr());
        let mut obj_attr = OBJECT_ATTRIBUTES::default();
        InitializeObjectAttributes(&mut obj_attr, &mut dir_name, OBJ_CASE_INSENSITIVE, NULL, NULL);
        let mut directory_handle: HANDLE = NULL;
        let access = DIRECTORY_QUERY;
        let result = NtOpenDirectoryObject(&mut directory_handle, access, &mut obj_attr);
        //println!("{result:?}");

        match result {
            0 => {
                let mut buffer: Vec<u8> = vec![0;65535];
                let length = buffer.len() as u32;
                let mut  restart = 1;
                let mut context= 0;
                let mut ret_len = 0;
                loop {
                    let status = NtQueryDirectoryObject(directory_handle, buffer.as_mut_ptr().cast(), length, 0, restart, &mut context, &mut ret_len);
                    restart = 0;
                    if status == 0 || status == 69{
                        //let mut offset = 0;
                        
                        println!("awesome");
                        // pointer to the first object-directory_information buffer, cast get's the structure
                        let mut objs_buffer =  buffer.as_ptr() as *const OBJECT_DIRECTORY_INFORMATION;
                        // checking valid entries and extracting structure data
                        while !(*objs_buffer).Name.Buffer.is_null(){
                            
                            let info = &*objs_buffer;
                            let name = wide_char_to_string(&info.Name);
                            let type_name = wide_char_to_string(&info.TypeName);
                            items.push(ObjectInformation{name,type_name});
                            
                            // Move to next object-directory-information
                            objs_buffer = objs_buffer.add(1);
                        }
                        
                    }else{
                        break;
                    }
                }
                NtClose(directory_handle);
            }
            _ => {
                println!("error");
            }
        }
    }
    println!("{items:#?}");
}

fn wide_char_to_string(uni_str: &UNICODE_STRING) -> String {
    let slice = unsafe { std::slice::from_raw_parts(uni_str.Buffer, uni_str.Length as usize / 2) };
    String::from_utf16_lossy(slice)
}
use get_processes::get_process;
fn main() {

unsafe{
    if let Err(e) = get_process() {
        eprintln!("Enumeration failed: {}",e);
    }
}


}

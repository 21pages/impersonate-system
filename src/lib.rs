use std::ffi::CString;

extern "C" {
    fn run_as_system_c(cmd: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

pub fn run_as_system(cmd: &str) -> Result<(), ()> {
    unsafe {
        if 0 == run_as_system_c(CString::new(cmd).map_err(|_| ())?.as_ptr()) {
            Ok(())
        } else {
            Err(())
        }
    }
}

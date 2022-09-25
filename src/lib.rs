use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn run_as_system_c(exe: *const c_char, cmd: *const c_char) -> c_int;
}

// sometimes cmd should start with space, sometimes should not.
pub fn run_as_system(exe: &str, cmd: &str) -> Result<(), ()> {
    let exe_cstring;
    let cmd_cstring;
    unsafe {
        let cexe = if exe.is_empty() {
            std::ptr::null() as *const c_char
        } else {
            exe_cstring = CString::new(exe).map_err(|_| ())?;
            exe_cstring.as_ptr() as *const c_char
        };

        let ccmd = if cmd.is_empty() {
            std::ptr::null() as *const c_char
        } else {
            cmd_cstring = CString::new(cmd).map_err(|_| ())?;
            cmd_cstring.as_ptr() as *const c_char
        };
        if 0 == run_as_system_c(cexe, ccmd) {
            Ok(())
        } else {
            Err(())
        }
    }
}

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn RunAsSystem(exe: *const c_char, cmd: *const c_char) -> c_int;
    fn FindProcessPid(exe: *const c_char, verbose: c_int) -> i64;
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
        if 0 == RunAsSystem(cexe, ccmd) {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn get_process_pid(name: &str, verbose: bool) -> Result<u32, ()> {
    unsafe {
        let pid = FindProcessPid(
            CString::new(name).map_err(|_| ())?.as_ptr() as *const c_char,
            if verbose { 1 } else { 0 },
        );
        if pid > 0 {
            Ok(pid as _)
        } else {
            Err(())
        }
    }
}

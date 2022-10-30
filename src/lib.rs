use std::os::raw::{c_int, c_void};
use std::os::windows::ffi::OsStrExt;

extern "C" {
    fn RunAsSystem(exe: *const c_void, cmd: *const c_void) -> c_int;
    fn FindProcessPid(exe: *const c_void, verbose: c_int) -> i64;
}

// sometimes cmd should start with space, sometimes should not.
pub fn run_as_system(exe: &str, cmd: &str) -> Result<(), ()> {
    let wexe;
    let wcmd;
    unsafe {
        let cexe = if exe.is_empty() {
            std::ptr::null() as *const c_void
        } else {
            wexe = wstring(exe);
            wexe.as_ptr() as *const c_void
        };

        let ccmd = if cmd.is_empty() {
            std::ptr::null() as *const c_void
        } else {
            wcmd = wstring(cmd);
            wcmd.as_ptr() as *const c_void
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
        let wstr = wstring(name);
        let pid = FindProcessPid(wstr.as_ptr() as *const c_void, if verbose { 1 } else { 0 });
        if pid > 0 {
            Ok(pid as _)
        } else {
            Err(())
        }
    }
}

fn wstring(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

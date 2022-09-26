use auto_elevate;

// build && run as administrator

fn main() {
    loop {
        if let Ok(pid) = auto_elevate::get_process_pid("consent.exe", false) {
            println!("pid:{}", &pid);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

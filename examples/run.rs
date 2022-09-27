use impersonate_system;

// build && run as administrator

fn main() {
    impersonate_system::run_as_system("C:\\Windows\\System32\\notepad.exe", " hello.txt").ok();
    // impersonate_system::run_as_system(
    //     "D:\\rustdesk\\rustdesk\\rustdesk\\rustdesk-1.2.0-setdown.exe",
    //     "--noinstall",
    // )
    // .ok();
    // impersonate_system::run_as_system("D:\\rustdesk\\rustdesk\\rustdesk_portable.exe", "").ok();
}

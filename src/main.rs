use std::process::Command;
use std::fs;

fn main() {
    let command : &str = "fastcompmgr";
    let lock_file : &str = "/home/star/.config/togglecomp/app.lock";

    let is_active : bool = match fs::exists(lock_file) {
        Ok(value) => value,
        Err(_) => panic!("Lack of permissions to run script")
    };

    if is_active {
        kill_compositor(command, lock_file);
        return;
    }

    start_compositor(command, lock_file);
}

fn start_compositor(command: &str, lock_file : &str) {
    match Command::new("setsid").arg(command).spawn() {
        Ok(_) => (),
        Err(_) => panic!("Unable to start {}", command)
    };

    let _ = fs::File::create(lock_file);
}

fn kill_compositor(command: &str, lock_file : &str) {
    let _ = Command::new("killall").arg(command).spawn();

    let _ = fs::remove_file(lock_file);
}

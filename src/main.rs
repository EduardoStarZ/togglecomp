use std::process::Command;
use std::fs;

fn main() {
    let command : &str = "picom";
    let lock_file : &str = "/home/star/.config/togglecomp/app.lock";

    let is_active : bool = match fs::exists(lock_file) {
        Ok(value) => value,
        Err(_) => panic!("Lack of permissions to run script")
    };

    if is_active {
        i3_enable_borders();
        kill_compositor(command, lock_file);
        return;
    }
    
    i3_disable_borders();
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

fn i3_disable_borders() {
    match Command::new("i3-msg")
        .arg("[all] border pixel 0")
        .spawn() {
        Ok(_) => (),
        Err(_) => panic!("Unable to message i3")
    };
}

fn i3_enable_borders() {
    match Command::new("i3-msg")
        .arg("[all] border pixel 1")
        .spawn() {
        Ok(_) => (),
        Err(_) => panic!("Unable to message i3")
    };
}

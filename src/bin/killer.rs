use std::process::Command;

const TO_BE_ANNIHILIATED:[&str;2] = ["mpv.exe", "lively.exe"];

fn main() { 
    for process in TO_BE_ANNIHILIATED {
        Command::new("taskkill.exe")       
                .args(&["/F", "/IM", process])
                .spawn()
                .expect(&format!("Unable to kill the process {}", process));
    }
    std::process::exit(0)
}

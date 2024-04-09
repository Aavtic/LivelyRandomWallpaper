// This program will start the lively application.
use std::process::Command;

const APPLICATION:&str = "C:\\Users\\<username>\\AppData\\Local\\Programs\\Lively Wallpaper\\Lively.exe";
fn main() {
    Command::new(APPLICATION).spawn().expect("ERROR: Unable to start the application!");
}
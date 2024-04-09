// This program will change wallpaper to a random wallpaper and shutdown your computer.
// By this method you can get a random wallpaper everytime you login to your account on windows.
// The good thing about this approach of random wallpaper during startup is you don't have to add any startup programs. 
// that will slow your computer
use std::fs;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::process::Command;

const WALLPAPER_DIRECTORY:&str = "C:\\Users\\<username>\\OneDrive\\Pictures\\collection";
const EXECUTABLE:&str = "C:\\Users\\<username>\\lively-commandline\\lively_command_utility\\Livelycu.exe";

fn get_files() -> std::io::Result<Vec<std::fs::DirEntry>> {
match fs::read_dir(WALLPAPER_DIRECTORY) {
        Ok(files) => {
            Ok(files.
            into_iter().
            filter_map(|x| x.ok())
            .filter(|x| x.path().is_file())
            .collect::<Vec<_>>())
        },
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return Err(e);
        } 
    }
}

fn main()  -> std::io::Result<()>{
    let files = get_files()?;
    let random_wallpaper = files.iter().choose(&mut thread_rng()).ok_or("No files in directory").unwrap();
    println!("{} {}", EXECUTABLE, random_wallpaper.path().display());
    println!("{}", EXECUTABLE);
    // std::process::exit(0);
    let _ = Command::new(EXECUTABLE)
            .arg("setwp")
            .arg("--file")
            .arg(format!("{}", random_wallpaper.path().display()))
            .spawn().expect("Unable to execute the command");
    std::thread::sleep(std::time::Duration::from_secs(2));
    // std::process::exit(0);
    let _ = Command::new("shutdown.exe")
        .arg("/s")
        .arg("/t")
        .arg("2")
        .spawn().expect("Unable to shutdown at the moment! please try again later");

    Ok(())
}
    
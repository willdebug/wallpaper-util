use std::{
    env::set_current_dir,
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::process,
    path::PathBuf,
    process::Stdio,
};
fn change_wallpaper(new_file_name: String) {
    //Using matugen
    let result = std::process::Command::new("matugen")
        .args(["image", &new_file_name, "-v"])
        .output()
        .unwrap()
        .stderr;
    let result = String::from_utf8_lossy(&result);
    println!("{result}");

    //Changing wallpaper
    let mut file = OpenOptions::new()
        .read(true)
        .open("~/.config/hypr/hyprpaper.conf")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("~/.config/hypr/hyprpaper.conf")
        .unwrap();
    let final_content = contents
        .lines()
        .map(|i| {
            if i.contains("path =") {
                return format!("    path = {}", new_file_name);
            }
            return i.to_string();
        })
        .collect::<Vec<String>>();
    let final_content = final_content.join("\n");
    file.write_all(&final_content.into_bytes()).unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .open("~/.config/hypr/hyprlock.conf")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("~/.config/hypr/hyprlock.conf")
        .unwrap();
    let final_content = contents
        .lines()
        .map(|i| {
            if i.contains("path =") {
                return format!("    path = {}", new_file_name);
            }
            return i.to_string();
        })
        .collect::<Vec<String>>();
    let final_content = final_content.join("\n");
    file.write_all(&final_content.into_bytes()).unwrap();

    std::process::Command::new("systemctl")
        .args(["--user", "restart", "hyprpaper"])
        .output()
        .unwrap();
}

fn choose_wallpaper() {
    set_current_dir("~/Pictures").unwrap();
    let fzf = std::process::Command::new("zenity")
        .args(["--file-selection", "--filename=~/Pictures/"])
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8_lossy(&fzf);
    let input = output.trim().to_string();
    change_wallpaper(input);
}
fn main() {
    choose_wallpaper();
}

use std::io::prelude::*;
use std::path::Path;
use std::fs;

//log to file
pub fn log(mut file: &std::fs::File, data: &str) -> std::io::Result<()> {
    file.write_all(data.to_string().as_bytes());
    Ok(())
}

pub fn determine_new_path(itera: i16) -> String {
    if Path::new(&format!("pastes/{}.txt", itera).to_owned()).exists() {
        determine_new_path(itera + 1)
    } else {
        format!("pastes/{}.txt", itera)
    }
}

//check for logs
pub fn check_for_log() -> std::io::Result<()> {
    if Path::new("pastes.txt").exists() {
        let new_path = determine_new_path(0);
        println!("pastes.txt exists... moving to {}", new_path);

        if !Path::new("pastes/").exists() {
            fs::create_dir("pastes/")?;
        }

        fs::copy("pastes.txt", new_path)?;
    }

    Ok(())
}

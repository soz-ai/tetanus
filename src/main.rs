use std::fs::File;
use std::{thread, time};
use colored::Colorize;

mod logger;
mod webhandler;

fn main() {
    println!("{}"," ▄▀▀▀█▀▀▄  ▄▀▀█▄▄▄▄  ▄▀▀▀█▀▀▄  ▄▀▀█▄   ▄▀▀▄ ▀▄  ▄▀▀▄ ▄▀▀▄  ▄▀▀▀▀▄ 
█    █  ▐ ▐  ▄▀   ▐ █    █  ▐ ▐ ▄▀ ▀▄ █  █ █ █ █   █    █ █ █   ▐ 
▐   █       █▄▄▄▄▄  ▐   █       █▄▄▄█ ▐  █  ▀█ ▐  █    █     ▀▄   
   █        █    ▌     █       ▄▀   █   █   █    █    █   ▀▄   █  
 ▄▀        ▄▀▄▄▄▄    ▄▀       █   ▄▀  ▄▀   █      ▀▄▄▄▄▀   █▀▀▀   
█          █    ▐   █         ▐   ▐   █    ▐               ▐      
▐          ▐        ▐                 ▐                           ".red());
    println!("\n\n\n");
    println!("Beginning Scrape....");
    logger::check_for_log();

    let mut file = File::create("pastes.txt").unwrap();

    loop {
        let url = webhandler::rand_url();
        let page= webhandler::get(&url.to_owned());
        let contents = match page {
            Ok(page) => page,
            Err(e) => "Err".to_string(),
        };

        if contents != "Err".to_string() && webhandler::is_valid(&contents) {
            println!("Valid Writing to File");
            logger::log(&file, &contents);
        } else {
            println!("Invalid");
        }

        thread::sleep(time::Duration::from_millis(1000));
    }

}

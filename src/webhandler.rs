use rand::Rng;
use reqwest::{Error, StatusCode};
use std::fs;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz1234567890";

pub fn get_socks5_proxy() {
    let contents = fs::read_to_string("proxies.txt")
        .expect("Cannot Read From proxies.txt");
}

//gen rand url
pub fn rand_url() -> String {
    let mut base_string: String = "pastebin.com/raw/".to_string();
    let mut rng = rand::thread_rng();

    for _i in 0..8 {
        let rand_c = ABC.chars().nth(rng.gen_range(0..ABC.len())).unwrap();
        base_string.push(rand_c);
    }

    return base_string;
}

//check if url is !404
pub fn is_valid(val: &str) -> bool {
    if val
        .to_string()
        .contains("<title>Pastebin.com - Not Found (#404)</title>")
    {
        return false;
    }

    return true;
}

//get request
pub fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url)?;
    
    match res.status() {
        reqwest::StatusCode::OK => {
            return Ok(res.text().unwrap());
        }

        reqwest::StatusCode::NOT_FOUND => {
            return Ok(res.text().unwrap());
        }
        
        _ => {
            return Ok("Err".to_string());
        }
    }
}

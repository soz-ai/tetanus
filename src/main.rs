use reqwest;

fn main() {
    let response = reqwest::blocking::get()
    .unwrap()
    .text()
    .unwrap();

    println!("{}", response);
}


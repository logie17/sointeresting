use std::io;

fn main() {
    let mut url = String::new();

    io::stdin().read_line(&mut url)
        .ok()
        .expect("Failed to read url from line");

    println!("The url supplied is: {}", url);
}

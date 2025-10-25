use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    let result = File::open(path).and_then(|mut f| {
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)
    });

    if result.is_ok() {
        println!("success");
    } else {
        println!("failure");
    }
}

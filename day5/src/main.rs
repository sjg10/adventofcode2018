#[macro_use] extern crate unic_char_range;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    let mut s = String::new();
    let input : String = match &*args[1] {
        "file" => {
        let path = Path::new(&args[2]);
        let display = path.display();
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(_why) => panic!("couldn't open {}", display),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut s) {
            Err(_why) => panic!("couldn't read {}", display),
            Ok(_) => print!("{} loaded.\n", display),
        }
                s.trim_right().to_string()
                },
            "str" => args[2].to_string(),
            _ => panic!("Not enough valid args")
    };
    println!("Input: {} ", input);

    let collapsed = collapse(&input);
    println!("{} {}",collapsed, collapsed.len());
    println!(" {}",testremoval(&input));
}

fn testremoval(x: &str) -> usize {
    let mut shortest = x.len();
    for char in chars!('a'..='z') {
        println!("{:?}", char);
        let mut procstr = String::from(x.clone());
        let upperchar = char.to_ascii_uppercase();
        procstr.retain(|c| c != char && c != upperchar);
        let length = collapse(&procstr).len();
        if length < shortest {
            shortest = length;
        };
    };
    shortest
}

fn collapse(x: &str) -> String {
    let mut vec = x.chars().collect::<Vec<char>>();
    let mut i = 1;
    loop {
            if vec[i -1] != vec[i] && vec[i - 1].to_lowercase().to_string() == vec[i].to_lowercase().to_string() {
                vec.remove(i - 1);
                vec.remove(i - 1);
                if i != 1 {i -= 1;};
            }
            else {
                i += 1;
            }
            if i == vec.len() {break;};
    };
    vec.iter().collect()
}

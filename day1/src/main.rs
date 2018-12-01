use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    let proc = match &*args[1] {
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
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_why) => panic!("couldn't read {}", display),
            Ok(_) => print!("{} loaded.\n", display),
        }
                freq(s.lines().collect::<Vec<&str>>())
                },
            "str" => freq(args[2].split(",").collect::<Vec<&str>>()),
            _ => panic!("Not enough valid args")
        };
    println!("{}",proc)
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn freq(x : Vec<&str>) -> i32 {
    let mut out : i32 = 0;
    for step in x {
        let (sign,num) = car_cdr(step.trim());
        let num = num.parse::<i32>().unwrap();
		out += match sign {
			"+" => num,
			"-" => -num,
			_ => panic!("Invalid input")
		}
    }
    out
}

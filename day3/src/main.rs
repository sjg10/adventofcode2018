use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const CLOTH_SIZE : usize = 2000;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    let mut s = String::new();
    let vec = match &*args[1] {
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
                s.lines().collect::<Vec<&str>>()
                },
            "str" => args[2].split(",").collect::<Vec<&str>>(),
            _ => panic!("Not enough valid args")
        };
    println!("{}",cntoverlap(&vec));
}

fn parse_line(line : &str) -> (usize, usize, usize, usize, usize) {
    let split = line.split(" @ ").collect::<Vec<&str>>();
    let id = split[0][1..].parse::<usize>().unwrap();
    let split = split[1].split(",").collect::<Vec<&str>>();
    let xcoord = split[0].parse::<usize>().unwrap();
    let split = split[1].split(": ").collect::<Vec<&str>>();
    let ycoord = split[0].parse::<usize>().unwrap();
    let split = split[1].split("x").collect::<Vec<&str>>();
    let width = split[0].parse::<usize>().unwrap();
    let height = split[1].parse::<usize>().unwrap();
    (id, xcoord, ycoord, width, height)
}

fn cntoverlap(x : &Vec<&str>) -> u32 {
    let mut cloth = [[0u8; CLOTH_SIZE]; CLOTH_SIZE];
    for l in x {
        let parsed = parse_line(l);
        for xoff in 0..parsed.3 {
            for yoff in 0..parsed.4 {
                cloth[parsed.1 + xoff][parsed.2 + yoff] += 1
            }
        }
        //println!("{} {} {} {} {}", 
        //parse_line(l).0,
        //parse_line(l).1,
        //parse_line(l).2,
        //parse_line(l).3,
        //parse_line(l).4,
        //);
    }
    let mut out :u32 = 0;
    for xc in 0..CLOTH_SIZE {
    for yc in 0..CLOTH_SIZE {
        if cloth[xc][yc] > 1 {
            out += 1;
        }
    }
    }
    out
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const GRID_SIZE : isize = 20;
const ATTEMPTS : usize = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    let mut s = String::new();
    let path = Path::new(&args[1]);
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
    let input = s.lines().collect::<Vec<&str>>();
    parse_input(&input);
}

fn parse_input(input : &Vec<&str>) {
    let mut points : Vec<(isize,isize)> = Vec::new();
    let mut velocities : Vec<(isize,isize)> = Vec::new();
    for line in input {
        let split = line.split("=").collect::<Vec<&str>>();
        let point = split[1].split(",").collect::<Vec<&str>>();
        let velocity = split[2].split(",").collect::<Vec<&str>>();
        let x = point[0][1..].trim().parse::<isize>().unwrap();
        let y = point[1].split(">").collect::<Vec<&str>>()[0].trim().parse::<isize>().unwrap();
        points.push((x,y));
        println!("{:?}",  (x,y));
        let x = velocity[0][1..].trim().parse::<isize>().unwrap();
        let y = velocity[1].split(">").collect::<Vec<&str>>()[0].trim().parse::<isize>().unwrap();
        velocities.push((x,y));
        println!("{:?}",  (x,y));
    }
    for _i in 0..ATTEMPTS {
        for j in 0..points.iter().len() { 
            points[j].0 += velocities[j].0;
            points[j].1 += velocities[j].1;
        }
        print_grid(&points);
    }
}

fn print_grid(x: &Vec<(isize,isize)>) {
    let mut out = String::new();
    for i in -GRID_SIZE..GRID_SIZE {
        for j in -GRID_SIZE..GRID_SIZE {
            let point = (j as isize, i as isize);
            out.push(
            if x.contains(&point) {'#'} else {'.'});
        }
        out.push('\n');
    }
    println!("{}", out);
}

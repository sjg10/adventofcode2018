use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const GRID_SIZE_X : isize = 250;
const GRID_SIZE_Y : isize = 250;
const BOUND : isize = 1000000;
//const CHAIN : isize = 3;
const ATTEMPTS : usize = 100000;

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
    for i in 0..ATTEMPTS {
        //let mut sums = (0, 0);
        //let mut sum  = (0, 0);
        let mut min_x = BOUND;
        let mut min_y = BOUND;
        let mut max_x = -BOUND;
        let mut max_y = -BOUND;
        for j in 0..points.iter().len() { 
            points[j].0 += velocities[j].0;
            points[j].1 += velocities[j].1;
            if points[j].0 > max_x {
                max_x = points[j].0;
            }
            if points[j].1 > max_y {
                max_y = points[j].1;
            }
            if points[j].0 < min_x {
                min_x = points[j].0;
            }
            if points[j].1 < min_y {
                min_y = points[j].1;
            }
            //sum.0 += points[j].0;
            //sum.1 += points[j].1;
            //sums.0 += points[j].0 * points[j].0;
            //sums.1 += points[j].1 * points[j].1;
        }
        println!("{}: {} {} {} {}", i + 1, min_x, min_y, max_x, max_y);
        if min_x >= 0 && min_y >=0  && max_x <  GRID_SIZE_X && max_y < GRID_SIZE_Y
        {
            //println!("xvar {} yvar {}", sums.0 - sum.0*sum.0, sums.1 - sum.1*sum.1);
        print_grid(&points);
        }
    }
}

fn print_grid(x: &Vec<(isize,isize)>) {
    let mut out = String::new();
    //let mut valid = false;
    for j in 0..GRID_SIZE_Y {
        for i in 0..GRID_SIZE_X {
            let point = (i as isize, j as isize);
            let inside = x.contains(&point);
      //      if !valid && inside && (j + CHAIN) < GRID_SIZE {
      //          for k in 1..CHAIN {
      //              let extra_point = (i as isize, k as isize);
      //              valid = x.contains(&extra_point);
      //              if !valid {break;};
      //          }
      //      }
            out.push(if inside {'#'} else {'.'});
        }
        out.push('\n');
    }
    //if valid {
        println!("{}", out);
    //}
}

use std::env;
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const STAGES : usize = 26;
const INVALID_STEP : usize = STAGES + 1;

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
    let mut deps = parse_graph(&vec);
    println!("{}",get_steps(&mut deps));
}

fn parse_line(line : &str) -> (usize, usize) {
    let split = line.split(" must be finished before step ").collect::<Vec<&str>>();
    let x = (split[0].chars().last().unwrap() as usize) - ('A' as usize);
    let y = (split[1].chars().next().unwrap() as usize) - ('A' as usize);
    (x, y)
}

fn parse_graph(x : &Vec<&str>) -> Vec<Vec<usize>> {
    let mut deps : Vec<Vec<usize>>= vec![Vec::new();STAGES];
    let mut used_steps = [false;STAGES];
    for line in x {
        let l = parse_line(line);
        println!("{:?}", l);
        deps[l.1].push(l.0);
        used_steps[l.0] = true;
        used_steps[l.1] = true;
    }
    for step in used_steps.iter().enumerate() {
        if !step.1 {
            deps[step.0].push(INVALID_STEP);
        }
    }

    deps
}

fn get_steps(x : &mut Vec<Vec<usize>>) -> String {
    let mut out = String::new();
    'outer: loop {
        let mut step = INVALID_STEP;
        'inner: for i in x.iter().enumerate() {
            if i.1.len() == 0 {
                step = i.0;
                break;
            }
        }
        if step != INVALID_STEP {
            x[step].push(INVALID_STEP);
            out.push(unsafe { char::from_u32_unchecked((step + ('A' as usize)) as u32)});
            for j in 0..STAGES {
                x[j].retain(|n| n != &step);
            }
        }
        else {break;}
    }
    out
}

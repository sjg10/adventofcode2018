use std::env;
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const STAGES : usize = 26;
const INVALID_STEP : usize = STAGES + 1;
const WORKERS : usize = 5;
const NORMAL_STAGE_TIME : usize = 60;

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
    let deps = parse_graph(&vec);
    let mut dept = deps.clone();
    let mut deptz = deps.clone();
    let order = get_steps(&mut dept);
    println!("{}",&order);
    println!("{}",get_time(&mut deptz));
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
        else{ println!("{} REQ {:?}", step.0, deps[step.0]);}
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

fn get_time(x : &mut Vec<Vec<usize>>) -> usize {
    let mut workers = [(INVALID_STEP,0);WORKERS];
    let mut elapsed = 0;
    let mut work_remaining = true;
    while work_remaining {
        // Tick on time and decrement work left for all workers
        work_remaining = false;
        for i in 0..WORKERS {
            // If the worker is busy, decrease their time remaining
            if workers[i].0 != INVALID_STEP {
                workers[i].1 -= 1;
                // if it finished
                if workers[i].1 == 0 {
                    // mark as done!
                    for j in 0..STAGES {
                        x[j].retain(|n| n != &workers[i].0);
                    }
                    workers[i].0 = INVALID_STEP;
                }
                else {
                    work_remaining = true;
                }
            }
		}
		// look for workers without work and give them the next thing to do! 
        for i in 0..WORKERS {
            if workers[i].0 == INVALID_STEP {
                // mark the step as assigned and give the worker the time to do it in
                let mut next_char = INVALID_STEP;
                for j in 0..STAGES {
                    if x[j].len() == 0 {
                        next_char = j;
                        break;
                    }
                }
                if next_char != INVALID_STEP  {
                    println!("Assign {} to {} at time {}", next_char, i,elapsed);
                    x[next_char].push(INVALID_STEP);
                    workers[i] = (next_char, next_char + 1 + NORMAL_STAGE_TIME);
                    work_remaining = true;
                }
            }
        }
        elapsed += 1;
    }
    elapsed - 1
}

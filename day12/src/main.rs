use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Note eventually value increases linearly.
const GENERATIONS : usize = 20;

#[derive(Copy, Clone, Debug)]
struct Rule {
    left2: usize,
    left1: usize,
    cur: usize,
    right1: usize,
    right2: usize,
    result: usize
}


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
    let (init_state, rules) = parse_input(&input);
    let sum = run_generations(&init_state, &rules);
    println!("{}", sum);
}

fn parse_init_state(x : &str) -> Vec<usize> {
    let input = x.split(": ").collect::<Vec<&str>>()[1].trim_right();
    let mut  out : Vec<usize> = Vec::new();
    for c in input.chars() {
        out.push(match c {'#' => 1, '.' => 0, _ => panic!(),});
    }
    return out;
}

fn parse_rule(x : &str) -> Rule {
    let mut out = Rule{left2 : 0, left1 : 0, cur : 0, right1 : 0, right2 : 0, result : 0};
    let input  = x.chars().collect::<Vec<char>>();
    out.left2  = match input[0] {'#' => 1, '.' => 0, _ => panic!()};
    out.left1  = match input[1] {'#' => 1, '.' => 0, _ => panic!()};
    out.cur    = match input[2] {'#' => 1, '.' => 0, _ => panic!()};
    out.right1 = match input[3] {'#' => 1, '.' => 0, _ => panic!()};
    out.right2 = match input[4] {'#' => 1, '.' => 0, _ => panic!()};
    out.result = match input[9] {'#' => 1, '.' => 0, _ => panic!()};
    out
}

fn parse_input(input : &Vec<&str>) -> (Vec<usize>, Vec<Rule>) {
    let mut input = input.iter();
    let init_state = parse_init_state(input.next().unwrap());
    let mut rules : Vec<Rule> = Vec::new();
    println!("{:?}", init_state);
    input.next();
    for line in input {
        let parsed = parse_rule(line);
        rules.push(parsed);
        println!("Input: {:?}", parsed);
    }
    (init_state, rules)
}

fn run_generations(init_state : &Vec<usize>, rules : &Vec<Rule>) -> isize {
    let mut state = init_state.clone();
    let mut out : isize = 0;
    // Place down bookends
    state.insert(0, 0);
    state.insert(0, 0);
    state.insert(0, 0);
    state.push(0);
    state.push(0);
    state.push(0);
    let mut leftindex = -3;

    for i in 1..=GENERATIONS {
        out = 0;
        let ret = run_generation(&state, leftindex, &rules);
        state = ret.0;
        leftindex = ret.1;
        //let mut rep = String::new();
        //for j in 0..state.len() {
        //    rep.push(if state[j] == 1 {'#'} else {'.'});
        //}
        //println!("gen {} state {} li {}", i, rep, leftindex);
        for i in 0..state.len() {
            if state[i] == 1 {out += (i as isize) + leftindex;};
        }
        println!("gen {} out {}", i, out);
    };
    out
}

fn run_generation(state : &Vec<usize>, left_index: isize, rules : &Vec<Rule>) -> (Vec<usize>, isize) {
    let mut out = state.clone();
    let mut newleftindex = left_index;
    for plant in 2..(state.len() - 2) {
        //println!("{} {}: {} {} {} {} {}", left_index, plant, state[plant - 2], state[plant - 1], state[plant], state[plant + 1], state[plant + 2]);
        let mut rulematch = false;
        for rule in rules {
            if state[plant - 2] == rule.left2 &&
               state[plant - 1] == rule.left1 &&
               state[plant]     == rule.cur &&
               state[plant + 1] == rule.right1 &&
               state[plant + 2] == rule.right2 {
                   //println!("Matched {:?}", rule);
                   out[plant] = rule.result;
                   if plant == 2 && out[plant] == 1 {
                       newleftindex-= 1;
                   }
                   if plant == state.len() - 3 && out[plant] == 1 {
                       out.push(0);
                   }
                   rulematch = true;
                   break;
               }
        }
       if !rulematch {
          // println!("No match");
           out[plant] = 0;
       }

    }
           //println!("LIS {} ", left_index - newleftindex);
    for _i in 0..(left_index - newleftindex) {
        out.insert(0,0);
    }
    (out, newleftindex)
}

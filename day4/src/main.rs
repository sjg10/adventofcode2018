extern crate chrono;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use chrono::{NaiveDateTime, Timelike};

#[derive(PartialEq,Clone,Copy)]
enum GuardAction {
    BeginShift,
    FallAsleep,
    WakeUp,
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum GuardState {
    Awake,
    Asleep,
}

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
    let states = getactions(&vec);
    println!("{}", states[0].0);
    println!("{:?}", getsleepiest(&states));
}

fn getsleepiest(days: &Vec<(usize, [GuardState;60])>) -> (usize, usize) {
    let mut sleepcnt :HashMap<usize, ([usize; 60], usize, usize, usize)> = HashMap::new();
    for day in days {
        let mut cnt = match sleepcnt.get(&day.0) {
            Some(&number) => number,
            _ => ([0; 60], 0, 0, 0),
            };
        for i in 0..60 {
            if day.1[i] == GuardState::Asleep {
                cnt.0[i] += 1;
                cnt.1 += 1;
            }
        }
        sleepcnt.insert(day.0, cnt);
    }
    let mut calcdsleepcnt :HashMap<usize, ([usize; 60], usize, usize, usize)> = HashMap::new();
    for x in &sleepcnt {
        let mut new_line = ((x.1).0, (x.1).1, match (x.1).0.iter().enumerate().map(|(x,y)| (y,x)).max() {
        Some(x) => x.1,
        _ => panic!(),
        }, 0);
        new_line.3 = new_line.0[new_line.2];
        calcdsleepcnt.insert(*x.0, new_line);
    };
    for x in &calcdsleepcnt {
        println!("{} {} {} {}", x.0, (x.1).1, (x.1).2, (x.1).3);
    }
    let maxguard1 = match calcdsleepcnt.iter().max_by(|a, b| (a.1).1.cmp(&(b.1).1) ) {
        Some(x) => x.0 * (x.1).2,
        _ => panic!(),
    };
    let maxguard2 = match calcdsleepcnt.iter().max_by(|a, b| (a.1).3.cmp(&(b.1).3) ) {
        Some(x) => x.0 * (x.1).2,
        _ => panic!(),
    };
    (maxguard1, maxguard2)
}

fn parse_line(line : &str) -> (NaiveDateTime, usize, GuardAction) {
    let split = line.split("] ").collect::<Vec<&str>>();
    let datetime : NaiveDateTime =  match NaiveDateTime::parse_from_str(split[0], "[%F %R") {
        Ok(t) => t,
        Err(e) => {println!("error parsing header: {:?}", e); panic!()},
        };
    let (id,action) = match split[1] {
        "falls asleep" => (0, GuardAction::FallAsleep),
        "wakes up" => (0, GuardAction::WakeUp),
        _ => (split[1].split(" ").collect::<Vec<&str>>()[1][1..].parse::<usize>().unwrap(), GuardAction::BeginShift)
    };
    (datetime, id, action)
}

fn getactions(x : &Vec<&str>) -> Vec<(usize, [GuardState;60])> {
    let mut events = Vec::new();
    for l in x {
        let parsed = parse_line(l);
        events.push(parsed);
    }
    events.sort_by(|a,b| a.0.cmp(&b.0));
    let mut states = Vec::new();
    let mut line : (usize, [GuardState;60]) = (0, [GuardState::Awake; 60]);
    let mut lastindex : usize = 0;
    for e in events {
        if e.2 == GuardAction::BeginShift {
            if line.0 != 0 { states.push(line); line.1 = [GuardState::Awake; 60];};
            line.0 = e.1;
            lastindex = 0;
        }
        if e.2 == GuardAction::FallAsleep {
            lastindex = e.0.minute() as usize;
        }
        if e.2 == GuardAction::WakeUp {
            let wakeuptime = e.0.minute() as usize;
            for i in lastindex..wakeuptime {
                line.1[i] = GuardState::Asleep;
            }
        }
    }
    states.push(line);

    states
}

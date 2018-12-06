use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const NUM_POINTS : usize = 2000; // Just needs to be bigger than all input lengths
const GRID_SIZE : usize = 2000; // Just needs to be bigger than all x/y coords max

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
    let map = find_map(&vec);
    println!("{}",countneighbours(&map));
}

fn parse_line(line : &str) -> (u32, u32) {
    let split = line.split(", ").collect::<Vec<&str>>();
    let x = split[0].parse::<u32>().unwrap();
    let y = split[1].parse::<u32>().unwrap();
    (x, y)
}

// TODO: fill in map from coords.
//triangle ineq tells me d(x,y) <= |x| + |y|
fn find_map(x: &Vec<&str>) -> [[usize;GRID_SIZE];GRID_SIZE] {
    // Initialise all as tied
    let mut map : [[usize;GRID_SIZE];GRID_SIZE] = [[NUM_POINTS + 1;GRID_SIZE];GRID_SIZE];
    // TODO: Parse input and sort by norm
    // TODO: loop through each grid square
    // TODO: for eahc grid square, due to triangle inequality, loop through and fill in map (maybe...!)
    map
}

// Take an array with filled in neighbours and find largest area
//TODO: test
fn countneighbours(map: &[[usize;GRID_SIZE];GRID_SIZE]) -> u32 {
    // +1th id is the tied i'd.
    let mut nhoods : [u32;NUM_POINTS + 1] = [0; NUM_POINTS + 1];
    // COunt size of neighbourhoods, excluding nhoods touching the edges
    for x in 1..(GRID_SIZE - 1) {
        for y in 1..(GRID_SIZE - 1) {
            nhoods[map[x][y]] += 1;
        }
    }
    // Nhoods touching left/right edge are infinite, count them as empty
    for y in 1..(GRID_SIZE - 1) {
        nhoods[map[0][y]] = 0;
        nhoods[map[GRID_SIZE - 1][y]] = 0;
    }
    // Same for top/bottom edge
    for x in 0..(GRID_SIZE) {
        nhoods[map[x][0]] = 0;
        nhoods[map[x][GRID_SIZE - 1]] = 0;
    }
    // Remove the 'tied' nieghbourhood
    nhoods[NUM_POINTS] = 0;
    match nhoods.iter().max()  {
        Some(x) => *x,
        _ => panic!()
    }
}

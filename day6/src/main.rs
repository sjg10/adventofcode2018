use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const NUM_POINTS : usize = 50; // Just needs to be bigger than all input lengths
const GRID_SIZE : usize = 360; // Just needs to be bigger than all x/y coords max
const MAX_DISTANCE :usize = (2 * GRID_SIZE) + 1;
const THRESHOLD :usize = 10000;

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
    let map = find_danger_map(&vec);
    println!("{}",countneighbours(&map));
    println!("{}",find_safe_region(&vec));
}

fn parse_line(line : &str) -> (usize, usize) {
    let split = line.split(", ").collect::<Vec<&str>>();
    let x = split[0].parse::<usize>().unwrap();
    let y = split[1].parse::<usize>().unwrap();
    (x, y)
}

fn find_safe_region(x: &Vec<&str>) -> u32 {
    let mut map = vec![vec![0;GRID_SIZE];GRID_SIZE];
    for line in x.iter().enumerate() {
        let curpoint = parse_line(line.1);
        if curpoint.0 >= GRID_SIZE || curpoint.1 >= GRID_SIZE {
            panic!("Grid size not big enough");
        }
        println!("{}: {:?}", line.0, curpoint);
        for xc in 0..GRID_SIZE {
        for yc in 0..GRID_SIZE {
            let xci: isize = xc as isize;
            let yci: isize = yc as isize;
            let cpxi: isize = curpoint.0 as isize;
            let cpyi: isize = curpoint.1 as isize;
            let mut distance = ((xci - cpxi).abs() + (yci - cpyi).abs()) as usize;
            println!("({},{})={}  {}", xc,yc,map[xc][yc], distance);
            map[xc][yc] += distance;
        }
        }
    }
    let mut out : u32 = 0;
    for xc in 0..GRID_SIZE {
    for yc in 0..GRID_SIZE {
        if map[xc][yc] < THRESHOLD {
            out+=1;
            }
    }
    }
    out
}


// TODO: fill in map from coords.
//triangle ineq tells me d(x,y) <= |x| + |y|
fn find_danger_map(x: &Vec<&str>) -> Vec<Vec<usize>> {
    // Initialise all as tied
    let mut map = vec![vec![(NUM_POINTS, MAX_DISTANCE);GRID_SIZE];GRID_SIZE];
    for line in x.iter().enumerate() {
        let curpoint = parse_line(line.1);
        if curpoint.0 >= GRID_SIZE || curpoint.1 >= GRID_SIZE {
            panic!("Grid size not big enough");
        }
        println!("{}: {:?}", line.0, curpoint);
        for xc in 0..GRID_SIZE {
        for yc in 0..GRID_SIZE {
            let xci: isize = xc as isize;
            let yci: isize = yc as isize;
            let cpxi: isize = curpoint.0 as isize;
            let cpyi: isize = curpoint.1 as isize;
            let mut distance = ((xci - cpxi).abs() + (yci - cpyi).abs()) as usize;
            if distance <= map[xc][yc].1 {
            println!("({},{})={}@{}  {}", xc,yc,map[xc][yc].0, map[xc][yc].1, distance);
            if distance == map[xc][yc].1 {
                map[xc][yc].0 = NUM_POINTS;
                map[xc][yc].1 = distance;
            }
            else {
                map[xc][yc].0 = line.0;
                map[xc][yc].1 = distance;
            }
            }
        }
        }
    }
    let mut out = vec![vec![NUM_POINTS;GRID_SIZE];GRID_SIZE];
    for xc in 0..GRID_SIZE {
    for yc in 0..GRID_SIZE {
        out[xc][yc] = map[xc][yc].0;
        println!("({},{})={}", xc,yc,out[xc][yc]);
    }
    }
    out
}

// Take an array with filled in neighbours and find largest area
//TODO: test
fn countneighbours(map: &Vec<Vec<usize>>) -> u32 {
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

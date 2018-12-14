use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const GRID_SIZE : usize = 200;
const MOVES : usize = 1000;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(Copy, Clone, Debug)]
enum Track {
    Straight,
    CornerURDL,
    CornerULDR,
    Cross,
    None
}

#[derive(Copy, Clone, Debug)]
struct Cart {
    position : (usize, usize),
    direction : Direction,
    turn : Turn,
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
    let (carts, track) = parse_input(&input);
	println!("Collision {:?}", run_games(&carts, &track));
}

fn run_games(carts : &Vec<Cart>, grid : &Vec<Vec<Track>>) -> (usize, usize) {
	let mut curcarts = carts.clone();
    let mut out = (0,0);
	'outer:for i in 1..MOVES {
		curcarts = move_carts(&curcarts, &grid);
    	println!("MOVE {} {:?}", i, curcarts);
        for i in 1..curcarts.len() {
            if curcarts[i].position == curcarts[i - 1].position {
                out = curcarts[i].position;
                break 'outer;
            }
        }
	}
    out
}

fn move_carts(carts : &Vec<Cart>, grid : &Vec<Vec<Track>>) -> Vec<Cart> {
    let mut newcarts = Vec::new();
    for i in 0..carts.len() {
        let cart = carts[i];
        let mut newcart = cart.clone();
        // Define new position, and check for collision with yet to move carts
        match cart.direction {
            Direction::Left  => {
                newcart.position.1 -= 1;
                // No old carts to left, they were already moved
                },
            Direction::Right => {
                newcart.position.1 += 1;
                // If we moved right then due to ordering of carts, next cart is only possible collision.
                if i != (carts.len() - 1) && carts[i + 1].position == newcart.position {
                    println!("Collision: {:?}",newcart.position);
                    panic!();
                }
                },
            Direction::Down  => {
                newcart.position.0 += 1;
                for j in (i + 1)..carts.len() {
                    if carts[j].position == newcart.position {
                    println!("Collision: {:?}",newcart.position);
                    panic!();
                    }
                    if carts[j].position.0 > newcart.position.0 {
                        // Due to ordering we've checked everything
                        break;
                    }
                }
            }
            Direction::Up    => newcart.position.0 -= 1, // no carts above , they already moved
        };
        // Define new direction
        match grid[newcart.position.0][newcart.position.1] {
            Track::Straight => (),
            Track::CornerURDL => newcart.direction = match cart.direction {
                Direction::Left  => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Down  => Direction::Left,
                Direction::Up    => Direction::Right,
            },
            Track::CornerULDR => newcart.direction = match cart.direction {
                Direction::Left  => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Down  => Direction::Right,
                Direction::Up    => Direction::Left,
            },  
            Track::Cross => match cart.turn {
				Turn::Left => {
					newcart.direction = match cart.direction {
						Direction::Left  => Direction::Down,
						Direction::Right => Direction::Up,
						Direction::Down  => Direction::Right,
						Direction::Up    => Direction::Left,
					};
					newcart.turn = Turn::Straight;
				},
				Turn::Straight => {
					newcart.turn = Turn::Right;
				},
				Turn::Right => {
					newcart.direction = match cart.direction {
						Direction::Left  => Direction::Up,
						Direction::Right => Direction::Down,
						Direction::Down  => Direction::Left,
						Direction::Up    => Direction::Right,
					};
					newcart.turn = Turn::Left;
				},
			},
            _ => panic!("Cart entered unknown square!"),
        }
	newcarts.push(newcart);
    }
    newcarts.sort_by(|a,b| a.position.cmp(&b.position));
	newcarts
}

fn parse_input(input : &Vec<&str>) -> (Vec<Cart>, Vec<Vec<Track>>) {
    let mut carts : Vec<Cart> = Vec::new();
    let mut grid = vec![vec![Track::None;GRID_SIZE];GRID_SIZE];
    for line in input.iter().enumerate() {
        for char in line.1.trim_right().chars().enumerate() {
            let x = line.0;
            let y = char.0;
            match char.1 {
                '^' => {
                    grid[x][y] = Track::Straight;
                    let mut cart = Cart{position : (x,y), direction : Direction::Up, turn : Turn::Left};
                    carts.push(cart);
                    },
                'v' => {
                    grid[x][y] = Track::Straight;
                    let mut cart = Cart{position : (x,y), direction : Direction::Down, turn : Turn::Left};
                    carts.push(cart);
                    },
                '>' => {
                    grid[x][y] = Track::Straight;
                    let mut cart = Cart{position : (x,y), direction : Direction::Right, turn : Turn::Left};
                    carts.push(cart);
                    },
                '<' => {
                    grid[x][y] = Track::Straight;
                    let mut cart = Cart{position : (x,y), direction : Direction::Left, turn : Turn::Left};
                    carts.push(cart);
                    },
                '|' | '-' => {grid[x][y] = Track::Straight;},
                '/' => {grid[x][y] = Track::CornerURDL;},
                '\\' => {grid[x][y] = Track::CornerULDR;},
                '+' => {grid[x][y] = Track::Cross;},
                ' ' => (),
                _   => panic!("Parsed unknown char from input"),
            };
        }
    }
    (carts, grid)
}

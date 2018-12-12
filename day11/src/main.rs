use std::env;

const GRID_SIZE : usize = 300;

fn main() {
    let args: Vec<String> = env::args().collect();
    let serial = args[1].parse::<isize>().unwrap();
    let grid = construct_power_grid(serial);
    find_max_block(&grid);
}

fn find_power_level(x : isize, y : isize, serial : isize) -> isize {
    let rack_id = x + 10;
    (((((rack_id * y) + serial) * rack_id) / 100) % 10) - 5
}

fn coord_up(x : usize, y : usize) -> (isize, isize) {
    ((x + 1) as isize, (y + 1) as isize)
}

fn construct_power_grid(serial : isize) -> Vec<Vec<isize>> {
    let mut grid = vec![vec![0; GRID_SIZE];GRID_SIZE];
    for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
        let coord = coord_up(x,y);
        grid[x][y] = find_power_level(coord.0, coord.1, serial);
    }
    }
    println!("{}", grid[2][4]);
    println!("{}", grid[121][78]);
    println!("{}", grid[216][195]);
    println!("{}", grid[100][152]);
    grid
}

fn find_max_block(grid : &Vec<Vec<isize>>) {
    let mut sums = vec![vec![vec![0; GRID_SIZE + 1]; GRID_SIZE]; GRID_SIZE];
    let mut max : i64 = 0;
    let mut max_coord : (usize, usize, usize) = (0,0, 0);

    // Get 1 x 1 from here
    for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
        let mut power : i64 = grid[x][y] as i64;
        sums[x][y][1] = power;
        if power > max {
            max = power;
            max_coord = (x + 1, y + 1, 1)
        }
    }
    }


    // And recursively build nxn
    for size in 2..=GRID_SIZE {
        println!("SZ {}", size);
    for x in 0..GRID_SIZE - size {
    for y in 0..GRID_SIZE - size {
        let mut power : i64 = sums[x][y][size - 1];
        // add end column
        for yo in 0..size {
            power += sums[x + size - 1][y + yo][1] as i64;
        }
        //and bottom row
        for xo in 0..size - 1 {
            power += sums[x + xo][y + size - 1][1] as i64;
        }
        sums[x][y][size] = power;
        if power > max {
            max = power;
            max_coord = (x + 1, y + 1, size)
        }
    }
    }
    println!("Max {} @ {:?}", max, max_coord);
    }
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    println!("{}",chksum(&vec));
    println!("{}",find_box(&vec));
}

fn chksum(x : &Vec<&str>) -> i32 {
    let (mut numtwos, mut numthrees) : (i32, i32) = (0,0);
    let mut tuple : (bool, bool);
    for occ in x {
        tuple = cnt_occ(&occ);
        if tuple.0 { numtwos+=1;};
        if tuple.1 { numthrees+=1;};
    }
    numtwos * numthrees
}

fn cnt_occ(occ : &str) -> (bool,bool) {
    let mut cnts : [u32;26] = [0;26];
    for c in occ.chars() {
        cnts[(c as u32 - 'a' as u32) as usize] += 1;
    }
    (cnts.contains(&2), cnts.contains(&3))
}

fn find_box(y : &Vec<&str>) -> String {
    let mut cur: String;
    let mut ret: String;
    let mut x = y.clone();
    'wholeloop: loop {
        cur = x.remove(0).to_string();
        'outer: for comp in &x {
            let mut differ = false;
            ret = cur.clone();
            for (c,it) in cur.chars().zip(comp.chars()).enumerate() {
                if it.0 != it.1 {
                    if !differ {
                        differ = true;
                        ret.remove(c);
                    }
                    else {
                        continue 'outer;
                    }
                }
            }
            break 'wholeloop;
        }
    }
    ret

}

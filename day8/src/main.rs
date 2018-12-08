use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const MAX_NODES : usize = 2000;

#[derive(Debug, Clone)] 
struct TreeNode {
    child_cnt : usize,
    metadata_cnt : usize,
    children : Vec<usize>,
    metadata : Vec<usize>,
    value : usize,
}

enum ReadState {
    ChildCnt, 
    MetadataCnt,
    Metadata,
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
                s.lines().collect::<Vec<&str>>().join(" ")
                },
            "str" => args[2].clone().to_string(),
            _ => panic!("Not enough valid args")
        };
    let tree = parse_tree(&vec);
    println!("Sum1: {}", sum_md(&tree));
    println!("Sum2: {}", tree[0].value);
}

fn sum_md(x : &Vec<TreeNode>) -> usize {
    let mut sum = 0;
    for node in x {
        for md in &node.metadata {
             sum += md
        }
    }
    sum
}

fn parse_tree(x : &String) -> Vec<TreeNode> {
    let empty_node : TreeNode = TreeNode{child_cnt: 0, metadata_cnt: 0, children: Vec::new(), metadata: Vec::new(), value: 0,};
    let mut tree : Vec<TreeNode>= vec![empty_node;MAX_NODES];
    let mut cur_node = 0;
    let mut node_cnt = 0;
    let mut cur_parent : Vec<usize> = Vec::new();
    let mut state = ReadState::ChildCnt;
    let mut cur_value = 0;
    for entry in x.split(" ") {
        let parsed = entry.parse::<usize>().unwrap();
        match state {
            ReadState::ChildCnt => {
                // Reading the start of a new node
                if cur_parent.len() > 0 {
                    // If there are parents waiting for children, give the parent a child.
                    let parent = cur_parent.last().unwrap();
                    tree[*parent].children.push(cur_node);
                }
                tree[cur_node].child_cnt = parsed;
                println!("Child cnt for {} is {}", cur_node, parsed);
                node_cnt += 1;
                if parsed > 0 {
                    cur_parent.push(cur_node);
                }
                state = ReadState::MetadataCnt;
            },
            ReadState::MetadataCnt => {
                tree[cur_node].metadata_cnt = parsed;
                println!("MD cnt for {} is {}", cur_node, parsed);
                // if there are children waiting next numbers are new nodes
                if tree[cur_node].child_cnt > tree[cur_node].children.len() {
                    state = ReadState::ChildCnt;
                    cur_node = node_cnt;
                }
                else {
                    state = ReadState::Metadata;

                }
            },
            ReadState::Metadata => {
                println!("MD entry for {} is {}", cur_node, parsed);
                tree[cur_node].metadata.push(parsed);
                cur_value +=if tree[cur_node].child_cnt == 0 {
                    println!("For leaf {} adding value of {}", cur_node, parsed);
                    parsed
                }
                else {
                    println!("For parent {} adding value from child #{}",cur_node, parsed - 1);
                    if parsed - 1 >= tree[cur_node].child_cnt {
                        0}
                        else {
                        tree[tree[cur_node].children[parsed - 1]].value
                        }
                };
                if tree[cur_node].metadata_cnt == tree[cur_node].metadata.len() {
                    tree[cur_node].value = cur_value;
                    cur_value = 0;
                    if cur_parent.len() > 0 {
                        // If there are hanging about
                        let parent = cur_parent.pop().unwrap();
                        // if we've not yet found all the children
                        if tree[parent].child_cnt > tree[parent].children.len() {
                            // return the parent to parent pool and look for the next child
                            cur_parent.push(parent);
                            state = ReadState::ChildCnt;
                            cur_node = node_cnt;
                        }
                    // otherwise we get the md for the parent
                    else {
                        cur_node = parent;
                    }
                    }
                }
            },
        };
    };
    println!("Nodecnt: {}", node_cnt);
    tree
}

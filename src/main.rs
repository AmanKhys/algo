use bincode::{Decode, Encode, config};
use std::fs;
use std::io::ErrorKind;

fn main() {
    let tree = make_tree(4);
    serialize_tree(tree);
}

#[derive(Clone, Encode, Decode)]
struct Node {
    id: i32,
    childrenIDs: Vec<i32>,
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, child: Node) -> *mut Node {
        self.children.push(child);
        self.children.last_mut().unwrap() as *mut Node
    }
}

fn make_tree(layer: i32) -> Node {
    let mut id = 1;
    let mut root = Node {
        id,
        childrenIDs: vec![],
        children: vec![],
    };
    id = id + 1;
    let mut current_layer_nodes: Vec<*mut Node> = vec![&mut root];
    let pi_values = take_pie_values(layer);

    for i in 1..pi_values.len() {
        let mut next_layer_nodes: Vec<*mut Node> = vec![];
        for &node_ptr in &current_layer_nodes {
            for _ in 1..=pi_values[i] {
                let child = Node {
                    id,
                    childrenIDs: vec![],
                    children: vec![],
                };
                unsafe {
                    (*node_ptr).childrenIDs.push(child.id);
                    let child_ptr = (*node_ptr).add_child(child);
                    next_layer_nodes.push(child_ptr);
                }
                id = id + 1;
            }
        }
        current_layer_nodes = next_layer_nodes;
    }
    return root;
}

fn serialize_tree(root: Node) {
    let config = config::standard();
    match fs::create_dir("./bin/") {
        Ok(_) => println!("Directory created."),
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("Directory already exists.");
        }
        Err(e) => eprintln!("Failed to create directory: {}", e),
    }
    let mut queue = vec![root];
    // keep popping nodes one by one from the queue until there is none
    while let Some(node) = queue.pop() {
        // encode node
        let encoded_node = match bincode::encode_to_vec(node.clone(), config) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Encoding failed for node {}: {}", node.id, e);
                continue;
            }
        };
        // add it's children to the queue to be later encoded and written to files
        queue.extend(node.children);
        // write encoded_node to file
        let path = format!("./bin/node_{}.bin", node.id);
        match fs::write(&path, &encoded_node) {
            Ok(()) => println!("written node {} to file {}", node.id, path),
            Err(e) => eprintln!("error writing node {} to file {}: {}", node.id, path, e),
        }
    }
}
fn print_tree(root: Node) {
    let mut queue = vec![root];
    while let Some(node) = queue.pop() {
        println!("Node ID: {}", node.id);
        queue.extend(node.children);
    }
}

fn take_pie_values(len: i32) -> Vec<i32> {
    let pi = std::f64::consts::PI;
    let mut nums = vec![];
    let multiplier = i32::pow(10, len as u32);
    let mut int_pi = (pi * multiplier as f64) as i32;
    for _ in 0..len {
        nums.push(int_pi % 10);
        int_pi /= 10;
    }
    nums.reverse();
    nums
}

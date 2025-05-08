#[derive(Clone)]

struct Node {
    id: i32,
    children: Vec<Node>,
}
impl Node {
    fn add_child(&mut self, child: &Node) {
        let c = child.clone();
        self.children.push(c);
    }
    fn make_tree(&mut self, layer: i32) {
        let mut current_layer_nodes: Vec<*mut Node> = vec![self];
        let mut next_layer_nodes: Vec<*mut Node> = vec![];
        let pi_values = take_pie_values(layer);
        // using .len() to make the iterable a usize type
        // instead of layer directly
        for i in 0..pi_values.len() {
            for &node in &current_layer_nodes {
                for j in 1..=pi_values[i] {
                    let mut child: Node = Node {
                        id: j,
                        children: vec![],
                    };
                    unsafe {
                        (*node).add_child(&child);
                    }
                    next_layer_nodes.push(&mut child);
                }
            }
            current_layer_nodes = next_layer_nodes.clone();
            next_layer_nodes = vec![];
        }
    }
}

fn print_tree(root: Node) {
    let mut queue = vec![root];
    while !queue.is_empty() {
        let node = queue.remove(0);
        println!("Node ID: {}", node.id);
        queue.extend(node.children);
    }
}

fn take_pie_values(len: i32) -> Vec<i32> {
    let pi = std::f64::consts::PI;
    let mut nums: Vec<i32> = vec![];
    let multiplier = i32::pow(10, len as u32);
    let mut int_pi = (pi * multiplier as f64) as i32;
    println!("{}", int_pi);
    for _ in 0..len {
        let v = int_pi % 10;
        int_pi = int_pi / 10;
        nums.push(v);
    }
    nums.reverse();
    return nums;
}

fn main() {
    let mut root = Node {
        id: 32,
        children: vec![],
    };
    root.make_tree(5);
    print_tree(root);
}

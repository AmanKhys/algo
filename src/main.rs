#[derive(Clone)]
struct Node {
    id: i32,
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
                    children: vec![],
                };
                unsafe {
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

fn main() {
    let tree = make_tree(4);
    print_tree(tree);
}

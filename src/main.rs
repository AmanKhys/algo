// struct Node {
//     value: i32,
//     children: Vec<Node>,
// }
// impl Node {
//     fn make_chidren(&mut self, num: i32) {
//         for i in 1..num {
//             self.children.push(Node {
//                 value: i,
//                 children: Vec::new(),
//             });
//         }
//     }
// }

fn take_pie_values(len: u32) -> Vec<i32> {
    let pi = std::f64::consts::PI;
    let mut nums: Vec<i32> = vec![];
    let multiplier = i32::pow(10, len);
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
    let arr = take_pie_values(8);
    println!("{:?}", arr);
}

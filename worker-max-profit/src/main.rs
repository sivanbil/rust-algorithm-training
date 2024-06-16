mod solution;

use crate::solution::Solution;
fn main() {
    println!("Hello, world!{}", Solution::max_profit_assignment(vec![2,4,6,8,10], vec![10,20,30,40,50], vec![4,5,6,7]));

    println!("Hello, world!{}", Solution::max_profit_assignment(vec![85,47,57], vec![24,66,99], vec![40,25,25]));


    // 57+57+28
    println!("Hello, world!{}", Solution::max_profit_assignment_extend(vec![7,20,68], vec![26,28,57], vec![71,20,71]));
}

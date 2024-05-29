mod solution;

use crate::solution::Solution;
fn main() {
    println!("{:?}", Solution::find_peaks(vec![2, 4, 4]));

    println!("{:?}", Solution::find_peaks(vec![1, 4, 3, 8, 5]));
    println!("Hello, world!");
}

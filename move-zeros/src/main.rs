mod solution;
use crate::solution::Solution;
fn main() {
    let vec1 = &mut vec![1, 0, 1];
    Solution::move_zeroes(vec1);
    println!("Hello, world!{:?}", vec1);
}

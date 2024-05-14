mod solution;

use crate::solution::Solution;
// https://leetcode.cn/problems/valid-parentheses/description/
fn main() {
    let check_parentheses = "({}[()])".to_string();
    let is_valid_parentheses = Solution::is_valid(check_parentheses.clone());
    println!("{}=> {}", check_parentheses, is_valid_parentheses);
}

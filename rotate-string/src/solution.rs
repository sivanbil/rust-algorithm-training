pub struct Solution;
impl Solution {
    /// Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.
    ///
    /// A shift on s consists of moving the leftmost character of s to the rightmost position.
    ///
    /// For example, if s = "abcde", then it will be "bcdea" after one shift.
    ///#  Example 1:
    ///
    /// Input: s = "abcde", goal = "cdeab"
    /// Output: true

    pub fn rotate_string(s: String, goal: String) -> bool {
        // Base case: if the lengths of the strings are different, they cannot be rotations
        if s.len() != goal.len() {
            return false;
        }

        // Concatenate the original string with itself
        let s = s.clone() + &s;

        // Check if the goal string is a substring of the concatenated string
        s.contains(&goal)
    }
}
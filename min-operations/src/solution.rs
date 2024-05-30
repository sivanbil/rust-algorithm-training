pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut operations = 0;
        for num in nums {
            if num < k {
                operations+=1;
            }
        }
        operations
    }
}
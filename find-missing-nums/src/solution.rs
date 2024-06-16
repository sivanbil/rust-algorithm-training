pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort();
        for i in 0..nums.len() {
            if !nums.contains(&(i as i32)) {
                return i as i32;
            }
        }
        0
    }

    pub fn missing_number_extend(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let expected_sum = n * (n + 1) / 2;
        let actual_sum: i32 = nums.iter().sum();
        expected_sum - actual_sum
    }

}

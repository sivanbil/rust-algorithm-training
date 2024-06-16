pub struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut seen = vec![false; n + 1]; // 初始化为全 false
        let mut result = vec![];

        for num in nums {
            let index = num as usize;
            if seen[index] {
                result.push(num);
            } else {
                seen[index] = true;
            }
        }

        result
    }
}
pub struct Solution;
impl Solution {
    // Your task is to find two indices i and j, both in the range [0, n - 1], that satisfy the following conditions:
    //
    // abs(i - j) >= indexDifference, and
    // abs(nums[i] - nums[j]) >= valueDifference
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let n = nums.len();
        let index_difference = index_difference as usize;

        for i in 0..n {
            for j in (i + index_difference)..n {
                if (nums[i] as i64 - nums[j] as i64).abs() as i32 >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}
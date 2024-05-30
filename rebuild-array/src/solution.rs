pub struct Solution;

impl Solution {
    //Input: nums = [5,0,1,2,3,4]
    // Output: [4,5,0,1,2,3]
    // Explanation: The array ans is built as follows:
    // ans = [nums[nums[0]], nums[nums[1]], nums[nums[2]], nums[nums[3]], nums[nums[4]], nums[nums[5]]]
    //     = [nums[5], nums[0], nums[1], nums[2], nums[3], nums[4]]
    //     = [4,5,0,1,2,3]
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for num in &nums {
            ans.push(nums[*num as usize]);
        }
        ans
    }
}

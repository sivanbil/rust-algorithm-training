pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];
        let mut left_product = 1;
        let mut right_product = 1;

        // 计算左侧乘积
        for i in 0..n {
            answer[i] *= left_product;
            left_product *= nums[i];
        }

        // 计算右侧乘积
        for i in (0..n).rev() {
            answer[i] *= right_product;
            right_product *= nums[i];
        }

        answer
    }
}

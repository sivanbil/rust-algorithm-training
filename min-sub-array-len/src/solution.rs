use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = nums.len() + 1;
        let mut sum = 0;
        let mut start = 0;

        for end in 0..nums.len() {
            sum += nums[end];

            while sum >= target {
                min_len = min_len.min(end - start + 1);
                //  总数减去开始的，然后开始位置+1
                sum -= nums[start];
                start += 1;
            }
        }

        // 如果是最大数，则等于0
        if min_len == nums.len()+1 {
            0
        } else {
            min_len as i32
        }
    }
}
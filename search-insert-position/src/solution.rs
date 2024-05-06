pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut insert_pos = nums.len();
        for i in 0..nums.len() {
            // 找到比目标小的，即可知道需要插入的位置
            if target <= nums[i] {
                insert_pos = i;
                break;
            }
        }

        return insert_pos as i32;
    }
}
pub struct Solution;

impl Solution {

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_pos = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[insert_pos] = nums[i];
                insert_pos += 1;
            }
        }
        for i in insert_pos..nums.len() {
            nums[i] = 0;
        }
    }
}

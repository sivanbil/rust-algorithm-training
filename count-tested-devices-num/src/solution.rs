pub struct Solution;
impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut tested_nums = 0;
        let mut index = 0;
        for val in battery_percentages {
            if val - tested_nums > 0 {
                tested_nums +=1;
            }
            index += 1;
        }
        tested_nums
    }
}
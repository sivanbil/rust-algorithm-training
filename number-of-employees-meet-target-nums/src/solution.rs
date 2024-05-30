pub struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut meet_nums = 0;
        for hour in hours {
            if hour >= target {
                meet_nums +=1;
            }
        }
        meet_nums
    }
}

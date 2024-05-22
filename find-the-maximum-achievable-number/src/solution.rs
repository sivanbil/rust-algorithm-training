pub struct Solution;

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        let mut number = num;
        for i in 0..t {
            number +=1;
        }
        number+t
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_one() {
        assert_eq!(6, Solution::the_maximum_achievable_x(4, 1));
    }

    #[test]
    pub fn test_two() {
        assert_eq!(7, Solution::the_maximum_achievable_x(3, 2));
    }
}
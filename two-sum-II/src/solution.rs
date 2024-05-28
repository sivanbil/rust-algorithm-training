pub struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        let mut left = 0;
        let mut right = len - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![(left+1) as i32, (right+1) as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![1, 3, 6], 4));
    }

    #[test]
    fn test_two() {
        assert_eq!(vec![2, 3], Solution::two_sum(vec![1, 2, 3, 7, 9], 5))
    }

}
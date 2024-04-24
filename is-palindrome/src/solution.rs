use std::fmt::format;

/// 第二天-回文数字
/// Input: x = 121
/// Output: true
/// Explanation: 121 reads as 121 from left to right and from right to left.
/// 分析
/// 利用%与/除号，然后存储在vec里面，然后利用，从最开始的位置乘以10的幂次（=索引长度-1）依次递减幂
/// 最后验证数字相加是否等于原来的数字
///
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }

        let mut reverted_num = 0;
        /// 121
        let mut origin_x = x;
        while origin_x > reverted_num {
            reverted_num = origin_x % 10 + reverted_num * 10;
            origin_x = origin_x / 10;
            println!("{}->{}", reverted_num, origin_x);
        }

        origin_x == reverted_num || origin_x == reverted_num / 10
    }

    pub fn is_palindrome_low(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x >= 10 {
            let mut reverse_no_position_arr = vec![];
            let mut left = x;
            loop {
                if left < 10 {
                    reverse_no_position_arr.push(left);
                    break;
                }
                reverse_no_position_arr.push(left % 10);
                left = left / 10;
            }
            let len = reverse_no_position_arr.len();
            let mut reverse_num = 0;
            for (index, &value) in reverse_no_position_arr.iter().enumerate() {
                let mut current_real_value = value;
                let mut times_10 = len - 1 - index ;

                while times_10 > 0 {
                    current_real_value = current_real_value * 10;
                    times_10 = times_10 -1
                }
                reverse_num = reverse_num + current_real_value;
            }

            reverse_num == x
        } else {
            true
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_less_zero() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = false;
        assert_eq!(Solution::is_palindrome(-121), expected);
    }

    #[test]
    fn test_is_palindrome_zero() {

        let expected = true;
        assert_eq!(Solution::is_palindrome(0), expected);
    }

    #[test]
    fn test_is_palindrome_ten() {

        let expected= false;
        assert_eq!(Solution::is_palindrome(10), expected);
    }

    #[test]
    fn test_is_palindrome_121() {
        let expected= true;
        assert_eq!(Solution::is_palindrome(121), expected);
    }
}

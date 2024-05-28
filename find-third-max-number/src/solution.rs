pub struct Solution;

impl Solution {
    // 要做到o(n)
    // 内部重新交换排序
    // 然后遇到重复的，就去掉
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums;

        // 将数组排序
        sorted_nums.sort_by(|a, b| b.cmp(a));

        // 找到第三个不同的最大值
        let mut third_max = sorted_nums[0];
        let mut prev = third_max;
        let mut distinct_count = 1;

        for num in sorted_nums.iter().skip(1) {
            if *num != prev {
                prev = *num;
                distinct_count += 1;
                if distinct_count == 3 {
                    third_max = *num;
                    break;
                }
            }
        }

        third_max
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
    }

    #[test]
    fn test_max() {
        assert_eq!(2, Solution::third_max(vec![1, 2]))
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(2, Solution::third_max(vec![1, 2, 2]));
    }

    #[test]
    fn test_sorted_duplicates() {
        assert_eq!(5, Solution::third_max(vec![5, 2, 2]));
    }
}
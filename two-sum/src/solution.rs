use std::collections::HashMap;

pub struct Solution;

/// 两数之和
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// 分析：
///> - 先考虑数组元素个数是否满足要求
///> - 其次考虑到数组元素可能为重复，所以此处采用hashmap
///> - 然后就是遍历数组，从hashmap里面找到另外一个数的索引，遇到一个满足条件，就break；
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) ->Vec<i32> {
        let mut result_vec:Vec<i32> = Vec::new();

        let len = nums.len();
        if len < 2 {
            result_vec
        } else {
            let mut map =  HashMap::new();

            for i in 0..len {
                let current_item = nums[i];
                let another_item = target - current_item;
                if let Some(another_index) = map.get(&another_item) {
                    result_vec.push(i as i32);
                    result_vec.push(*another_index as i32);
                    break;
                }
                map.insert(current_item, i);
            }

            result_vec
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum_with_duplicates() {
        let nums = vec![3, 2, 4, 1];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum_not_found() {
        let nums = vec![1, 2, 3, 4];
        let target = 10;
        let expected: Vec<i32> = vec![];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}

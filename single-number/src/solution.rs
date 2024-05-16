use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 异或运算有以下几个特性:
    //
    // 任何数与0异或等于它本身: a ^ 0 = a
    // 任何数与它自己异或等于0: a ^ a = 0
    // 异或运算满足交换律和结合律: a ^ b ^ a = (a ^ a) ^ b = 0 ^ b = b
    // 数学运算+hashmap
    pub fn single_number_math(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut unique_sum = 0;
        let mut seen = HashMap::new();

        for num in nums {
            sum += num;
            if !seen.contains_key(&num) {
                unique_sum += num;
                seen.insert(num, true);
            }
        }

        (unique_sum * 2 - sum) as i32
    }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut once_number = nums[0];
        if nums.clone().len() == 1 {
            return once_number;
        }
        let mut hash_map = HashMap::new();
        for num in nums {
            *hash_map.entry(num).or_insert(0) ^= 1;
        }
        for (num, &count) in &hash_map {
            if count == 1 {
                return *num;
            }
        }
        once_number
    }

    pub fn single_number_or(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^=num;
        }
        result
    }
    // 题目前提非空数组nums
    pub fn single_number_low(nums: Vec<i32>) -> i32 {
        let mut once_number = nums[0];
        if nums.clone().len() == 1 {
            return once_number;
        }
        let mut map = HashMap::new();

        for i in nums.clone() {
            if map.contains_key(&i) {
                let num = map.get(&i).unwrap();
                map.insert(i, num+1);
            } else {
                map.insert(i, 1);
            }
        }

        for j in  nums.clone() {
            if map.contains_key(&j) && map.get(&j).unwrap().eq(&1) {
                return j;
            }
        }

        once_number

    }
}
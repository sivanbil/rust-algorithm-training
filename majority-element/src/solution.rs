use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let compare = (nums.len()+1) as i32 / 2;
        let mut map = HashMap::new();

        for num in nums {
            let mut val = 1;
            if map.contains_key(&num) {
                val = *map.get(&num).unwrap() + val;
            }
            map.insert(num.clone(), val);
            if val >= compare {
                return num;
            }
        }

        0

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three() {
        assert_eq!(3,Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn test_two() {
        assert_eq!(2,Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}

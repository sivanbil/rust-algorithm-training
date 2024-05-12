use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            if set.contains(&nums.get(i)) {
                return true;
            } else {
                set.insert(nums.get(i));
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec1 = vec![1, 2, 3, 1];
        let duplicate = Solution::contains_duplicate(vec1);
       assert_eq!(duplicate, true);
    }

    #[test]
    fn test2() {
        let vec1 = vec![4, 2, 3, 1];
        let duplicate = Solution::contains_duplicate(vec1);
        assert_eq!(duplicate, false);
    }
}

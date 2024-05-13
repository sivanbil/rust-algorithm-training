pub struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let num_rows = row_index + 1;

        let mut curr = 0;
        let mut above = vec![];
        while curr < num_rows {
            curr =curr + 1;
            let mut inner_vec: Vec<i32> = vec![];
            for i in 0..curr {
                if i == 0 || i == curr -1 {
                    inner_vec.push(1i32);
                } else {
                    inner_vec.push(above[(i-1) as usize] + above[i as usize]);
                }
            }
            above = inner_vec.clone();
        }
        above
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let vec = Solution::get_row(1);
        assert_eq!(vec,vec![1, 1]);
    }

    #[test]
    fn test_three() {
        let vec = Solution::get_row(3);
        assert_eq!(vec,vec![1, 3, 3, 1]);
    }
}
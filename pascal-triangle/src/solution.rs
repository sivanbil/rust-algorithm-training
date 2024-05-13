pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut vec1:Vec<Vec<i32>> = vec![];
        if num_rows > 0 {
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

                vec1.push(inner_vec);


            }
        }
        vec1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let vec = Solution::generate(1);
        assert_eq!(vec, vec![vec![1]]);
    }
    #[test]
    fn test_five() {
        let vec = Solution::generate(5);
        assert_eq!(vec, vec![vec![1],vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }
}
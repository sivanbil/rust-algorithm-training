pub struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut top = 0;
        let mut bottom = n as usize - 1;
        let mut left = 0;
        let mut right = n as usize - 1;
        let mut num = 1;

        while num <= n * n {
            // 从左到右填充上行
            for col in left..=right {
                matrix[top][col] = num;
                num += 1;
            }
            top += 1;

            // 从上到下填充右列
            for row in top..=bottom {
                matrix[row][right] = num;
                num += 1;
            }
            right -= 1;

            // 从右到左填充下行
            if top <= bottom {
                for col in (left..=right).rev() {
                    matrix[bottom][col] = num;
                    num += 1;
                }
                bottom -= 1;
            }

            // 从下到上填充左列
            if left <= right {
                for row in (top..=bottom).rev() {
                    matrix[row][left] = num;
                    num += 1;
                }
                left += 1;
            }
        }

        matrix
    }
}

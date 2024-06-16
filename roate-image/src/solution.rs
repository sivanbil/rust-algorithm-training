pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for layer in 0..n / 2 {
            let last = n - 1 - layer;
            for i in layer..last {
                let offset = i - layer;
                let top = matrix[layer][i]; // 保存上面的元素

                // 左 -> 上
                matrix[layer][i] = matrix[last - offset][layer];

                // 下 -> 左
                matrix[last - offset][layer] = matrix[last][last - offset];

                // 右 -> 下
                matrix[last][last - offset] = matrix[i][last];

                // 上 -> 右 (从之前保存的元素)
                matrix[i][last] = top; // 上面的元素放到最右边
            }
        }
    }
}
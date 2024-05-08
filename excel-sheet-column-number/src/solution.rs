pub struct Solution;

impl Solution {
    // 将标题转成数字，而且是A-Z组合的标题，每个字母对应ASCII码 u32,然后又因为是从65开始的，所以再计算出ASCII码后减去64即可
    // 最低的位乘数是1，第二位就要考虑进位，因为是26进制，所以每增加一位，都需要将乘数放大26倍
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;

        let mut multiplier = 1;

        for char in column_title.chars().rev() {
            let ascii_val = char as u32 - 64;
            result += ascii_val * multiplier;

            multiplier *= 26;
        }

        result as i32
    }

}
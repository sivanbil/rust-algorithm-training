pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        let mut carry = 1;
        let mut i = result.len() as i64 - 1;

        // 在原来的数组上进行挨个数字处理，如果需要进位，则在最前面的位置进行插入进位数
        while carry > 0 && i >= 0 {
            let sum = result[i as usize] + carry;
            result[i as usize] = sum % 10;
            carry = sum / 10;
            i -= 1;
        }

        if carry > 0 {
            result.insert(0, carry);
        }

        result
    }
}
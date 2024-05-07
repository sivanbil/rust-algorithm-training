pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        let mut last_char_was_space = true;
        // 反方向统计即可
        for c in s.chars().rev() {
            if c == ' ' {
                if !last_char_was_space {
                    break;
                }
            } else {
                len += 1;
                last_char_was_space = false;
            }
        }

        len
    }
}
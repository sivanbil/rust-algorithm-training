pub struct Solution;


impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut target_occur_index = -1;
        if needle != "" && haystack.len() >= needle.len() {
            // 根据目标的长度，来划定依次挪动的窗口
            for i in 0..=(haystack.len() - needle.len()) {
                // 滑动窗口+切片
                if haystack[i..(i + needle.len())] == needle {
                    target_occur_index = i as i32;
                    break;
                }
            }
        }
        return target_occur_index;
    }
}
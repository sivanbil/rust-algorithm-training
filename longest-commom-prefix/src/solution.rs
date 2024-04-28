//Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
//
//
// Example 1:
//
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:
//
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
///最长字符串，一开始思路是计算数组的长度，然后从前第一个字符串开始遍历，依次与剩下的开始比较，
/// 相等则推进到数组里，当数组长度与原字符串数组的长度-1则代表，还要记录中断的状态，常规思路，破费周折。
///
/// 后面要做优化，只能减少遍历次数

pub struct Solution;

impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String  {
        match strs.is_empty() {
            true => "".to_string(),
            false => {
                // 用最短的字符串来遍历，然后利用迭代器来操作
                let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
                let first = &strs[0];
                for i in 0..min_len {
                    let c = first.chars().nth(i).unwrap();
                    if !strs.iter().all(|s| s.chars().nth(i) == Some(c)) {
                        return first[..i].to_string();
                    }
                }
                first[..min_len].to_string()
            }
        }
    }

    pub fn longest_common_prefix_up_little(strs: Vec<String>) -> String {
        let strs_len = strs.clone().len();
        if strs_len < 2 {
            return strs.join("");
        }

        let mut longest_prefix = String::from("");

        let mut stack = Vec::new();

        for (char_index, char) in strs[0].char_indices() {
            for j in 1..strs_len {
                if strs[j].len() <= char_index || &strs[j][char_index..char_index+1] !=&char.to_string() {
                    return longest_prefix;
                }
                stack.push(char.to_string());
                if stack.len() == strs_len-1 {
                    longest_prefix += &*char.to_string();
                    stack = Vec::new();
                }
            }
        }

        longest_prefix
    }

    pub fn longest_common_prefix_low(strs: Vec<String>) -> String {
        let strs_len = strs.clone().len();
        if strs_len < 2 {
            return strs.join("");
        }

        let first_str = strs.clone().first().unwrap().to_string();
        let mut stack = Vec::new();
        let mut start_index = 0;
        let mut longest_prefix = "".to_string();
        let mut is_broken = false;
        for char in first_str.chars() {
            for index in 1..strs.len() {
                let str_clone = strs.clone();
                if let Some(result ) = str_clone[index].get(start_index..start_index+1) {
                    if result.to_string() == char.to_string() && !is_broken{
                        stack.push(result.to_string());
                        if stack.len() == strs_len-1 {
                            longest_prefix += result;
                            stack = Vec::new();
                        }
                    } else {
                        is_broken = true;
                    }
                } else {
                    stack = Vec::new();
                }
            }
            start_index = start_index + 1;
        }


        longest_prefix
    }
}
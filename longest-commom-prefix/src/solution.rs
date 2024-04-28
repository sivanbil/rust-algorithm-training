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
///
///

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
            for index in 0..strs.len() {
                let str_clone = strs.clone();
                if let Some(result ) = str_clone[index].get(start_index..start_index+1) {
                    if result.to_string() == char.to_string() && !is_broken{
                        stack.push(result.to_string());
                        if stack.len() == strs_len {
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
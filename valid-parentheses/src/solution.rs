
/// 给定一个仅包含字符 、 、 和 的字符串，确定输入字符串是否有效。s'('')''{''}''['']'
///在以下情况下，输入字符串有效：
/// 打开的括号必须用相同类型的括号闭合。
/// 每个右括号都有一个相同类型的对应左括号。
///
/// 采用设置stack,依次推进stack,遇到相邻的一对，出栈，最后就看栈是否为空
///
pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '{' | '(' | '[' => stack.push(c),
                '}' | ')' | ']' => {
                    if let Some(&peek) = stack.last(){
                        if (c == ')' && peek== '(')
                            || (c=='}' && peek == '{' || c==']' && peek == '['){
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else {
                        stack.push(c);
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_parentheses_one() {
        let check_parentheses = "({}[()])".to_string();
        let is_valid_parentheses = Solution::is_valid(check_parentheses.clone());
        let expected = true;
        assert_eq!(Solution::is_valid(check_parentheses.clone()), expected);
    }

    #[test]
    fn test_is_parentheses_two() {
        let check_parentheses = "({}[()]){".to_string();
        let is_valid_parentheses = Solution::is_valid(check_parentheses.clone());
        let expected = false;
        assert_eq!(Solution::is_valid(check_parentheses.clone()), expected);
    }

    #[test]
    fn test_is_parentheses_third() {
        let check_parentheses = "]".to_string();
        let is_valid_parentheses = Solution::is_valid(check_parentheses.clone());
        let expected = false;
        assert_eq!(Solution::is_valid(check_parentheses.clone()), expected);
    }
}
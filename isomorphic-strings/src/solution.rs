use std::collections::{HashMap};

pub struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // 长度要一样且不能都为0，
        // 其次s字母与t映射替换后，能与t相等，则是同构
        if s == t {
            true
        } else {
            if s.len() != t.len() {
                false
            } else {
                let mut map: HashMap<String, String> = HashMap::new();
                let mut map_t_s: HashMap<String, String> = HashMap::new();
                let mut vec: Vec<String> = Vec::with_capacity(s.len());

                for i in 0..s.len() {
                    let s_curr = s[i..i+1].to_string();
                    let t_curr = t[i..i+1].to_string();
                    // 如果之前已经做了, great, aseat
                    if map.contains_key(&s_curr) {
                        let val = map.get(&s_curr).unwrap().to_string();
                        if val != t_curr {
                            return false;
                        }

                    } else if map_t_s.contains_key(&t_curr) {
                        let val = map_t_s.get(&t_curr).unwrap().to_string();
                        if val != s_curr {
                            return false;
                        }

                    }  else {
                        map.insert(s_curr.clone(), t_curr.clone());
                        map_t_s.insert(t_curr.clone(), s_curr.clone());
                    }
                    vec.push(t_curr.clone());
                }
                if vec.join("") == t {
                    return true
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let isomorphic = Solution::is_isomorphic(String::from("egg"), String::from("add"));
        assert_eq!(isomorphic, true);
    }

    #[test]
    fn test2() {
        let isomorphic = Solution::is_isomorphic(String::from("foo"), String::from("bar"));
        assert_eq!(isomorphic, false);
    }

    #[test]
    fn test3() {
        let isomorphic = Solution::is_isomorphic(String::from("paper"), String::from("title"));
        assert_eq!(isomorphic, true);

    }

    #[test]
    fn test4() {
        let isomorphic = Solution::is_isomorphic(String::from("badc"), String::from("baba"));
        assert_eq!(isomorphic, false);

    }
}
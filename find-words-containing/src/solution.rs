pub struct Solution;
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = vec![];
        let mut index = 0;
        for word in words {
            if word.contains(x) {
                result.push(index);
            }

            index +=1;
        }

        result
    }
}
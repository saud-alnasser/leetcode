struct Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> String {
        let mut output = String::new();

        if words.len() == 0 {
            return output;
        }

        let mut i = 0;

        loop {
            let mut current_char = None;

            for word in &words {
                let word_char = word.get(i..=i);

                if let None = word_char {
                    return output;
                }

                let word_char = word_char.unwrap();

                if let None = current_char {
                    current_char = Some(word_char);
                }

                if word_char != current_char.unwrap() {
                    return output;
                }
            }

            if let Some(ch) = current_char {
                output.push_str(ch);
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl".to_string()
        )
    }

    #[test]
    pub fn case_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            "".to_string()
        )
    }
}

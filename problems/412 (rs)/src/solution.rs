use std::collections::BTreeMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut rules = BTreeMap::from([(3, "Fizz"), (5, "Buzz")]);

        let mut output = vec![];

        for i in 1..=n {
            let mut text = String::new();

            for (num, segment) in rules.iter() {
                if i % num == 0 {
                    text.push_str(segment);
                }
            }

            if text.is_empty() {
                text = i.to_string();
            }

            output.push(text);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"])
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"])
    }

    #[test]
    pub fn case_3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        )
    }
}

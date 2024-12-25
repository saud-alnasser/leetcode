use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut collection = HashMap::new();

        for ch in magazine.chars() {
            collection.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        for ch in ransom_note.chars() {
            collection.entry(ch).and_modify(|c| *c -= 1).or_insert(-1);
        }

        for count in collection.values() {
            if *count < 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn case_1() {
        let ransom_note = "a".into();
        let magazine = "b".into();

        assert_eq!(Solution::can_construct(ransom_note, magazine), false)
    }

    #[test]
    pub fn case_2() {
        let ransom_note = "aa".into();
        let magazine = "ab".into();

        assert_eq!(Solution::can_construct(ransom_note, magazine), false)
    }

    #[test]
    pub fn case_3() {
        let ransom_note = "aa".into();
        let magazine = "aab".into();

        assert_eq!(Solution::can_construct(ransom_note, magazine), true)
    }
}

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        if x == 0 {
            return true;
        }

        let mut digits: Vec<u8> = Vec::with_capacity(10);
        let mut v = x;

        while v > 0 {
            let n = (v % 10) as u8;
            v /= 10;
            digits.push(n);
        }

        if digits.len() == 1 {
            return true;
        }

        let mut i = 0;
        let mut j = digits.len() - 1;

        while i < j {
            if digits.get(i) != digits.get(j) {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::is_palindrome(121), true)
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }

    #[test]
    pub fn case_3() {
        assert_eq!(Solution::is_palindrome(10), false)
    }
}

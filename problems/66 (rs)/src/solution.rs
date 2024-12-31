struct Solution;

#[allow(unused)]
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut index = (digits.len() - 1) as i32;
        let mut carry = 1;

        loop {
            if index < 0 {
                if carry <= 0 {
                    break;
                }

                digits.insert(0, 0);
                index = 0;
            }

            let factor = digits[index as usize] + carry;

            digits[index as usize] = factor % 10;

            carry = if factor > 9 { factor - 9 } else { 0 };

            index -= 1;
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4])
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2])
    }

    #[test]
    pub fn case_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0])
    }
}

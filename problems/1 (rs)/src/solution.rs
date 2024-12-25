use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (j, current) in nums.iter().enumerate() {
            let x = target - current;

            if let Some(i) = map.get(&x) {
                return vec![*i as i32, j as i32];
            }

            map.insert(*current, j);
        }

        unreachable!(
            "this should've been unreadable due to the input constraints on the given description!"
        )
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    pub fn case_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1])
    }
}

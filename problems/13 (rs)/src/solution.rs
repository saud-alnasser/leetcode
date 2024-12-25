struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut numerals = s.chars().map(Numeral::from).peekable();

        while let Some(current) = numerals.next() {
            sum += {
                if (current == Numeral::I
                    && (numerals.peek() == Some(&Numeral::V)
                        || numerals.peek() == Some(&Numeral::X)))
                    || (current == Numeral::X
                        && (numerals.peek() == Some(&Numeral::L)
                            || numerals.peek() == Some(&Numeral::C)))
                    || (current == Numeral::C
                        && (numerals.peek() == Some(&Numeral::D)
                            || numerals.peek() == Some(&Numeral::M)))
                {
                    i32::from(numerals.next().unwrap()) - i32::from(current)
                } else {
                    i32::from(current)
                }
            }
        }

        sum
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Numeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<char> for Numeral {
    fn from(ch: char) -> Self {
        match ch {
            'I' => Numeral::I,
            'V' => Numeral::V,
            'X' => Numeral::X,
            'L' => Numeral::L,
            'C' => Numeral::C,
            'D' => Numeral::D,
            'M' => Numeral::M,
            _ => panic!("invalid numeral"),
        }
    }
}

impl From<Numeral> for i32 {
    fn from(value: Numeral) -> i32 {
        match value {
            Numeral::I => 1,
            Numeral::V => 5,
            Numeral::X => 10,
            Numeral::L => 50,
            Numeral::C => 100,
            Numeral::D => 500,
            Numeral::M => 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::roman_to_int("III".into()), 3)
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58)
    }

    #[test]
    pub fn case_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994)
    }
}

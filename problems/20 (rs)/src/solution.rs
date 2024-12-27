struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let brackets = s.chars().map(Bracket::from);
        let mut stack: Vec<Bracket> = vec![];

        for bracket in brackets {
            if bracket.open() {
                stack.push(bracket);
            } else {
                let open = match stack.pop() {
                    Some(bracket) => bracket,
                    None => return false,
                };

                let close = bracket;

                if !open.accept(close) {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Paren(bool),
    Square(bool),
    Brace(bool),
}

impl Bracket {
    pub fn open(&self) -> bool {
        *match self {
            Bracket::Paren(open) => open,
            Bracket::Square(open) => open,
            Bracket::Brace(open) => open,
        }
    }

    pub fn accept(&self, other: Bracket) -> bool {
        if let _current @ Bracket::Paren(..) = self {
            if let _other @ Bracket::Paren(..) = other {
                return true;
            }
        }

        if let _current @ Bracket::Square(..) = self {
            if let _other @ Bracket::Square(..) = other {
                return true;
            }
        }

        if let _current @ Bracket::Brace(..) = self {
            if let _other @ Bracket::Brace(..) = other {
                return true;
            }
        }

        false
    }
}

impl From<char> for Bracket {
    fn from(value: char) -> Self {
        match value {
            '(' => Bracket::Paren(true),
            ')' => Bracket::Paren(false),
            '[' => Bracket::Square(true),
            ']' => Bracket::Square(false),
            '{' => Bracket::Brace(true),
            '}' => Bracket::Brace(false),
            _ => panic!("invalid bracket"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn case_1() {
        assert_eq!(Solution::is_valid("()".into()), true);
    }

    #[test]
    pub fn case_2() {
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
    }

    #[test]
    pub fn case_3() {
        assert_eq!(Solution::is_valid("(]".into()), false);
    }

    #[test]
    pub fn case_4() {
        assert_eq!(Solution::is_valid("([])".into()), true);
    }
}

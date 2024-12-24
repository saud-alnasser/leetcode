pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut elements = vec![];
        let mut current = head;

        while let Some(node) = current {
            elements.push(node.val);
            current = node.next;
        }

        let mut i = 0;
        let mut j = elements.len() - 1;

        while i < j {
            if (elements[i] - elements[j]) != 0 {
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
    use super::*;

    #[test]
    pub fn case_1() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::is_palindrome(head), true)
    }

    #[test]
    pub fn case_2() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

        assert_eq!(Solution::is_palindrome(head), false)
    }

    #[test]
    pub fn case_3() {
        let mut head = Some(Box::new(ListNode::new(1)));

        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(2)));

        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::is_palindrome(head), true)
    }
}

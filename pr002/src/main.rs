use std::convert::TryFrom;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Converts a slice to a ListNode in reverse order.
fn from_vec_helper(mut input: Vec<i32>) -> Option<Box<ListNode>> {
    if let Some(x) = input.pop() {
        Some(Box::new(ListNode {
            val: x,
            next: from_vec_helper(input),
        }))
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ZeroLengthError {
    _priv: (),
}

impl TryFrom<Vec<i32>> for ListNode {
    type Error = ZeroLengthError;
    fn try_from(mut value: Vec<i32>) -> Result<Self, Self::Error> {
        value.reverse();
        from_vec_helper(value)
            .map(|x| *x)
            .ok_or(ZeroLengthError { _priv: () })
    }
}

impl From<ListNode> for Vec<i32> {
    fn from(value: ListNode) -> Self {
        let mut output = Vec::new();
        output.push(value.val);
        let mut curr_node = value.next;
        while curr_node != None {
            let (next_node, val) = pop(curr_node);
            output.push(val);
            curr_node = next_node;
        }
        output
    }
}

fn pop(input: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
    if let Some(x) = input {
        (x.next, x.val)
    } else {
        (None, 0)
    }
}

/// Add with carry
pub fn adc(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    let (l1_next, l1_val) = pop(l1);
    let (l2_next, l2_val) = pop(l2);
    let total = l1_val + l2_val + carry;
    if l1_next == None && l2_next == None && total == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: adc(l1_next, l2_next, total / 10),
            val: total % 10,
        }))
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1 == None && l2 == None {
        None
    } else {
        let result = adc(l1, l2, 0);
        if result == None {
            Some(Box::new(ListNode::new(0)))
        } else {
            result
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_add(l1: Vec<i32>, l2: Vec<i32>, output: Vec<i32>) {
        let parsed_l1 = l1.try_into().map(|x| Box::new(x)).ok();
        let parsed_l2 = l2.try_into().map(|x| Box::new(x)).ok();
        let parsed_output = output.try_into().map(|x| Box::new(x)).ok();
        assert_eq!(add_two_numbers(parsed_l1, parsed_l2), parsed_output);
    }

    #[test]
    fn test_ex_1() {
        let l1 = vec![2, 4, 3];
        let l2 = vec![5, 6, 4];
        let output = vec![7, 0, 8];
        test_add(l1, l2, output);
    }
    #[test]
    fn test_ex_2() {
        let l1 = vec![0];
        let l2 = vec![0];
        let output = vec![0];
        test_add(l1, l2, output);
    }
    #[test]
    fn test_ex_3() {
        let l1 = vec![9, 9, 9, 9, 9, 9, 9];
        let l2 = vec![9, 9, 9, 9];
        let output = vec![8, 9, 9, 9, 0, 0, 0, 1];
        test_add(l1, l2, output);
    }
}

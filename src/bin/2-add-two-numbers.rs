struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);

        let mut curr = &mut result;
        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        let mut c = 0;

        while p.is_some() || q.is_some() {
            let x = p.map_or(0, |n| n.val);
            let y = q.map_or(0, |n| n.val);


            let sum = x + y + c;

            c = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();

            p = p.and_then(|n| n.next.as_ref());
            q = q.and_then(|n| n.next.as_ref());
        }

        if c > 0 {
            curr.next = Some(Box::new(ListNode::new(1)));
        }

        result.next
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode::new(8)));
    let l2 = Some(Box::new(ListNode::new(9)));
    let mut result = Some(Box::new(ListNode::new(7)));
    result.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));

    assert_eq!(result, Solution::add_two_numbers(l1, l2));
}
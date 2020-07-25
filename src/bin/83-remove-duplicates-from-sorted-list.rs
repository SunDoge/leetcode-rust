// Definition for singly-linked list.
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

struct Solution;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut h = head;
        let mut p1 = h.as_mut().unwrap();

        while let Some(p2) = p1.next.as_mut() {
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }

        h
    }
}

fn new_list_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode { val, next }))
}

fn main() {
    let head = new_list_node(
        1,
        new_list_node(
            1,
            new_list_node(2, new_list_node(3, new_list_node(3, None))),
        ),
    );
    let output = new_list_node(1, new_list_node(2, new_list_node(3, None)));

    assert_eq!(Solution::delete_duplicates(head), output);
}

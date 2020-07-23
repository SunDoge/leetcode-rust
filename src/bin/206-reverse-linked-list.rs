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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut boxed_node) = curr {
            let mut next = boxed_node.next.take();
            boxed_node.next = prev.take();
            prev = Some(boxed_node);
            curr = next.take();
        }

        prev
    }
}

fn new_list_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode { val, next }))
}

fn main() {
    let input = new_list_node(
        1,
        new_list_node(
            2,
            new_list_node(3, new_list_node(4, new_list_node(5, None))),
        ),
    );

    let output = new_list_node(
        5,
        new_list_node(
            4,
            new_list_node(3, new_list_node(2, new_list_node(1, None))),
        ),
    );

    assert_eq!(Solution::reverse_list(input), output);
}

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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(left), Some(right)) => {
                if left.val < right.val {
                    Some(Box::new(ListNode {
                        val: left.val,
                        next: Self::merge_two_lists(left.next, Some(right)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: right.val,
                        next: Self::merge_two_lists(Some(left), right.next),
                    }))
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}

fn new_list_node(val: i32, next: Option<Box<ListNode>>) -> Box<ListNode> {
    Box::new(ListNode { val, next })
}

fn main() {
    let l1 = Some(new_list_node(
        1,
        Some(new_list_node(2, Some(new_list_node(4, None)))),
    ));

    let l2 = Some(new_list_node(
        1,
        Some(new_list_node(3, Some(new_list_node(4, None)))),
    ));

    let r = Some(new_list_node(
        1,
        Some(new_list_node(
            1,
            Some(new_list_node(
                2,
                Some(new_list_node(
                    3,
                    Some(new_list_node(4, Some(new_list_node(4, None)))),
                )),
            )),
        )),
    ));

    assert_eq!(Solution::merge_two_lists(l1, l2), r);
}

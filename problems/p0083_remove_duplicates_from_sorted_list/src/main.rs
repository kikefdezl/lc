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
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr = &mut head;

        while let Some(node) = curr {
            while let Some(next) = &node.next
                && (next.val == node.val)
            {
                node.next = node.next.take().unwrap().next;
            }
            curr = &mut node.next;
        }
        head
    }
}

fn main() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        })),
    }));

    let expected = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    assert_eq!(Solution::delete_duplicates(input), expected);
}

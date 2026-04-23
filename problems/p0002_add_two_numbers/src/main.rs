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

    fn print(&self) {
        print!("{}", self.val);
        let mut curr = &self.next;
        while let Some(next) = curr {
            print!(" -> {}", next.val);
            curr = &next.next;
        }
        println!();
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node1) = l1 {
                sum += node1.val;
                l1 = node1.next
            };
            if let Some(node2) = l2 {
                sum += node2.val;
                l2 = node2.next;
            }
            carry = sum / 10;
            sum %= 10;

            curr.next = Some(Box::new(ListNode::new(sum)));
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }
}

fn build(values: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;
    for &value in values {
        curr.next = Some(Box::new(ListNode::new(value)));
        curr = curr.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {
    let l1 = build(&[2, 4, 3]);
    let l2 = build(&[5, 6, 4]);
    let sum = Solution::add_two_numbers(l1, l2);
    sum.unwrap().print();

    let l1 = build(&[0]);
    let l2 = build(&[0]);
    let sum = Solution::add_two_numbers(l1, l2);
    sum.unwrap().print();

    let l1 = build(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = build(&[9, 9, 9, 9]);
    let sum = Solution::add_two_numbers(l1, l2);
    sum.unwrap().print();
}

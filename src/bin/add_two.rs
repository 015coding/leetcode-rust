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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans: Option<Box<ListNode>> = None;
        let mut tail = &mut ans;

        let mut c1 = l1.as_ref();
        let mut c2 = l2.as_ref();
        let mut c = 0;

        loop {
            if c1.is_none() && c2.is_none() && c == 0 {
                break;
            }
            let mut x = 0;
            let mut y = 0;
            let mut current = 0;

            if let Some(node) = c1 {
                x = node.val;
                c1 = node.next.as_ref();
            } else {
                x = 0;
            }

            if let Some(node) = c2 {
                y = node.val;
                c2 = node.next.as_ref();
            } else {
                y = 0;
            }
            current = (x + y + c) % 10;
            c = (x + y + c) / 10;
            *tail = Some(Box::new(ListNode {
                val: current,
                next: None,
            }));
            tail = &mut tail.as_mut().unwrap().next;
        }

        ans
    }
}

fn main() {
    // l1 = [2, 4, 3]
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));

    // l2 = [5, 6, 4]
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));

    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);

    let result = Solution::add_two_numbers(l1, l2);

    println!("result = {:?}", result);
}

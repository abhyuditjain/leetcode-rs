#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_arr(v: &[i32]) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;

        for x in v {
            current.next = Some(Box::new(ListNode::new(*x)));
            current = current.next.as_mut().unwrap();
        }

        front.next
    }
}

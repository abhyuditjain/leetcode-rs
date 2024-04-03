use leetcode_rs::utils::listnode::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    dummy.next = head;

    let mut dummy = Box::new(dummy);

    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while fast.next.is_some() {
        slow = slow.next.as_mut().unwrap();
        fast = fast.next.unwrap();
    }

    let next = slow.next.as_mut().unwrap();

    slow.next = next.next.clone();

    dummy.next
}

fn main() {}

#[cfg(test)]
mod tests {
    use leetcode_rs::utils::listnode::ListNode;

    use crate::remove_nth_from_end;

    #[test]
    fn case_1() {
        let list = ListNode::from_arr(&[1, 2, 3, 4, 5]);

        assert_eq!(
            remove_nth_from_end(list, 2),
            ListNode::from_arr(&[1, 2, 3, 5])
        );
    }

    #[test]
    fn case_2() {
        let list = ListNode::from_arr(&[1]);

        assert_eq!(remove_nth_from_end(list, 1), ListNode::from_arr(&[]));
    }

    #[test]
    fn case_3() {
        let list = ListNode::from_arr(&[1, 2]);

        assert_eq!(remove_nth_from_end(list, 1), ListNode::from_arr(&[1]));
    }
}

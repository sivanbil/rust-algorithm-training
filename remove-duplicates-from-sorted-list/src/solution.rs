pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut prev = &mut dummy;
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();
            // 比较当前节点与之前的节点的val进行比较，如果curr不为None，则比较val，满足条件或者本身curr为None
            // 则设置下一个节点
            if curr.as_ref().map_or(true, |next| next.val != node.val) {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
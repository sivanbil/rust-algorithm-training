pub struct Solution;

/// Input: head = [1,2,6,3,4,5,6], val = 6
/// Output: [1,2,3,4,5]
///
///Input: head = [], val = 1
/// Output: []
/// Example 3:
///
/// Input: head = [7,7,7,7], val = 7
/// Output: []
///
/// Definition for singly-linked list.
/// 分析：
/// 单向链表，删除某个元素，虽然可以通过循环来删除，但是并没有维护下到上的关系，这样就不好设置next
/// 节点的信息
///
/// 这里考虑使用链表依次通过next遍历，然后存储顺序到数组里，然后遍历数组把关系做上
///
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
  pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head;
    let mut curr = dummy.as_mut().unwrap();

    while let Some(node) = curr.next.as_mut() {
      if node.val == val {
        curr.next = node.next.take();
      } else {
        curr = curr.next.as_mut().unwrap();
      }
    }

    dummy.unwrap().next
  }
}
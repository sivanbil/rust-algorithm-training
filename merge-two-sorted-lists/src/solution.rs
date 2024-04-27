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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            // 都是空的
            (None, None) => None,
            // 其中一个是空的
            (Some(list), None) | (None, Some(list)) => Some(list),
            // 两个都非空
            (Some(mut list1), Some(mut list2)) => {
                if list1.val <= list2.val {
                    // 继续拿着list1的next与list2的所有节点进行比较
                    list1.next = Solution::merge_two_lists(list1.next, Some(list2));
                    Some(list1)
                } else {
                    // 继续拿着list2的next与list1的所有节点进行比较
                    list2.next = Solution::merge_two_lists(Some(list1), list2.next);
                    Some(list2)
                }
            }
        }
    }
}
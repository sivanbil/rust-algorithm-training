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
            (None, None) => None,
            (Some(list), None) | (None, Some(list)) => Some(list),
            (Some(mut list1), Some(mut list2)) => {
                if list1.val <= list2.val {
                    list1.next = Solution::merge_two_lists(list1.next, Some(list2));
                    Some(list1)
                } else {
                    list2.next = Solution::merge_two_lists(Some(list1), list2.next);
                    Some(list2)
                }
            }
        }
    }

}
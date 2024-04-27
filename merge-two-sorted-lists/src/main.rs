mod solution;
use crate::solution::ListNode;
use crate::solution::Solution;

fn main() {
    let  list1_head = Some(Box::from(ListNode::new(2)));
    let mut list2_head = ListNode::new(1);
    list2_head.next = Some(Box::from(ListNode::new(3)));
    let result = Solution::merge_two_lists(list1_head, Some(Box::from(list2_head)));
    println!("Hello, world!{:?}", result);
}

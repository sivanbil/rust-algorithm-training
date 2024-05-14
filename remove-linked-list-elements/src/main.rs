mod solution;

use std::collections::linked_list;
use crate::solution::{Solution, ListNode};
// :https://leetcode.cn/problems/remove-linked-list-elements/description/
fn main() {
    let mut linked_list = ListNode::new(1);

    let mut linked_list_node2 = ListNode::new(2);

    let mut linked_list_node3 = Box::from(ListNode::new(6));

    let mut linked_list_node4 = Box::from(ListNode::new(3));

    let mut linked_list_node5 = Box::from(ListNode::new(4));

    let mut linked_list_node6 = Box::from(ListNode::new(5));

    let mut linked_list_node7 = Box::from(ListNode::new(6));
    linked_list_node6.next = Some(Box::from(linked_list_node7));

    linked_list_node5.next = Some(Box::from(linked_list_node6.clone()));

    linked_list_node4.next = Some(Box::from(linked_list_node5.clone()));

    linked_list_node3.next = Some(Box::from(linked_list_node4.clone()));

    linked_list_node2.next = Some(Box::from(linked_list_node3.clone()));

    linked_list.next = Some(Box::from(linked_list_node2.clone()));

    let result = Solution::remove_elements(Some(Box::from(linked_list.clone())), 2);
    println!("{:?}", result);
}

use std::collections::{HashSet};

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
    pub fn get_intersection_node(node_a:  Option<Box<ListNode>>, node_b:  Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if node_a.is_none() || node_b.is_none() {
            return None;
        }

        let mut pointer_a = node_a.clone();
        let mut pointer_b = node_b.clone();

        while pointer_a != pointer_b {
            // 重定向指针，是为了解决两个链表长度不相等的情况，从而让循环仍然能够继续
            pointer_a = match pointer_a {
                None => node_b.clone(),
                Some(node) => node.next
            };

            pointer_b = match pointer_b {
                None => node_a.clone(),
                Some(node) => node.next
            }
        }

        pointer_a
    }

    pub fn get_intersection_node_low(node_a:  Option<Box<ListNode>>, node_b:  Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr_a = &node_a;
        let mut exist_map = HashSet::new();

        while let Some(node) = ptr_a  {
            exist_map.insert(node.val);

            ptr_a = &node.next;
        }

        let mut ptr_b = &node_b;
        while let Some(node) = ptr_b {
            if exist_map.contains(&node.val) {
                return Some(node.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建一个链表节点
    fn create_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }

    #[test]
    fn test_get_intersection_node() {
        // 测试用例1: 两个链表不相交
        let list1 = create_node(1, create_node(2, create_node(3, None)));
        let list2 = create_node(4, create_node(5, None));
        assert_eq!(Solution::get_intersection_node(list1, list2), None);

        // 测试用例2: 两个链表相交
        let shared_node = create_node(7, create_node(8, create_node(9, None)));
        let list1 = create_node(1, create_node(2, create_node(3, shared_node.clone())));
        let list2 = create_node(4, create_node(5, create_node(6, shared_node.clone())));
        assert_eq!(Solution::get_intersection_node(list1, list2), shared_node.clone());

        // 测试用例3: 其中一个链表为空
        let list1 = create_node(1, create_node(2, create_node(3, None)));
        let list2 = None;
        assert_eq!(Solution::get_intersection_node(list1, list2), None);

        // 测试用例4: 两个链表都为空
        let list1 = None;
        let list2 = None;
        assert_eq!(Solution::get_intersection_node(list1, list2), None);
    }
}
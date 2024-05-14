mod solution;
use std::rc::Rc;
use std::cell::RefCell;
use crate::solution::{Node, Solution};
// https://leetcode.cn/problems/linked-list-cycle/description/
fn main() {
    // 创建一个循环链表
    let mut nodes = Vec::new();

    for i in 1..=5 {
        nodes.push(Rc::new(RefCell::new(Node {
            value: i,
            next: None,
        })));
    }

    for i in 0..4 {
        nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
    }
    nodes[4].borrow_mut().next = Some(Rc::clone(&nodes[1]));

    let head = Some(Rc::clone(&nodes[0]));


    let result = Solution::has_cycle(&head);

    println!("{}", result);
}

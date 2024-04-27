// 引用计数用于多个所有者之间共享数据的ownership
use std::rc::Rc;
// 智能指针-内部可变性
use std::cell::RefCell;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Solution;

impl Solution {
    /// 循环链表-采用类似c++的smart pointer来实现节点引用
    ///
    pub fn has_cycle<T>(head: &Option<Rc<RefCell<Node<T>>>>) -> bool {
        let mut slow = head.as_ref().map(|node| Rc::clone(node));
        let mut fast = head.as_ref().map(|node| Rc::clone(node));

        // 在安全和受控的方式下，进行处理
        while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
            slow = slow.unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));
            fast = fast.unwrap().borrow().next.as_ref().unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));

            if Rc::ptr_eq(slow.as_ref().unwrap(), fast.as_ref().unwrap()) {
                return true;
            }
        }

        false
    }
}

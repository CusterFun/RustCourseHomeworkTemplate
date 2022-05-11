#![allow(unused)]

use std::rc::{Rc, Weak};

// 2 Assignment

// 实现 Drop 打印 id
struct Node {
    id: i32,
    next: Option<Box<Node>>,
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
fn generate_n_loop_pointer(n: usize) -> Node {
    let mut head = Node { id: 0, next: None };
    let mut values = Vec::new();
    for i in 1..n {
        values.push(Node {
            id: i as i32,
            next: None,
        });
    }

    let mut list = head;
    for i in values {
        list.next = Some(Box::new(i));
    }
    list
}

// Drop 打印 id
struct WeakNode {
    id: i32,
    next: Weak<Node>,
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
fn generate_n_loop_weak_pointer(n: usize) -> WeakNode {
    todo!()
}

fn main() {}

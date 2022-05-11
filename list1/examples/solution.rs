#![allow(unused)]

/// 链表数字顺序从小到大
/// push(3)
/// 3
/// push(1)
/// 1 -> 3
/// push(2)
/// 1 -> 2 -> 3
/// pop(2)
/// 1 -> 3

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Clone)]
struct Node {
    val: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    // insert a value
    // 如果有相同的数字，仍然插入
    fn push(&mut self, v: i32) {
        let mut node = Box::new(Node { val: v, next: None });
        match self.head.as_mut() {
            Some(head) => {
                if head.val >= v {
                    node.next = self.head.take();
                    self.head = Some(node);
                    return;
                }
            }
            None => {
                self.head = Some(node);
                return;
            }
        }
        let mut cur = self.head.as_mut().unwrap();
        while let Some(n) = &cur.next {
            if n.val >= v {
                // insert n after cur
                node.next = cur.next.take();
                cur.next = Some(node);
                return;
            }
            cur = cur.next.as_mut().unwrap();
        }
        // add to last
        cur.next = Some(node);
    }

    // pop 有这个数字 就返回 Some(i32)
    // 没有 就返回None
    // 如果有相同的数字，就删除一个就好了
    fn pop(&mut self, v: i32) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }
        let mut cur = self.head.as_mut().unwrap();
        while let Some(n) = &mut cur.next {
            if n.val == v {
                // remove n
                cur.next = n.next.take();
                return Some(v);
            }
        }
        None
    }
}

// O(1)
// Iter, IterMut, IntoIterator
// 顺序是从小的数字到大的数字输出
pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl List {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

impl List {
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

pub struct IntoIter {
    head: Link,
}

impl List {
    pub fn into_iter(self) -> IntoIter {
        IntoIter { head: self.head }
    }
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut list = List::new();
        list.push(3);
        list.push(5);
        list.push(1);
        list.push(2);
        list.push(2);
        for i in list.iter() {
            println!("{}", i);
        }
        list.pop(2);
        for i in list.iter_mut() {
            println!("{}", i);
        }
        list.pop(2);
        for i in list.into_iter() {
            println!("{}", i);
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(3);
    list.push(1);
    list.push(2);
    list.push(5);
    list.push(2);
    for i in list.iter() {
        println!("{}", i);
    }
}
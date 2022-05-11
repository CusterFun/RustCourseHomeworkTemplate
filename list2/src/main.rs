#![allow(unused)]

// 1个数据结构 它有自己的数据
// 还有自己的一些操作。这些操作就维持了数据一些结构保证它的一些特性
// stack push pop
// queue push pop
// 持久化的数据结构就是 可以有多个数据结构(历史版本)它们可以共享一些data
// 保持数据结构的历史性
// List, treap, segment tree, ...
// n1 -> n2 -> n3 -> n4

// prepend
// n4: v1
// n3 -> n4: v2
// n2 -> n3 -> n4: v3
// n1 -> n2 -> n3 -> n4: v4
// list: n1 -> n2 -> n3 -> n4 (指针指向n4)
// 我想知道 v1 版本的list
// 得到一个新的返回值 list_v1: n4
// v2: 版本的  list_v2: n3 -> n4
// v5: 版本 pop 一个出去 n2 -> n3 -> n4
// v6: n6 -> n2 -> n3 -> n4
// v6 版本怎么得到 v4 版本的list
// n1 -> n2 -> n3 -> n4 -> .. -> n10000
//       ^
//       |
//       n6
// 我们怎么实现一个持久化的链表
// v6 版本的 list
// n6 -> n2 -> n3 -> n4
// 多重Hash
// History
// rc

pub mod vec_list;

struct ListHistory<T: Clone> {
    list: Vec<List<T>>,
    size: usize,
}

// implement Drop trait and call methods
impl<T: Clone> ListHistory<T> {
    pub fn new() -> Self {
        ListHistory {
            list: vec![List::new()],
            size: 1,
        }
    }

    pub fn get_list_version(&self, version: u64) -> Option<&List<T>> {
        if version as usize > self.size {
            return None;
        }
        Some(&self.list[version as usize])
    }

    pub fn prepend(&mut self, elem: T) -> List<T> {
        let new_list = self.list[self.size - 1].prepend(elem.clone());
        self.list.push(new_list);
        self.size += 1;
        self.list[self.size - 1].prepend(elem)
    }

    pub fn tail(&mut self) -> List<T> {
        self.size -= 1;
        self.list.pop().unwrap()
    }

    pub fn current(&self) -> List<T> {
        self.list[self.size - 1].clone()
    }
}

impl<T: Clone> Drop for ListHistory<T> {
    fn drop(&mut self) {}
}

impl<T: Eq + Clone> std::ops::Index<usize> for ListHistory<T> {
    type Output = List<T>;
    fn index(&self, pos: usize) -> &Self::Output {
        &self.list[pos]
    }
}

use std::rc::Rc;

#[derive(Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(), // Rc
            })),
        }
    }

    // 得到一个 prepend 之前的一个版本
    // n1 -> n2 -> n3
    // n2 -> n3
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    // 得到 头的数据
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// push pop
// push_front push_back
// pop_front pop_back
// peek_front peek_back

// 加这个 Drop 的初衷
// 其实是 List 过长的话就会 栈溢出
// 怎么样防止栈溢出

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut rc_node) = cur_link {
//             // rc_node: mut Rc<Node<T>>
//             // replace, None)
//             // mem::replace(Rc<Node<T>>, None)
//             // &mut T
//             cur_link = rc_node.next.take();
//         }
//     }
// }

fn main() {}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn list_history() {
        let mut version = ListHistory::<i32>::new();
        assert_eq!(version.list[0].head(), None);
        let list = version.prepend(1);
        assert_eq!(version.list[1].head(), Some(&1));
        let list = version.prepend(2);
        assert_eq!(list.head(), version.list[2].head() /* , Some(&2)*/);
        let list = version.get_list_version(2).unwrap();
        assert_eq!(list.deref().head(), Some(&2));
        let list = version.tail();
        assert_eq!(list.head(), Some(&2));
        let list = version.current();
        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

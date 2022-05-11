use std::ops::{Index, IndexMut};
use std::cmp::Ordering;


pub trait List<T>: 
    Eq + Index<usize> + IndexMut<usize> + IntoIterator
{
    type Error;

    fn insert(&mut self, pos: usize, elem: T) -> Result<(), Self::Error>;
    fn remove(&mut self, pos: usize) -> Result<T, Self::Error>;
    fn length(&self) -> usize;
    fn contains(&self, elem: T) -> bool;
}


#[derive(Clone, Eq, PartialEq)]
pub struct VecList<T: Eq + Clone> {
    pub elems: Vec<T>,
}


impl<T: Eq + Clone> List<T> for VecList<T> {
    type Error = String;

    fn insert(&mut self, pos: usize, elem: T) -> Result<(), Self::Error> {
        match pos.cmp(&self.elems.len()) {
            Ordering::Less    => Ok(self.elems.insert(pos, elem)),
            Ordering::Equal   => Ok(self.elems.push(elem)),
            Ordering::Greater => Err("Out of bounds".to_string()),
        }
    }

    fn remove(&mut self, pos: usize) -> Result<T, Self::Error> {
        match pos.cmp(&self.elems.len()) {
            Ordering::Less => Ok(self.elems.remove(pos)),
            _ => Err("Out of bounds".to_string()),
        }
    }

    fn length(&self) -> usize {
        self.elems.len()
    }

    fn contains(&self, elem: T) -> bool {
        self.iter().any(|x| *x == elem)
    }
}


impl<T: Eq + Clone> VecList<T> {
    pub fn iter(&self) -> RefVecListIterator<T> {
        self.into_iter()
    }
}


impl<T: Eq + Clone> Index<usize> for VecList<T> {
    type Output = T;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.elems[pos]
    }
}

impl<T: Eq + Clone> IndexMut<usize> for VecList<T> {
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.elems[pos]
    }
}


pub struct VecListIterator<T: Eq + Clone> {
    pos: usize,
    list: VecList<T>,
}

impl<T: Eq + Clone> Iterator for VecListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.list.length() {
            self.pos += 1;
            Some(self.list[self.pos - 1].clone())
        } else {
            None
        }
    }
}

impl<T: Eq + Clone> IntoIterator for VecList<T> {
    type Item = T;
    type IntoIter = VecListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecListIterator { pos: 0, list: self }
    }
}


pub struct RefVecListIterator<'a, T: Eq + Clone> {
    pos: usize,
    list: &'a VecList<T>,
}

impl<'a, T: Eq + Clone> Iterator for RefVecListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.list.length() {
            self.pos += 1;
            Some(&self.list[self.pos - 1])
        } else {
            None
        }
    }
}


impl<'a, T: Eq + Clone> IntoIterator for &'a VecList<T> {
    type Item = &'a T;
    type IntoIter = RefVecListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefVecListIterator {
            pos: 0,
            list: self,
        }
    }
}

impl<'a, T: Eq + Clone> IntoIterator for &'a mut VecList<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter_mut()
    }
}



#[test]
fn modify_values() {
    let mut x = VecList { elems: vec![1, 2, 3] };
    
    for i in &mut x {
        *i += 1;
    }
    
    assert_eq!(x.elems, vec![2, 3, 4]);
}
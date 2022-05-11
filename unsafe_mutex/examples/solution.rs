use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(contents: T) -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(contents),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::thread::yield_now();
        }

        MutexGuard {
            mutex: self as *const _,
        }
    }
}

pub struct MutexGuard<T> {
    mutex: *const Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<T> {
    fn drop(&mut self) {
        let t = unsafe { &*self.mutex };
        t.locked.store(false, Ordering::Release);
    }
}

use std::ops::{Deref, DerefMut};

impl<'a, T> Deref for MutexGuard<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let t = unsafe { &*self.mutex };
        unsafe { &*t.data.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let t = unsafe { &*self.mutex };
        unsafe { &mut *t.data.get() }
    }
}

struct Counter {
    mutex: Mutex<usize>,
}

fn increment_the_counter(ctr: &Counter) {
    let mut guard = ctr.mutex.lock();
    *guard += 1;
}

fn read_the_counter(ctr: &Counter) -> usize {
    let guard = ctr.mutex.lock();
    *guard
}

use std::sync::Arc;
use std::thread;

unsafe impl Sync for Counter {}
unsafe impl Send for Counter {}

fn main() {
    let ctr = Arc::new(Counter {
        mutex: Mutex::new(0),
    });
    let mut threads = Vec::new();
    for _ in 0..10 {
        let c = ctr.clone();
        let th = thread::spawn(move || {
            for _ in 0..1000 {
                increment_the_counter(&c);
            }
        });
        threads.push(th);
    }
    for th in threads {
        th.join().unwrap();
    }
    let res = read_the_counter(&ctr);
    assert_eq!(10 * 1000, res);
    println!("{}", res);
}

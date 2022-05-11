use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::spawn;

#[test]
fn test() {
    let num = 100;
    let mutex = Arc::new(Mutex::new(0));
    let ths: Vec<_> = (0..num)
        .map(|_| {
            let mutex = mutex.clone();
            spawn(move || {
                let mut lock = mutex.acquire();
                *lock += 1;
            })
        })
        .collect();

    for thread in ths {
        thread.join().unwrap();
    }

    let lock = mutex.acquire();

    assert_eq!(*lock, num)
}

struct Mutex<T> {
    is_acquired: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    fn new(data: T) -> Mutex<T> {
        Mutex {
            is_acquired: AtomicBool::default(),
            data: UnsafeCell::new(data),
        }
    }
    fn acquire(&self) -> MutexGuard<'_, T> {
        while !self.is_acquired.swap(true, Ordering::AcqRel) {
            spin_loop_hint()
        }
        MutexGuard { mutex: &self }
    }

    fn release(&self) {
        self.is_acquired.store(false, Ordering::Release);
    }
}

struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.release()
    }
}

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

unsafe impl<T> Send for MutexGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Send + Sync {}

fn main() {}

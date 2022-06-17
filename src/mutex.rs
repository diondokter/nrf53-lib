use core::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use crate::pac;

#[cfg(feature = "app")]
use pac::mutex_ns as mutex;
#[cfg(feature = "net")]
use pac::appmutex_ns as mutex;

pub struct Mutex<T> {
    peripheral: Option<mutex::MUTEX>,
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self { peripheral: None, inner: UnsafeCell::new(value) }
    }

    pub const fn new_with_peripheral(value: T, peripheral: mutex::MUTEX) -> Self {
        Self { peripheral: Some(peripheral), inner: UnsafeCell::new(value) }
    }

    pub fn assign_peripheral(&mut self, peripheral: mutex::MUTEX) -> Option<mutex::MUTEX> {
        self.peripheral.replace(peripheral)
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn borrow<'a>(&'a self) -> MutexGuard<'a, T> {
        let peripheral = self.peripheral.as_ref().unwrap();

        // Lock the mutex. We lock it by reading from it. When we read 0, it is now locked.
        while peripheral.read().mutex().is_locked() {}

        MutexGuard {
            peripheral,
            inner: unsafe { &mut *self.inner.get() },
        }
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    peripheral: &'a mutex::MUTEX,
    inner: &'a mut T,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.peripheral.write(|w| w.mutex().locked())
    }
}

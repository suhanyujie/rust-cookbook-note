use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait HandyRwLock<T> {
    fn wl(&self) -> RwLockWriteGuard<T>;
    fn rl(&self) -> RwLockReadGuard<T>;
}

impl<T> HandyRwLock<T> for RwLock<T> {
    #[inline]
    fn wl(&self) -> RwLockWriteGuard<T> {
        self.write().unwrap()
    }

    #[inline]
    fn rl(&self) -> RwLockReadGuard<T> {
        self.read().unwrap()
    }
}

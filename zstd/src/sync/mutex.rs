use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::time::Duration;
use zephyr::error::{Error, NegErrno};
use zephyr::time;

pub enum TryLockError {
    Timeout,
    WouldBlock,
}

type TryLockResult<Guard> = Result<Guard, TryLockError>;

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug)]
pub struct struct_k_mutex {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn zstd_impl_k_mutex_init() -> *const struct_k_mutex;
    fn zstd_impl_k_mutex_lock(
        mutex: *const struct_k_mutex,
        timeout: time::struct_k_timeout_t,
    ) -> i32;
    fn zstd_impl_k_mutex_unlock(mutex: *const struct_k_mutex) -> i32;
    fn zstd_impl_k_mutex_free(mutex: *const struct_k_mutex);
}
#[derive(Debug, Clone)]
pub struct RawMutex(pub(crate) *const struct_k_mutex);

impl RawMutex {
    fn new() -> Self {
        unsafe {
            let mutex = zstd_impl_k_mutex_init();
            if mutex.is_null() {
                panic!("Failed to allocate mutex");
            }
            Self(mutex)
        }
    }

    fn lock<T: Into<time::struct_k_timeout_t>>(
        &self,
        timeout: Option<T>,
    ) -> Result<(), Error> {
        let t: time::struct_k_timeout_t = match timeout {
            Some(timeout) => timeout.into(),
            None => time::K_FOREVER,
        };
        unsafe { zstd_impl_k_mutex_lock(self.0, t).maybe_zero() }
    }

    pub(crate) fn unlock(&self) -> Result<(), Error> {
        unsafe { zstd_impl_k_mutex_unlock(self.0).maybe_zero() }
    }
}

impl Drop for RawMutex {
    fn drop(&mut self) {
        unsafe { zstd_impl_k_mutex_free(self.0) }
    }
}

#[derive(Debug)]
pub struct Mutex<T: ?Sized> {
    pub(crate) inner: RawMutex,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
}

//impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        Mutex {
            inner: RawMutex::new(),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        match self.inner.lock(None::<Duration>) {
            Ok(_) => MutexGuard::new(&self),
            Err(e) => panic!("Failed to lock mutex: {:?}", e),
        }
    }

    pub fn try_lock_for<D: Into<time::struct_k_timeout_t>>(
        &self,
        timeout: D,
    ) -> TryLockResult<MutexGuard<'_, T>> {
        match self.inner.lock(Some(timeout)) {
            Ok(_) => Ok(MutexGuard::new(&self)),
            Err(Error::EBUSY) => Err(TryLockError::WouldBlock),
            Err(Error::EAGAIN) => Err(TryLockError::Timeout),
            Err(e) => panic!("Failed to lock mutex: {:?}", e),
        }
    }

    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        self.try_lock_for(time::K_NO_WAIT)
    }

    pub fn unlock(guard: MutexGuard<'_, T>) {
        drop(guard);
    }

    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.data.into_inner()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

impl<T> From<T> for Mutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    /// This is equivalent to [`Mutex::new`].
    fn from(t: T) -> Self {
        Mutex::new(t)
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    /// Creates a `Mutex<T>`, with the `Default` value for T.
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<'mutex, T: ?Sized> MutexGuard<'mutex, T> {
    fn new(lock: &'mutex Mutex<T>) -> MutexGuard<'mutex, T> {
        MutexGuard { lock }
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.inner.unlock().expect("unlock failed")
    }
}

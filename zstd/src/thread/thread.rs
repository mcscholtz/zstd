use zephyr::error::{Error, NegErrno};
use zephyr::time;

#[allow(non_camel_case_types)]
pub type struct_k_tid_t = *const struct_k_thread;

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct struct_k_thread {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct struct_k_thread_stack {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

pub type ThreadEntry = unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void);


extern "C" {
     fn zstd_impl_k_thread_stack_alloc(size: isize, flags: i32) -> *mut struct_k_thread_stack;
     fn zstd_impl_k_thread_stack_free(stack: *mut struct_k_thread_stack) -> i32;
     fn zstd_impl_k_thread_create(
        stack: *mut struct_k_thread_stack,
        stack_size: isize,
        entry: ThreadEntry,
        p1: *mut core::ffi::c_void,
        p2: *mut core::ffi::c_void,
        p3: *mut core::ffi::c_void,
        prio: i32,
        options: u32,
        delay: zephyr::time::struct_k_timeout_t) -> *mut struct_k_thread;
      fn zstd_impl_k_thread_free(thread: *mut struct_k_thread);
      fn zstd_impl_k_thread_join(thread: *mut struct_k_thread, timeout: zephyr::time::struct_k_timeout_t) -> i32;
}

extern "C" fn zstd_impl_thread_entry<F, T>(p1: *mut core::ffi::c_void, p2: *mut core::ffi::c_void, _p3: *mut core::ffi::c_void)
where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
    // re-construct the boxed closure & return value
    let closure: alloc::boxed::Box<F> = unsafe { alloc::boxed::Box::from_raw(p1 as *mut F) };
    let value: T = closure();

    // store the return value
    let return_ptr = p2 as *mut T;
    unsafe { return_ptr.write(value) };
}

pub struct JoinHandle<T> {
    thread: *mut struct_k_thread,
    stack: *mut struct_k_thread_stack,
    result: *mut core::ffi::c_void,
    _marker: core::marker::PhantomData<T>,
}

pub struct Builder {
    priority: i32,
    stack_size: usize,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            priority: 0,
            stack_size: 1024,
        }
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn stack_size(mut self, stack_size: usize) -> Self {
        self.stack_size = stack_size;
        self
    }

    pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: core::fmt::Debug + Send + 'static,
    {
        // TODO: Flags
        let stack = unsafe { zstd_impl_k_thread_stack_alloc(self.stack_size as isize, 2) };
        if stack.is_null() {
            return Err(Error::ENOMEM);
        }  

        // box the result placeholder
        let result = alloc::boxed::Box::new(core::mem::MaybeUninit::<T>::uninit() );
        let result_ptr = alloc::boxed::Box::into_raw(result);

        // box the closure
        let closure = alloc::boxed::Box::new(f);
        let closure_ptr = alloc::boxed::Box::into_raw(closure);

        let thread = unsafe {
            zstd_impl_k_thread_create(
                stack,
                self.stack_size as isize,
                zstd_impl_thread_entry::<F, T>,
                closure_ptr as *mut core::ffi::c_void,
                result_ptr as *mut core::ffi::c_void,
                0 as *mut core::ffi::c_void,
                self.priority,
                0,
                time::K_NO_WAIT,
            )
        };

        if thread.is_null() {
            unsafe { zstd_impl_k_thread_stack_free(stack) };
            return Err(Error::ENOMEM);
        }

        Ok(JoinHandle { 
            thread,
            stack,
            result: result_ptr as *mut core::ffi::c_void,
            _marker: core::marker::PhantomData,
        })
    }
}

pub fn spawn<F, T>(f: F) -> Result<JoinHandle<T>, Error>
where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: core::fmt::Debug + Send + 'static,
    {
    Builder::new().spawn(f)
}

impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T, Error> {
        if let Err(e) = unsafe { zstd_impl_k_thread_join(self.thread, time::K_FOREVER).maybe_zero() } {
            unsafe { zstd_impl_k_thread_stack_free(self.stack) };
            return Err(e);
        }

        // Now free the thread
        unsafe { zstd_impl_k_thread_free(self.thread) };
        // Now free the stack
        unsafe { zstd_impl_k_thread_stack_free(self.stack) };

        // get the resuling value
        let result = unsafe { alloc::boxed::Box::from_raw(self.result as *mut T) };
        
        return Ok(*result);
    }
}
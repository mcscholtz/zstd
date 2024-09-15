//use crate::kobject::{struct_k_msgq, AsKernelObject, KernelObject};
use core::ffi::c_void;
use core::fmt::Debug;
use zephyr::time;
use zephyr::error::*;
use zephyr::time::K_FOREVER;


#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug)]
pub struct struct_k_msgq {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

extern "C" {
    #[allow(dead_code)]
    // rust provides the buffer
    pub fn zstd_impl_k_msgq_new(msg_size: u32, msg_count: u32) -> *const struct_k_msgq;
    pub fn zstd_impl_k_msgq_put(
        msgq: *const struct_k_msgq,
        data: *const c_void,
        timeout: time::struct_k_timeout_t,
    ) -> i32;
    pub fn zstd_impl_k_msgq_get(
        msgq: *const struct_k_msgq,
        data: *mut c_void,
        timeout: time::struct_k_timeout_t,
    ) -> i32;
    pub fn zstd_impl_k_msgq_purge(msgq: *const struct_k_msgq) -> i32;
    pub fn zstd_impl_k_msgq_num_free_get(msgq: *const struct_k_msgq) -> i32;
    pub fn zstd_impl_k_msgq_num_used_get(msgq: *const struct_k_msgq) -> i32;
    pub fn zstd_impl_k_msgq_free(msgq: *const struct_k_msgq) -> i32;
}

#[derive(Debug, Clone, Copy)]
pub struct Sender<T> {
    pub(crate) msgq: *const struct_k_msgq,
    phantom: core::marker::PhantomData<T>,
}

//impl<T> AsKernelObject for Sender<T> {
//    fn as_kernel_object(&self) -> KernelObject {
//        KernelObject::MsgQ(self.msgq)
//    }
//}

unsafe impl<T: Sync> Sync for Sender<T> {}
unsafe impl<T: Send> Send for Sender<T> {}

#[derive(Debug, Clone, Copy)]
pub struct Receiver<T> {
    pub(crate) msgq: *const struct_k_msgq,
    phantom: core::marker::PhantomData<T>,
}

//impl<T> AsKernelObject for Receiver<T> {
//    fn as_kernel_object(&self) -> KernelObject {
//        KernelObject::MsgQ(self.msgq)
//    }
//}

unsafe impl<T: Sync> Sync for Receiver<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}

impl<T: Copy + Debug + Sized + Send> Sender<T> {
    pub fn send<TIMEOUT: Into<time::struct_k_timeout_t> + Copy>(
        &self,
        msg: &T,
        timeout: TIMEOUT,
    ) -> Result<(), Error> {
        let ptr: *const T = msg;
        unsafe { zstd_impl_k_msgq_put(self.msgq, ptr as *const c_void, timeout.into()).maybe_zero() }
    }

    pub fn blocking_send(
        &self,
        msg: &T,
    ) -> Result<(), Error> {
        self.send(msg, K_FOREVER)
    }
}

impl<T: Copy + Debug + Sized + Send> Receiver<T> {
    pub fn recv<TIMEOUT: Into<time::struct_k_timeout_t>>(&self, timeout: TIMEOUT) -> Result<T, Error> {
        unsafe {
            let mut value: T = core::mem::zeroed();
            let raw_ptr = &mut value as *mut T;
            zstd_impl_k_msgq_get(self.msgq, raw_ptr as *mut c_void, timeout.into()).maybe_zero()?;
            Ok(value)
        }
    }

    pub fn blocking_recv(&self) -> Result<T, Error> {
        self.recv(K_FOREVER)
    }
}

#[derive(Debug)]
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
    phantom: core::marker::PhantomData<T>,
}

unsafe impl<T: Sync> Sync for Channel<T> {}
unsafe impl<T: Send> Send for Channel<T> {}

impl<T: Copy + Debug + Sized + Send> Channel<T> {
    pub fn new(msg_count: usize) -> Result<Self, Error> {
        assert!(core::mem::align_of::<T>().is_power_of_two());
        // Create a static buffer on stack for the msgq
        let msgq: *const struct_k_msgq =
            unsafe { zstd_impl_k_msgq_new(core::mem::size_of::<T>() as u32, msg_count as u32) };
        Ok(Self {
            sender: Sender {
                msgq,
                phantom: core::marker::PhantomData,
            },
            receiver: Receiver {
                msgq,
                phantom: core::marker::PhantomData,
            },
            phantom: core::marker::PhantomData,
        })
    }

    pub fn send(&self, msg: &T, timeout: time::Duration) -> Result<(), Error> {
        self.sender.send(msg, timeout)
    }

    pub fn blocking_send(&self, msg: &T) -> Result<(), Error> {
        self.sender.blocking_send(msg)
    }

    pub fn recv(&self, timeout: time::Duration) -> Result<T, Error> {
        self.receiver.recv(timeout)
    }

    pub fn blocking_recv(&self) -> Result<T, Error> {
        self.receiver.blocking_recv()
    }

    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.sender, self.receiver)
    }
}

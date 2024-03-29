use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{
    AtomicUsize,
    Ordering::{Acquire, SeqCst},
};

const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C)]
pub struct KernelAllocator {
    usage: AtomicUsize,
}

extern "C" {
    //fn k_calloc(nmemb: usize, size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn aligned_alloc(align: usize, size: usize) -> *mut core::ffi::c_void;
    //fn malloc(size: usize) -> *mut core::ffi::c_void;
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        //let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return core::ptr::null_mut();
        }

        let ptr = aligned_alloc(align, size) as *mut u8;
        if ptr.is_null() {
            return core::ptr::null_mut();
        } else {
            self.usage.fetch_add(size, SeqCst);
            // TODO: alloc logging
            return ptr;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        free(ptr as *mut core::ffi::c_void);
        // TODO: dealloc logging
        self.usage.fetch_sub(layout.size(), SeqCst);
    }
}

impl KernelAllocator {
    pub const fn new() -> Self {
        Self {
            usage: AtomicUsize::new(0),
        }
    }

    pub fn usage(&self) -> usize {
        self.usage.load(Acquire)
    }
}

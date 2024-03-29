#![no_std]

extern crate alloc;

// TODO: This is not secure, a seperate heap should be created for the kernel and application
// linker sections will be needed for userspace
#[global_allocator]
static mut ALLOCATOR: zephyr::alloc::KernelAllocator = zephyr::alloc::KernelAllocator::new();


pub mod sync;
pub mod thread;

extern "C" {
     fn zstd_impl_printk(fmt: *const u8);
     fn zstd_impl_lock_scheduler();
     fn zstd_impl_unlock_scheduler();
     fn zstd_impl_compiler_barrier();
}


pub fn printk(fmt: &str) {
    unsafe { zstd_impl_printk(fmt.as_ptr()) };
}

pub fn lock_scheduler() {
    unsafe { zstd_impl_lock_scheduler() };
}

pub fn unlock_scheduler() {
    unsafe { zstd_impl_unlock_scheduler() };
}

pub fn compiler_barrier() {
    unsafe { zstd_impl_compiler_barrier() };
}
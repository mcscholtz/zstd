#![no_std]
use core::fmt::Write;

use alloc::sync::Arc;
use zstd::sync::mutex::Mutex;
use zstd::printk;

extern crate alloc;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // TODO: Add panic handler
    }
}


#[no_mangle]
pub extern "C" fn rust_test(a: i32, b: i32) -> i32 {
    printk("Hello from Rust!\n\0");
    
    let mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let thread1_mutex = mutex.clone();
    let thread2_mutex = mutex.clone();

    printk("spawining 2 threads from rust\n\0");
    // test spawning a thread
    let handle1 = zstd::thread::spawn(move || {
        printk("entered dynamically allocated rust thread #1....\n\0");
        let mut lock = thread1_mutex.lock();
        *lock += a;
        a
    }).expect("failed to spawn thread");

    

    let handle2 = zstd::thread::spawn(move || {
        printk("entered dynamically allocated rust thread #2....\n\0");
        let mut lock = thread2_mutex.lock();
        *lock += b;
        b
    }).expect("failed to spawn thread");

    let x = handle1.join().expect("failed to join thread");
    let y = handle2.join().expect("failed to join thread");
    let z = *mutex.lock();

    // TODO: better way to print
    let mut buf = heapless::String::<128>::new();
    core::write!(&mut buf, "Thread 1 returned: {}\n\0", x).unwrap();
    printk(&buf);
    buf.clear();
    core::write!(&mut buf, "Thread 2 returned: {}\n\0", y).unwrap();
    printk(&buf);
    buf.clear();
    core::write!(&mut buf, "Mutex value: {}\n\0", z).unwrap();
    printk(&buf);

    x + y
}

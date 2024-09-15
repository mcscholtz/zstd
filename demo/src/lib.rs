#![no_std]
use alloc::sync::Arc;
use zstd::{error, info};
use zstd::sync::mutex::Mutex;
use zstd::module;

extern crate alloc;


module!(rust, TerminalColor::DarkGreen);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {
        error!("Panic: {:#?}", info);
        zstd::thread::sleep(core::time::Duration::from_secs(1));
    }
}


#[no_mangle]
pub extern "C" fn rust_test(a: i32, b: i32) -> i32 {
    info!("Hello from Rust!");
    
    let mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let thread1_mutex = mutex.clone();
    let thread2_mutex = mutex.clone();

    info!("spawining 2 threads from rust");
    // test spawning a thread
    let handle1 = zstd::thread::spawn(move || {
        info!("entered dynamically allocated rust thread #1....");
        let mut lock = thread1_mutex.lock();
        *lock += a;
        a
    }).expect("failed to spawn thread");


    let handle2 = zstd::thread::spawn(move || {
        info!("entered dynamically allocated rust thread #2....");
        let mut lock = thread2_mutex.lock();
        *lock += b;
        b
    }).expect("failed to spawn thread");

    let x = if let Ok(value) = handle1.join() {
        info!("Thread 1 returned: {}", value);
        value
    } else {
        panic!("Failed to join thread 1")
    };

    let y = if let Ok(value) = handle2.join() {
        info!("Thread 2 returned: {}", value);
        value
    } else {
        panic!("Failed to join thread 2")
    };

    info!("Mutex value: {}",  *mutex.lock());

    panic!("we are having a panic!");
    x + y
}

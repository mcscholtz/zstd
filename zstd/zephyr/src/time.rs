pub use core::time::Duration;


extern "C" {
    fn zephyr_k_get_uptime() -> i64;
    //fn zephyr_k_get_cycles() -> u32;
    //fn zephyr_k_clock_freq() -> u64;
    fn zephyr_k_timeout_from_ms(ms: u32) -> struct_k_timeout_t;
    fn zephyr_k_timeout_from_us(us: u32) -> struct_k_timeout_t;
    fn zephyr_k_timeout_from_ns(ns: u32) -> struct_k_timeout_t;
}

#[allow(non_camel_case_types)]
pub type k_ticks = u32;

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct struct_k_timeout_t {
    pub ticks: k_ticks,
}

impl From<Duration> for struct_k_timeout_t {
    fn from(duration: Duration) -> Self {
        let nanos = duration.as_nanos() as u64;
        if nanos > u32::MAX as u64 {
            let us = duration.as_micros() as u64;
            if us > u32::MAX as u64 {
                let ms = duration.as_millis() as u64;
                if ms > u32::MAX as u64 {
                    K_FOREVER
                } else {
                    unsafe { zephyr_k_timeout_from_ms(ms as u32) }
                }
            } else {
                unsafe { zephyr_k_timeout_from_us(us as u32) }
            }
        } else {
            unsafe { zephyr_k_timeout_from_ns(nanos as u32) }
        }
    }
}

pub const K_FOREVER: struct_k_timeout_t = struct_k_timeout_t { ticks: u32::MAX };
pub const K_NO_WAIT: struct_k_timeout_t = struct_k_timeout_t { ticks: 0 };

#[inline(always)]
pub fn uptime() -> Duration {
    let time = unsafe { zephyr_k_get_uptime() };
    Duration::from_millis(time as u64)
}

#[inline(always)]
pub fn uptime_u32() -> u32 {
    let time = unsafe { zephyr_k_get_uptime() };
    time as u32
}

#[inline(always)]
pub fn elapsed_since(start: Duration) -> Duration {
    uptime() - start
}

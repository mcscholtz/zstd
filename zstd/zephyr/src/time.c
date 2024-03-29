#include <zephyr/kernel.h>

int64_t zephyr_k_get_uptime() {
	return k_uptime_get();
}

uint32_t zephyr_k_get_cycles() {
	return k_cycle_get_32();
}

uint64_t zephyr_k_clock_freq() {
	return sys_clock_hw_cycles_per_sec();
}

uint32_t zephyr_k_sys_tick_hz() {
	return CONFIG_SYS_CLOCK_TICKS_PER_SEC;
}

k_timeout_t zephyr_k_timeout_from_ms(uint32_t ms)
{
	return Z_TIMEOUT_MS(ms);
}

k_timeout_t zephyr_k_timeout_from_us(uint32_t us)
{
	return Z_TIMEOUT_US(us);
}

k_timeout_t zephyr_k_timeout_from_ns(uint32_t ns)
{
	return Z_TIMEOUT_NS(ns);
}


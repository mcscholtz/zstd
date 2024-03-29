#include <zephyr/kernel.h>

void zstd_impl_printk(const char* msg) {
    printk("%s\r\n", msg);
}


#include <zephyr/kernel.h>

//__syscall k_thread_stack_t *k_thread_stack_alloc(size_t size, int flags);
k_thread_stack_t *zstd_impl_k_thread_stack_alloc(size_t size, int flags) {
    k_thread_stack_t *stack = k_thread_stack_alloc(size, flags);
    printk("zstd_impl_k_thread_stack_alloc: %p\r\n", stack);
    return stack;
}

//__syscall int k_thread_stack_free(k_thread_stack_t *stack);
int zstd_impl_k_thread_stack_free(k_thread_stack_t *stack) {
    return k_thread_stack_free(stack);
}

//__syscall k_tid_t k_thread_create(struct k_thread *new_thread,
//				  k_thread_stack_t *stack,
//				  size_t stack_size,
//				  k_thread_entry_t entry,
//				  void *p1, void *p2, void *p3,
//				  int prio, uint32_t options, k_timeout_t delay);
struct k_thread * zstd_impl_k_thread_create(
                  k_thread_stack_t *stack,
                  size_t stack_size,
                  k_thread_entry_t entry,
                  void *p1, void *p2, void *p3,
                  int prio, uint32_t options, k_timeout_t delay) {
    struct k_thread *new_thread = k_malloc(sizeof(struct k_thread));
    if (new_thread == NULL) {
        return NULL;
    }
    return k_thread_create(new_thread, stack, stack_size, entry, p1, p2, p3, prio, options, delay);
}

void zstd_impl_k_thread_free(struct k_thread *thread) {
    k_free(thread);
    thread = NULL;
}

//__syscall int k_thread_join(struct k_thread *thread, k_timeout_t timeout);
int zstd_impl_k_thread_join(struct k_thread *thread, k_timeout_t timeout) {
    return k_thread_join(thread, timeout);
}


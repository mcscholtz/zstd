
#include <zephyr/kernel.h>

k_thread_stack_t *zstd_impl_k_thread_stack_alloc(size_t size, int flags) {
    k_thread_stack_t *stack = k_thread_stack_alloc(size, flags);
    return stack;
}

int zstd_impl_k_thread_stack_free(k_thread_stack_t *stack) {
    return k_thread_stack_free(stack);
}

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

int zstd_impl_k_thread_join(struct k_thread *thread, k_timeout_t timeout) {
    return k_thread_join(thread, timeout);
}

void zstd_impl_k_thread_abort() {
	k_thread_abort(k_current_get());
}

void zstd_impl_k_thread_usleep(uint32_t us)
{
	k_usleep(us);
}

void zstd_impl_k_thread_yield() {
	k_yield();
}
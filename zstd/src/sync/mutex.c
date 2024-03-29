
#include <zephyr/kernel.h>

struct k_mutex * zstd_impl_k_mutex_init() {
	// TODO: Probably want to use k_object_alloc instead if using userspace
	//struct k_mutex *mutex = k_object_alloc(K_OBJ_MUTEX);
	struct k_mutex *mutex = k_malloc(sizeof(struct k_mutex));

	if (mutex == NULL) {
		return NULL;
	}

	k_mutex_init(mutex);
	return mutex;
}

int zstd_impl_k_mutex_lock(struct k_mutex *mutex, k_timeout_t timeout) {
    return k_mutex_lock(mutex, timeout);
}

int zstd_impl_k_mutex_unlock(struct k_mutex *mutex) {
    return k_mutex_unlock(mutex);
}

void zstd_impl_k_mutex_free(struct k_mutex *mutex) {
	//k_object_free(mutex);
	k_free(mutex);
	mutex = NULL;
}

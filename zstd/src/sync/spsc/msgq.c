
#include <zephyr/kernel.h>
#include <zephyr/init.h>


struct k_msgq * zstd_impl_k_msgq_new(uint32_t msg_size, uint32_t msg_count) {
	//struct k_msgq *msgq = k_object_alloc(K_OBJ_MSGQ);
	struct k_msgq *msgq = k_malloc(sizeof(struct k_msgq));
	if (msgq == NULL) {
		return NULL;
	}

	if (0 > k_msgq_alloc_init(msgq, msg_size, msg_count)) {
		k_object_free(msgq);
		return NULL;
	}
	return msgq;
}

int zstd_impl_k_msgq_put(struct k_msgq * msgq, const void *buffer, k_timeout_t timeout) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
	return k_msgq_put(msgq, buffer, timeout);
}

int zstd_impl_k_msgq_get(struct k_msgq * msgq, void *buffer, k_timeout_t timeout) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
    return k_msgq_get(msgq, buffer, timeout);
}

void zstd_impl_k_msgq_purge(struct k_msgq * msgq) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
	k_msgq_purge(msgq);
}

uint32_t zstd_impl_k_msgq_num_free(struct k_msgq * msgq) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
	return k_msgq_num_free_get(msgq);
}

uint32_t zstd_impl_k_msgq_num_used(struct k_msgq * msgq) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
	return k_msgq_num_used_get(msgq);
}

void zstd_impl_k_msgq_free(struct k_msgq * msgq) {
	__ASSERT(msgq != NULL, "invalid msgq ptr");
	k_msgq_cleanup(msgq);
	//k_object_free(msgq);
    k_free(msgq);
}
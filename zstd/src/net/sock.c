#include <zephyr/kernel.h>
#include <zephyr/net/socket.h>
#include <stdio.h>

struct sockaddr * zstd_impl_sock_new_addr4() {
	struct sockaddr_in *addr4 = k_malloc(sizeof(struct sockaddr_in));
	if (!addr4) {
		return NULL;
	}
	(void)memset(addr4, 0, sizeof(struct sockaddr_in));
	return (struct sockaddr *)addr4;
}


const size_t zstd_impl_sock_addr4_len() {
	return sizeof(struct sockaddr_in);
}

const size_t zstd_impl_sock_addr6_len() {
	return sizeof(struct sockaddr_in6);
}

void zstd_impl_sock_free_addr(struct sockaddr *addr) {
	k_free(addr);
}

int zstd_impl_sock_set_addr4(struct sockaddr *addr, const char *address, int16_t port) {
	struct sockaddr_in *addr4 = (struct sockaddr_in *)addr;
	addr4->sin_family = AF_INET;
	addr4->sin_port = htons(port);
	return net_addr_pton(AF_INET, address, &addr4->sin_addr);
}

int zstd_impl_sock_socket(int family, int type, int proto)
{
	return zsock_socket(family, type, proto);
}

//int zstd_impl_sock_socketpair(int family, int type, int proto, int sv[2])
//{
//	return zsock_socketpair(family, type, proto, sv);
//}

int zstd_impl_sock_close(int sock)
{
	return zsock_close(sock);
}

int zstd_impl_sock_shutdown(int sock, int how)
{
	return zsock_shutdown(sock, how);
}

int zstd_impl_sock_bind(int sock, const struct sockaddr *addr, socklen_t addrlen)
{
	return zsock_bind(sock, addr, addrlen);
}

int zstd_impl_sock_connect(int sock, const struct sockaddr *addr,
			  socklen_t addrlen)
{
	return zsock_connect(sock, addr, addrlen);
}

int zstd_impl_sock_listen(int sock, int backlog)
{
	return zsock_listen(sock, backlog);
}

int zstd_impl_sock_accept(int sock, struct sockaddr *addr, socklen_t *addrlen)
{
	return zsock_accept(sock, addr, addrlen);
}

ssize_t zstd_impl_sock_send(int sock, const void *buf, size_t len, int flags)
{
	return zsock_send(sock, buf, len, flags);
}

ssize_t zstd_impl_sock_recv(int sock, void *buf, size_t max_len, int flags)
{
	return zsock_recv(sock, buf, max_len, flags);
}

/*
ssize_t zstd_impl_sock_sendto(int sock, const void *buf, size_t len, int flags,
			     const struct sockaddr *dest_addr,
			     socklen_t addrlen)
{
	return zsock_sendto(sock, buf, len, flags, dest_addr, addrlen);
}

ssize_t zstd_impl_sock_sendmsg(int sock, const struct msghdr *message,
			      int flags)
{
	return zsock_sendmsg(sock, message, flags);
}

ssize_t zstd_impl_sock_recvfrom(int sock, void *buf, size_t max_len, int flags,
			       struct sockaddr *src_addr, socklen_t *addrlen)
{
	return zsock_recvfrom(sock, buf, max_len, flags, src_addr, addrlen);
}

ssize_t zstd_impl_sock_recvmsg(int sock, struct msghdr *msg, int flags)
{
	return zsock_recvmsg(sock, msg, flags);
}

int zstd_impl_sock_poll(struct zsock_pollfd *fds, int nfds, int timeout)
{
	return zsock_poll(fds, nfds, timeout);
}

int zstd_impl_sock_getsockopt(int sock, int level, int optname,
			     void *optval, socklen_t *optlen)
{
	return zsock_getsockopt(sock, level, optname, optval, optlen);
}

int zstd_impl_sock_setsockopt(int sock, int level, int optname,
			     const void *optval, socklen_t optlen)
{
	return zsock_setsockopt(sock, level, optname, optval, optlen);
}

int zstd_impl_sock_getpeername(int sock, struct sockaddr *addr,
			      socklen_t *addrlen)
{
	return zsock_getpeername(sock, addr, addrlen);
}

int zstd_impl_sock_getsockname(int sock, struct sockaddr *addr,
			      socklen_t *addrlen)
{
	return zsock_getsockname(sock, addr, addrlen);
}

int zstd_impl_sock_getaddrinfo(const char *host, const char *service,
			      const struct zsock_addrinfo *hints,
			      struct zsock_addrinfo **res)
{
	return zsock_getaddrinfo(host, service, hints, res);
}

void zstd_impl_sock_freeaddrinfo(struct zsock_addrinfo *ai)
{
	zsock_freeaddrinfo(ai);
}
*/
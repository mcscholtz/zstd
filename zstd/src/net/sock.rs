use crate::fd;
use zephyr::error::*;

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug)]
struct struct_sockaddr {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn zstd_impl_sock_addr4_len() -> usize;
    fn zstd_impl_sock_addr6_len() -> usize;
    fn zstd_impl_sock_new_addr4() -> *mut struct_sockaddr;
    fn zstd_impl_sock_set_addr4(addr: *mut struct_sockaddr, address: *const core::ffi::c_char, port: u16) -> i32;
    fn zstd_impl_sock_free_addr(addr: *const struct_sockaddr);

    fn zstd_impl_sock_socket(family: i32, type_: i32, proto: i32) -> i32;
    fn zstd_impl_sock_close(sock: i32);
    fn zstd_impl_sock_shutdown(sock: i32, how: i32);
    fn zstd_impl_sock_bind(sock: i32, addr: *const struct_sockaddr, addrlen: usize) -> i32;
    //fn zstd_impl_sock_connect(sock: i32, addr: *const struct_sockaddr, addrlen: usize) -> i32;
    fn zstd_impl_sock_listen(sock: i32, backlog: i32) -> i32;
    fn zstd_impl_sock_accept(sock: i32, addr: *mut struct_sockaddr, addrlen: *mut usize) -> i32;
    fn zstd_impl_sock_send(sock: i32, buf: *const u8, len: usize, flags: i32) -> isize;
    fn zstd_impl_sock_recv(sock: i32, buf: *mut u8, max_len: usize, flags: i32) -> isize;
}

#[repr(i8)]
pub enum AddressFamily {
    Unspec = 0,
    Inet = 1,
    Inet6 = 2,
    Packet = 3,
    Can = 4,
    NetMgmt = 5,
    Unix = 6
}

impl Into<i32> for AddressFamily {
    fn into(self) -> i32 {
        self as i32
    }
}


#[repr(u8)]
pub enum IpProtocol {
    Ip = 0,
    Icmp = 1,
    Igmp = 2,
    Ipip = 4,
    Tcp = 6,
    Udp = 17,
    Ipv6 = 41,
    Icmpv6 = 58,
    Raw = 255
}

impl Into<i32>for IpProtocol {
    fn into(self) -> i32 {
        self as i32
    }
}

#[repr(u8)]
pub enum SocketType {
    Stream = 1,
    Dgram,
    Raw
}

impl Into<i32> for SocketType {
    fn into(self) -> i32 {
        self as i32
    }
}

#[repr(u8)]
enum ShutdownType {
    Read = 0,
    Write,
    Both
}

impl Into<i32> for ShutdownType {
    fn into(self) -> i32 {
        self as i32
    }
}


pub struct SocketAddrV4(*mut struct_sockaddr);

impl SocketAddrV4 {
    pub fn new(addr: &str) -> Result<SocketAddr, Error> {
        // split by : into <addr> <port>
        let split = addr.replace(":", "\0");
        let mut iter = split.split_inclusive('\0');
        let addr = match iter.next() {
            None => return Err(Error::EINVAL),
            Some(addr) => addr
        };
        let port = match iter.next() {
            None => return Err(Error::EINVAL),
            Some(port) => match port.parse::<u16>() {
                Err(_) =>  return Err(Error::EINVAL),
                Ok(port) => port
            }
        };
        
        if iter.next().is_some() {
            return Err(Error::EINVAL)
        }

        let sock_addr = unsafe {
            zstd_impl_sock_new_addr4()
        };

        if sock_addr.is_null() {
            return Err(Error::EINVAL)
        }

        unsafe {
            zstd_impl_sock_set_addr4(sock_addr, addr.as_ptr() as *const i8, port).maybe_zero()?  
        }

        Ok(SocketAddr::V4(SocketAddrV4(sock_addr)))
    }

    fn from_raw(addr: *mut struct_sockaddr, _len: usize) -> Result<SocketAddr, Error> {
        // parse the len or something???
        Ok(SocketAddr::V4(SocketAddrV4(addr)))
    }
}

impl Drop for SocketAddrV4 {
    fn drop(&mut self) {
        unsafe {
            zstd_impl_sock_free_addr(self.0)
        }
    }
}
pub struct SocketAddrV6(*mut struct_sockaddr);

pub enum SocketAddr {
    V4(SocketAddrV4),
    V6(SocketAddrV6),
}

impl SocketAddr {
    fn as_raw(&self) -> (*const struct_sockaddr, usize) {
        match self {
            SocketAddr::V4(addr) => {
                (addr.0, unsafe { zstd_impl_sock_addr4_len() })
            },
            SocketAddr::V6(addr) => {
                (addr.0, unsafe { zstd_impl_sock_addr6_len() })
            },
        }
    }
}

pub struct TcpListener {
    fd: fd::Fd,
    _addr: SocketAddr
}

impl TcpListener {
    pub fn bind(address: &str) -> Result<Self, Error> {
        let addr = SocketAddrV4::new(address)?;


        let fd = socket(
            AddressFamily::Inet, 
            SocketType::Stream, 
            IpProtocol::Tcp
        )?;

        bind(&fd, &addr)?;

        listen(&fd, 1)?;

        Ok(Self {
            fd,
            _addr: addr
        })
    }

    pub fn accept(&self) -> Result<TcpStream, Error> {
        let (fd, addr) = accept(&self.fd)?;
        Ok(TcpStream {
            fd, 
            _addr: addr
        })
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        shutdown(&self.fd, ShutdownType::Both);
        close(&self.fd)
    }
}

pub struct TcpStream {
    fd: fd::Fd,
    _addr: SocketAddr
}

impl TcpStream {
    pub fn close(&self) {
        close(&self.fd)
    }

    pub fn recv<'a, const N: usize>(&self, buf: &'a mut [u8; N]) -> Result<&'a [u8], Error> {
        recv(&self.fd, buf, 0)
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize, Error> {
        send(&self.fd, buf, 0)
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        self.close()
    }
}

fn socket(family: AddressFamily , type_: SocketType, proto: IpProtocol) -> Result<fd::Fd, Error>  {
    unsafe {
        fd::Fd::try_from(zstd_impl_sock_socket(family.into(), type_.into(), proto.into()))
    }
}

fn close(sock: &fd::Fd) {
    unsafe {
        zstd_impl_sock_close(sock.as_raw())
    }
}


fn shutdown(sock: &fd::Fd, how: ShutdownType) {
    unsafe {
        zstd_impl_sock_shutdown(sock.as_raw(), how.into());
    }
}


fn bind(sock: &fd::Fd, addr: &SocketAddr) -> Result<(), Error> {
    let (addr, addrlen) = addr.as_raw();
    unsafe {
        zstd_impl_sock_bind(sock.as_raw(), addr, addrlen).maybe_zero()
    }
}

/* 
fn connect(sock: &fd::Fd, addr: &SocketAddr) -> Result<(), Error> {
    let (addr, addrlen) = addr.as_raw();
    unsafe {
        zstd_impl_sock_connect(sock.as_raw(), addr, addrlen).maybe_zero()
    }
}
*/

fn listen(sock: &fd::Fd, backlog: i32) -> Result<(), Error>  {
    unsafe {
        zstd_impl_sock_listen(sock.as_raw(), backlog).maybe_zero()
    }
}

fn accept(sock: &fd::Fd) -> Result<(fd::Fd, SocketAddr), Error> {
    // This probably needs either a seperate call for Ip4 or we just do not support ip6 for now
    let peer_addr = unsafe {
        zstd_impl_sock_new_addr4()
    };
    let mut addrlen = unsafe { zstd_impl_sock_addr4_len() };
    let peer = unsafe {
        fd::Fd::try_from(zstd_impl_sock_accept(sock.as_raw(), peer_addr, &mut addrlen as *mut usize))?
    };

    let addr = SocketAddrV4::from_raw(peer_addr, addrlen)?;

    Ok((peer, addr))

}

fn send(sock: &fd::Fd, buf: &[u8], flags: i32) -> Result<usize, Error> {
    unsafe {
        zstd_impl_sock_send(sock.as_raw(), buf.as_ptr(), buf.len(), flags).maybe_usize()
    }
}

fn recv<'a, const N: usize>(sock: &fd::Fd, buf: &'a mut [u8; N], flags: i32) -> Result<&'a [u8], Error> {
    
    let len = unsafe {
        zstd_impl_sock_recv(sock.as_raw(), buf.as_mut_ptr(), N, flags).maybe_usize()?
    };

    // set the size of the slice
    Ok(&buf[..len])
}

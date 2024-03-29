
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Not owner
    EPERM = 1,
    /// No such file or directory
    ENOENT,
    /// No such context
    ESRCH,
    /// Interrupted system call
    EINTR,
    /// I/O error
    EIO,
    /// No such device or address
    ENXIO,
    /// Arg list too long
    E2BIG,
    /// Exec format error
    ENOEXEC,
    /// Bad file number
    EBADF,
    /// No children
    ECHILD,
    /// No more contexts
    EAGAIN,
    /// Not enough core
    ENOMEM,
    /// Permission denied
    EACCES,
    /// Bad address
    EFAULT,
    /// Block device required
    ENOTBLK,
    /// Mount device busy
    EBUSY,
    /**< Mount device busy */
    EEXIST,
    /**< File exists */
    EXDEV,
    /**< Cross-device link */
    ENODEV,
    /**< No such device */
    ENOTDIR,
    /**< Not a directory */
    EISDIR,
    /**< Is a directory */
    EINVAL,
    /**< Invalid argument */
    ENFILE,
    /**< File table overflow */
    EMFILE,
    /**< Too many open files */
    ENOTTY,
    /**< Not a typewriter */
    ETXTBSY,
    /**< Text file busy */
    EFBIG,
    /**< File too large */
    ENOSPC,
    /**< No space left on device */
    ESPIPE,
    /**< Illegal seek */
    EROFS,
    /**< Read-only file system */
    EMLINK,
    /**< Too many links */
    EPIPE,
    /**< Broken pipe */
    EDOM,
    /**< Argument too large */
    ERANGE,
    /**< Result too large */
    ENOMSG,
    /**< Unexpected message type */
    EDEADLK,
    /**< Resource deadlock avoided */
    ENOLCK,
    /**< No locks available */
    ENOSTR,
    /**< STREAMS device required */
    ENODATA,
    /**< Missing expected message data */
    ETIME,
    /**< STREAMS timeout occurred */
    ENOSR,
    /**< Insufficient memory */
    EPROTO,
    /**< Generic STREAMS error */
    EBADMSG,
    /**< Invalid STREAMS message */
    ENOSYS,
    /**< Function not implemented */
    ENOTEMPTY,
    /**< Directory not empty */
    ENAMETOOLONG,
    /**< File name too long */
    ELOOP,
    /**< Too many levels of symbolic links */
    EOPNOTSUPP,
    /**< Operation not supported on socket */
    EPFNOSUPPORT,
    /**< Protocol family not supported */
    ECONNRESET,
    /**< Connection reset by peer */
    ENOBUFS,
    /**< No buffer space available */
    EAFNOSUPPORT,
    /**< Addr family not supported */
    EPROTOTYPE,
    /**< Protocol wrong type for socket */
    ENOTSOCK,
    /**< Socket operation on non-socket */
    ENOPROTOOPT,
    /**< Protocol not available */
    ESHUTDOWN,
    /**< Can't send after socket shutdown */
    ECONNREFUSED,
    /**< Connection refused */
    EADDRINUSE,
    /**< Address already in use */
    ECONNABORTED,
    /**< Software caused connection abort */
    ENETUNREACH,
    /**< Network is unreachable */
    ENETDOWN,
    /**< Network is down */
    ETIMEDOUT,
    /**< Connection timed out */
    EHOSTDOWN,
    /**< Host is down */
    EHOSTUNREACH,
    /**< No route to host */
    EINPROGRESS,
    /**< Operation now in progress */
    EALREADY,
    /**< Operation already in progress */
    EDESTADDRREQ,
    /**< Destination address required */
    EMSGSIZE,
    /**< Message size */
    EPROTONOSUPPORT,
    /**< Protocol not supported */
    ESOCKTNOSUPPORT,
    /**< Socket type not supported */
    EADDRNOTAVAIL,
    /**< Can't assign requested address */
    ENETRESET,
    /**< Network dropped connection on reset */
    EISCONN,
    /**< Socket is already connected */
    ENOTCONN,
    /**< Socket is not connected */
    ETOOMANYREFS,
    /**< Too many references: can't splice */
    ENOTSUP,
    /**< Unsupported value */
    EILSEQ,
    /**< Illegal byte sequence */
    EOVERFLOW,
    /**< Value overflow */
    ECANCELED,
    /**< Unknown */
    EUNKNOWN,
}

impl From<Error> for u32 {
    fn from(err: Error) -> u32 {
        err as u32
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u32> for Error {
    fn from(x: u32) -> Error {
        match x {
            x if x == Error::EPERM as u32 => Error::EPERM,
            x if x == Error::ENOENT as u32 => Error::ENOENT,
            x if x == Error::ESRCH as u32 => Error::ESRCH,
            x if x == Error::EINTR as u32 => Error::EINTR,
            x if x == Error::EIO as u32 => Error::EIO,
            x if x == Error::ENXIO as u32 => Error::ENXIO,
            x if x == Error::E2BIG as u32 => Error::E2BIG,
            x if x == Error::ENOEXEC as u32 => Error::ENOEXEC,
            x if x == Error::EBADF as u32 => Error::EBADF,
            x if x == Error::ECHILD as u32 => Error::ECHILD,
            x if x == Error::EAGAIN as u32 => Error::EAGAIN,
            x if x == Error::ENOMEM as u32 => Error::ENOMEM,
            x if x == Error::EACCES as u32 => Error::EACCES,
            x if x == Error::EFAULT as u32 => Error::EFAULT,
            x if x == Error::ENOTBLK as u32 => Error::ENOTBLK,
            x if x == Error::EBUSY as u32 => Error::EBUSY,
            x if x == Error::EEXIST as u32 => Error::EEXIST,
            x if x == Error::EXDEV as u32 => Error::EXDEV,
            x if x == Error::ENODEV as u32 => Error::ENODEV,
            x if x == Error::ENOTDIR as u32 => Error::ENOTDIR,
            x if x == Error::EISDIR as u32 => Error::EISDIR,
            x if x == Error::EINVAL as u32 => Error::EINVAL,
            x if x == Error::ENFILE as u32 => Error::ENFILE,
            x if x == Error::EMFILE as u32 => Error::EMFILE,
            x if x == Error::ENOTTY as u32 => Error::ENOTTY,
            x if x == Error::ETXTBSY as u32 => Error::ETXTBSY,
            x if x == Error::EFBIG as u32 => Error::EFBIG,
            x if x == Error::ENOSPC as u32 => Error::ENOSPC,
            x if x == Error::ESPIPE as u32 => Error::ESPIPE,
            x if x == Error::EROFS as u32 => Error::EROFS,
            x if x == Error::EMLINK as u32 => Error::EMLINK,
            x if x == Error::EPIPE as u32 => Error::EPIPE,
            x if x == Error::EDOM as u32 => Error::EDOM,
            x if x == Error::ERANGE as u32 => Error::ERANGE,
            x if x == Error::ENOMSG as u32 => Error::ENOMSG,
            x if x == Error::EDEADLK as u32 => Error::EDEADLK,
            x if x == Error::ENOLCK as u32 => Error::ENOLCK,
            x if x == Error::ENOSTR as u32 => Error::ENOSTR,
            x if x == Error::ENODATA as u32 => Error::ENODATA,
            x if x == Error::ETIME as u32 => Error::ETIME,
            x if x == Error::ENOSR as u32 => Error::ENOSR,
            x if x == Error::EPROTO as u32 => Error::EPROTO,
            x if x == Error::EBADMSG as u32 => Error::EBADMSG,
            x if x == Error::ENOSYS as u32 => Error::ENOSYS,
            x if x == Error::ENOTEMPTY as u32 => Error::ENOTEMPTY,
            x if x == Error::ENAMETOOLONG as u32 => Error::ENAMETOOLONG,
            x if x == Error::ELOOP as u32 => Error::ELOOP,
            x if x == Error::EOPNOTSUPP as u32 => Error::EOPNOTSUPP,
            x if x == Error::EPFNOSUPPORT as u32 => Error::EPFNOSUPPORT,
            x if x == Error::ECONNRESET as u32 => Error::ECONNRESET,
            x if x == Error::ENOBUFS as u32 => Error::ENOBUFS,
            x if x == Error::EAFNOSUPPORT as u32 => Error::EAFNOSUPPORT,
            x if x == Error::EPROTOTYPE as u32 => Error::EPROTOTYPE,
            x if x == Error::ENOTSOCK as u32 => Error::ENOTSOCK,
            x if x == Error::ENOPROTOOPT as u32 => Error::ENOPROTOOPT,
            x if x == Error::ESHUTDOWN as u32 => Error::ESHUTDOWN,
            x if x == Error::ECONNREFUSED as u32 => Error::ECONNREFUSED,
            x if x == Error::EADDRINUSE as u32 => Error::EADDRINUSE,
            x if x == Error::ECONNABORTED as u32 => Error::ECONNABORTED,
            x if x == Error::ENETUNREACH as u32 => Error::ENETUNREACH,
            x if x == Error::ENETDOWN as u32 => Error::ENETDOWN,
            x if x == Error::ETIMEDOUT as u32 => Error::ETIMEDOUT,
            x if x == Error::EHOSTDOWN as u32 => Error::EHOSTDOWN,
            x if x == Error::EHOSTUNREACH as u32 => Error::EHOSTUNREACH,
            x if x == Error::EINPROGRESS as u32 => Error::EINPROGRESS,
            x if x == Error::EALREADY as u32 => Error::EALREADY,
            x if x == Error::EDESTADDRREQ as u32 => Error::EDESTADDRREQ,
            x if x == Error::EMSGSIZE as u32 => Error::EMSGSIZE,
            x if x == Error::EPROTONOSUPPORT as u32 => Error::EPROTONOSUPPORT,
            x if x == Error::ESOCKTNOSUPPORT as u32 => Error::ESOCKTNOSUPPORT,
            x if x == Error::EADDRNOTAVAIL as u32 => Error::EADDRNOTAVAIL,
            x if x == Error::ENETRESET as u32 => Error::ENETRESET,
            x if x == Error::EISCONN as u32 => Error::EISCONN,
            x if x == Error::ENOTCONN as u32 => Error::ENOTCONN,
            x if x == Error::ETOOMANYREFS as u32 => Error::ETOOMANYREFS,
            x if x == Error::ENOTSUP as u32 => Error::ENOTSUP,
            x if x == Error::EILSEQ as u32 => Error::EILSEQ,
            x if x == Error::EOVERFLOW as u32 => Error::EOVERFLOW,
            x if x == Error::ECANCELED as u32 => Error::ECANCELED,
            _ => Error::EUNKNOWN,
        }
    }
}

pub trait NegErr {
    fn neg_err(&self) -> Result<u32, u32>;
}

impl NegErr for i32 {
    fn neg_err(&self) -> Result<u32, u32> {
        if *self >= 0 {
            Ok(*self as u32)
        } else {
            Err((-*self) as u32)
        }
    }
}

impl NegErr for isize {
    fn neg_err(&self) -> Result<u32, u32> {
        if *self >= 0 {
            Ok(*self as u32)
        } else {
            Err((-*self) as u32)
        }
    }
}

pub trait NegErrno: NegErr {
    fn maybe_u32(&self) -> Result<u32, Error>;
    fn maybe_i32(&self) -> Result<i32, Error>;
    fn maybe_u8(&self) -> Result<u8, Error>;
    fn maybe_zero(&self) -> Result<(), Error>;
    fn maybe_usize(&self) -> Result<usize, Error>;
    fn maybe_isize(&self) -> Result<isize, Error>;
}

impl NegErrno for i32 {
    fn maybe_u32(&self) -> Result<u32, Error> {
        self.neg_err().map_err(|e| Error::from(e))
    }

    fn maybe_i32(&self) -> Result<i32, Error> {
        if *self >= 0 {
            Ok(*self)
        } else {
            let e = (-*self) as u32;
            Err(Error::from(e))
        }
    }

    fn maybe_u8(&self) -> Result<u8, Error> {
        Ok(self.maybe_u32()? as u8)
    }

    fn maybe_zero(&self) -> Result<(), Error> {
        self.neg_err().map_err(|e| Error::from(e))?;
        Ok(())
    }

    fn maybe_usize(&self) -> Result<usize, Error> {
        Ok(self.maybe_u32()? as usize)
    }

    fn maybe_isize(&self) -> Result<isize, Error> {
        Ok(self.maybe_i32()? as isize)
    }
}

impl NegErrno for isize {
    fn maybe_u32(&self) -> Result<u32, Error> {
        self.neg_err().map_err(|e| Error::from(e))
    }

    fn maybe_i32(&self) -> Result<i32, Error> {
        if *self >= 0 {
            Ok(*self as i32)
        } else {
            let e = (-*self) as u32;
            Err(Error::from(e))
        }
    }

    fn maybe_u8(&self) -> Result<u8, Error> {
        Ok(self.maybe_u32()? as u8)
    }

    fn maybe_zero(&self) -> Result<(), Error> {
        self.neg_err().map_err(|e| Error::from(e))?;
        Ok(())
    }

    fn maybe_usize(&self) -> Result<usize, Error> {
        Ok(self.maybe_u32()? as usize)
    }

    fn maybe_isize(&self) -> Result<isize, Error> {
        Ok(self.maybe_i32()? as isize)
    }
}
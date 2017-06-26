
use errno::errno;
use num::FromPrimitive;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone)]
    /// Errors that can be encoutered while working with FreeBSD's libc.
    /// Some of them are BSD specific, some of them are from POSIX.
    pub enum BsdError {
        EPERM=      1,  ENOENT, ESRCH, EINTR, EIO, ENXIO, E2BIG, ENOEXEC, EBADF,
        ECHILD=     10, EDEADLK, ENOMEM, EACCES, EFAULT, ENOTBLK, EBUSY, EEXIST, EXDEV, ENODEV,
        ENOTDIR=    20, EISDIR, EINVAL, ENFILE, EMFILE, ENOTTY, ETXTBSY, EFBIG, ENOSPC, ESPIPE,
        EROFS=      30, EMLINK, EPIPE, EDOM, ERANGE, EAGAIN, EINPROGRESS, EALREADY, ENOTSOCK, EDESTADDRREQ,
        EMSGSIZE=   40, EPROTOTYPE, ENOPROTOOPT, EPROTONOSUPPORT, ESOCKTNOSUPPORT, EOPNOTSUPP, EPFNOSUPPORT, EAFNOSUPPORT, EADDRINUSE, EADDRNOTAVAIL,
        ENETDOWN=   50, ENETUNREACH, ENETRESET, ECONNABORTED, ECONNRESET, ENOBUFS, EISCONN, ENOTCONN, ESHUTDOWN,
        ETIMEDOUT=  60, ECONNREFUSED, ELOOP, ENAMETOOLONG, EHOSTDOWN, EHOSTUNREACH, ENOTEMPTY, EPROCLIM, EUSERS, EDQUOT,
        ESTALE=     70, EBADRPC, ERPCMISMATCH, EPROGUNAVAIL, EPROGMISMATCH, EPROCUNAVAIL, ENOLCK, ENOSYS, EFTYPE,
        EAUTH=      80, ENEEDAUTH, EIDRM, ENOMSG, EOVERFLOW, ECANCELED, EILSEQ, ENOATTR, EDOOFUS, EBADMSG,
        EMULTIHOP=  90, ENOLINK, EPROTO, ENOTCAPABLE, ECAPMODE, ENOTRECOVERABLE, EOWNERDEAD

    }
}

impl BsdError {
    /// Create BsdError from errno in current thread. Returns None of error is
    /// not known or there is no error at all.
    pub fn from_errno() -> Option<BsdError> {
        let e = errno();
        BsdError::from_i32(e.0 as i32)
    }
}
impl Error for BsdError {
    fn description(&self) -> &str {
        match *self {
            BsdError::EPERM => {
                "Operation not permitted. An attempt was made to perform an operation limited to \
                 processes with appropriate privileges or to the owner of a file or other \
                 resources."
            }
            BsdError::ENOENT => {
                "No such file or directory. A component of a specified pathname did not exist, or \
                 the pathname was an empty string."
            }
            BsdError::ESRCH => {
                "No such process. No process could be found corresponsing to that specified by the \
                 given ID."
            }
            BsdError::EINTR => {
                "Interrupted system call. An asynchronous signal (such as SIGINT or SIGQUIT) was \
                 caught by the process during the execution of an interruptible function. If the \
                 signal handler performs a normal return, the interrupted system call will seem to \
                 have returned the error condition."
            }
            BsdError::EIO => {
                "Input/output error. Some physical input or output error occurred. This error will \
                 not be reported until a subsequent operation on the same file descriptor and may \
                 be lost (over written) by any subsequent errors."
            }
            BsdError::ENXIO => {
                "Device not configured. Input or output on a special file referred to a device \
                 that did not exist, or made a request beyond the limits of the device.  This \
                 error may also occur when, for example, a tape drive is not online or no disk \
                 pack is loaded on a drive"
            }
            BsdError::E2BIG => {
                "Argument list too long. The number of bytes used for the argument and environment \
                 list of the new process exceeded the current limit (NCARGS in <sys/param.h>)."
            }
            BsdError::ENOEXEC => {
                "Exec format error. A request was made to execute a file that, although it has the \
                 appropriate permissions, was not in the format required for an executable file."
            }
            BsdError::EBADF => {
                "Bad file descriptor. A file descriptor argument was out of range, referred to no \
                 open file, or a read (write) request was made to a file that was only open for \
                 writing (reading)."
            }
            BsdError::ECHILD => {
                "No child processes. A wait(2) or waitpid(2) function was executed by a process \
                 that had no existing or unwaited-for child processes."
            }
            BsdError::EDEADLK => {
                "Resource deadlock avoided. An attempt was made to lock a system resource that \
                 would have resulted in a deadlock situation."
            }
            BsdError::ENOMEM => {
                "Cannot allocate memory. The new process image required more memory than was \
                 allowed by the hardware or by system-imposed memory management constraints. A \
                 lack of swap space is normally temporary; however, a lack of core is not.  Soft \
                 limits may be increased to their corresponding hard limits."
            }
            BsdError::EACCES => {
                "Permission denied. An attempt was made to access a file in a way forbidden by its \
                 file access permissions."
            }
            BsdError::EFAULT => {
                "Bad address. The system detected an invalid address in attempting to use an \
                 argument of a call."
            }
            BsdError::ENOTBLK => {
                "Block device required. A block device operation was attempted on a non-block \
                 device or file."
            }
            BsdError::EBUSY => {
                "Device busy. An attempt to use a system resource which was in use at the time in \
                 a manner which would have conflicted with the request"
            }
            BsdError::EEXIST => {
                "File exists. An existing file was mentioned in an inappropriate context, for \
                 instance, as the new link name in a link(2) system call"
            }
            BsdError::EXDEV => {
                "Cross-device link. A hard link to a file on another file system was attempted."
            }
            BsdError::ENODEV => {
                "Operation not supported by device. An attempt was made to apply an inappropriate \
                 function to a device, for example, trying to read a write-only device such as a \
                 printer."
            }
            BsdError::ENOTDIR => {
                "Not a directory. A component of the specified pathname existed, but it was not a \
                 directory, when a directory was expected."
            }
            BsdError::EISDIR => {
                "Is a directory. An attempt was made to open a directory with write mode specified."
            }
            BsdError::EINVAL => {
                "Invalid argument. Some invalid argument was supplied. (For example, specifying an \
                 undefined signal to a signal(3) function or a kill(2) system call)."
            }
            BsdError::ENFILE => {
                "Too many open files in system. Maximum number of open files allowable on the \
                 system has been reached and requests for an open cannot be satisfied until at \
                 least one has been closed."
            }
            BsdError::EMFILE => {
                "Too many open files. Maximum number of file descriptors allowable in the process \
                 has been reached and requests for an open cannot be satisfied until at least one \
                 has been closed. The getdtablesize(2) system call will obtain the current limit."
            }
            BsdError::ENOTTY => {
                "Inappropriate ioctl for device. A control function (see ioctl(2)) was attempted \
                 for a file or special device for which the operation was inappropriate."
            }
            BsdError::ETXTBSY => {
                "Text file busy. The new process was a pure procedure (shared text) file which was \
                 open for writing by another process, or while the pure procedure file was being \
                 executed an open(2) call requested write access."
            }
            BsdError::EFBIG => "File too large. The size of a file exceeded the maximum.",
            BsdError::ENOSPC => {
                "No space left on device. A write(2) to an ordinary file, the creation of a \
                 directory or symbolic link, or the creation of a directory entry failed because \
                 no more disk blocks were available on the file system, or the allocation of an \
                 inode for a newly created file failed because no more inodes were available on \
                 the file system."
            }
            BsdError::ESPIPE => {
                "Illegal seek. An lseek(2) system call was issued on a socket, pipe or FIFO."
            }
            BsdError::EROFS => {
                "Read-only file system. An attempt was made to modify a file or directory on a \
                 file system that was read-only at the time."
            }
            BsdError::EMLINK => {
                "Too many links. Maximum allowable hard links to a single file has been exceeded \
                 (limit of 32767 hard links per file)."
            }
            BsdError::EPIPE => {
                "Broken pipe. A write on a pipe, socket or FIFO for which there is no process to \
                 read the data."
            }
            BsdError::EDOM => {
                "Numerical argument out of domain. A numerical input argument was outside the \
                 defined domain of the mathematical function."
            }
            BsdError::ERANGE => {
                "Result too large. A numerical result of the function was too large to fit in the \
                 available space (perhaps exceeded precision)."
            }
            BsdError::EAGAIN => {
                "Resource temporarily unavailable. This is a temporary condition and later calls \
                 to the same routine may complete normally."
            }
            BsdError::EINPROGRESS => {
                "Operation now in progress. An operation that takes a long time to complete (such \
                 as a connect(2)) was attempted on a non-blocking object (see fcntl(2))."
            }
            BsdError::EALREADY => {
                "Operation already in progress. An operation was attempted on a non-blocking \
                 object that already had an operation in progress."
            }
            BsdError::ENOTSOCK => "Socket operation on non-socket. Self-explanatory.",
            BsdError::EDESTADDRREQ => {
                "Destination address required. A required address was omitted from an operation on \
                 a socket."
            }
            BsdError::EMSGSIZE => {
                "Message too long. A message sent on a socket was larger than the internal message \
                 buffer or some other network limit."
            }
            BsdError::EPROTOTYPE => {
                "Protocol wrong type for socket. A protocol was specified that does not support \
                 the semantics of the socket type requested. For example, you cannot use the ARPA \
                 Internet UDP protocol with type SOCK_STREAM."
            }
            BsdError::ENOPROTOOPT => {
                "Protocol not available. A bad option or level was specified in a getsockopt(2) or \
                 setsockopt(2) call."
            }
            BsdError::EPROTONOSUPPORT => {
                "Protocol not supported. The protocol has not been configured into the system or \
                 no implementation for it exists."
            }
            BsdError::ESOCKTNOSUPPORT => {
                "Socket type not supported. The support for the socket type has not been \
                 configured into the system or no implementation for it exists."
            }
            BsdError::EOPNOTSUPP => {
                "Operation not supported. The attempted operation is not supported for the type of \
                 object referenced. Usually this occurs when a file descriptor refers to a file or \
                 socket that cannot support this operation, for example, trying to accept a \
                 connection on a datagram socket."
            }
            BsdError::EPFNOSUPPORT => {
                "Protocol family not supported. The protocol family has not been configured into \
                 the system or no implementation for it exists."
            }
            BsdError::EAFNOSUPPORT => {
                "Address family not  supported by protocol family. An address incompatible with \
                 the requested protocol was used. For example, you should not necessarily expect \
                 to be able to use NS addresses with ARPA Internet protocols."
            }
            BsdError::EADDRINUSE => {
                "Address already in use. Only one usage of each address is normally permitted."
            }
            BsdError::EADDRNOTAVAIL => {
                "Can't assign requested address. Normally results from an attempt to create a \
                 socket with an address not on this machine."
            }
            BsdError::ENETDOWN => "Network is down. A socket operation encountered a dead network.",
            BsdError::ENETUNREACH => {
                "Network is unreachable. A socket operation was attempted to an unreachable \
                 network."
            }
            BsdError::ENETRESET => {
                "Network dropped connection on reset. The host you were connected to crashed and \
                 rebooted."
            }
            BsdError::ECONNABORTED => {
                "Software caused connectionabort. A connection abort was caused internal to your \
                 host machine."
            }
            BsdError::ECONNRESET => {
                "Connection reset by peer. A connection was forcibly closed by a peer. This \
                 normally results from a loss of the connection on the remote socket due to a \
                 timeout or a reboot."
            }
            BsdError::ENOBUFS => {
                "No buffer space available. An operation on a socket or pipe was not performed \
                 because the system lacked sufficient buffer space or because a queue was full."
            }
            BsdError::EISCONN => {
                "Socket is already connected. A connect(2) request was made on an already \
                 connected socket; or, a sendto(2) or sendmsg(2) request on a connected socket \
                 specified a destination when already connected."
            }
            BsdError::ENOTCONN => {
                "Socket  is not connected. An request to send or receive data was disallowed \
                 because the socket was not connected and (when sending on a datagram socket) no \
                 address was supplied."
            }
            BsdError::ESHUTDOWN => {
                "Can't send after socket shutdown. A request to send data was disallowed because \
                 the socket had already been shut down with a previous shutdown(2) call."
            }
            BsdError::ETIMEDOUT => {
                "Operation timed out. A connect(2) or send(2) request failed because the connected \
                 party did not properly respond after a period of time.  (The timeout period is \
                 dependent on the communication protocol.)"
            }
            BsdError::ECONNREFUSED => {
                "Connection refused. No connection could be made because the target machine \
                 actively refused it.  This usually results from trying to connect to a service \
                 that is inactive on the foreign host."
            }
            BsdError::ELOOP => {
                "Too many levels of symbolic links. A path name lookup involved more than 32 \
                 (MAXSYMLINKS) symbolic links."
            }
            BsdError::ENAMETOOLONG => {
                "File name too long. A component of a path name exceeded {NAME_MAX} characters, or \
                 an entire path name exceeded {PATH_MAX} characters.(See also the description of \
                 _PC_NO_TRUNC in pathconf(2).)"
            }
            BsdError::EHOSTDOWN => {
                "Host is down. A socket operation failed because the destination host was down."
            }
            BsdError::EHOSTUNREACH => {
                "No route to host. A socket operation was attempted to an unreachable host."
            }
            BsdError::ENOTEMPTY => {
                "Directory not empty. A directory with entries other than `.' and `..' was \
                 supplied to a remove directory or rename call."
            }
            BsdError::EPROCLIM => "Too many processes.",
            BsdError::EUSERS => "Too many users. The quota system ran out of table entries.",
            BsdError::EDQUOT => {
                "Disc quota exceeded. A write(2) to an ordinary file, the creation of a directory \
                 or symbolic link, or the creation of a directory entry failed because the user's \
                 quota of disk blocks was exhausted, or the allocation of an inode for a newly \
                 created file failed because the user's quota of inodes was exhausted."
            }
            BsdError::ESTALE => {
                "Stale NFS file handle. An attempt was made to access an open file (on an NFS file \
                 system) which is now unavailable as referenced by the file descriptor.  This may \
                 indicate the file was deleted on the NFS server or some other catastrophic event \
                 occurred."
            }
            BsdError::EBADRPC => "RPC struct is bad. Exchange of RPC information was unsuccessful.",
            BsdError::ERPCMISMATCH => {
                "RPC version wrong. The version of RPC on the remote peer is not compatible with \
                 the local version."
            }
            BsdError::EPROGUNAVAIL => {
                "RPC prog. not avail. The requested program is not  registered on the remote host."
            }
            BsdError::EPROGMISMATCH => {
                "Program version wrong. The requested version of the program is not available on \
                 the remote host (RPC)."
            }
            BsdError::EPROCUNAVAIL => {
                "Bad procedure for program. An RPC call was attempted for a procedure which does \
                 not exist in the remote program."
            }
            BsdError::ENOLCK => {
                "No locks available. A system-imposed limit on the number of simultaneous file \
                 locks was reached."
            }
            BsdError::ENOSYS => {
                "Function not implemented. Attempted a system call that is not available on this \
                 system."
            }
            BsdError::EFTYPE => {
                "Inappropriate file type or format. The file was the wrong type for the operation, \
                 or a data file had the wrong format."
            }
            BsdError::EAUTH => {
                "Authentication error. Attempted to use an invalid authentication ticket to mount \
                 a NFS file system."
            }
            BsdError::ENEEDAUTH => {
                "Need authenticator. An authentication ticket must be obtained before the given \
                 NFS file system may be mounted."
            }
            BsdError::EIDRM => {
                "Identifier removed. An IPC identifier was removed while the current process was \
                 waiting on it."
            }
            BsdError::ENOMSG => {
                "No message of desired type. An IPC message queue does not contain a message of \
                 the desired type, or a message catalog does not contain the requested message."
            }
            BsdError::EOVERFLOW => {
                "Value too large to be stored in data type. A numerical result of the function was \
                 too large to be stored in the caller provided space."
            }
            BsdError::ECANCELED => "Operation canceled. The scheduled operation was canceled.",
            BsdError::EILSEQ => {
                "Illegal byte sequence.  While decoding a multibyte character the function came \
                 along an invalid or an incomplete sequence of bytes or the given wide character \
                 is invalid."
            }
            BsdError::ENOATTR => {
                "Attribute not found. The specified extended attribute does not exist."
            }
            BsdError::EDOOFUS => {
                "Programming error. A function or API is being abused in a way which could only be \
                 detected at run-time."
            }
            BsdError::EBADMSG => "Bad message. A corrupted message was detected.",
            BsdError::EMULTIHOP => {
                "Multihop attempted. This error code is unused, but present for compatibility with \
                 other systems."
            }
            BsdError::ENOLINK => {
                "Link has been severed. This error code is unused, but present for compatibility \
                 with other systems."
            }
            BsdError::EPROTO => {
                "Protocol error. A device or socket encountered an unrecoverable protocol error."
            }
            BsdError::ENOTCAPABLE => {
                "Capabilities insufficient. An operation on a capability file descriptor requires \
                 greater privilege than the capability allows."
            }
            BsdError::ECAPMODE => {
                "Not permitted in capability mode. The system call or operation is not permitted \
                 for capability mode processes."
            }
            BsdError::ENOTRECOVERABLE => {
                "State not recoverable. The state protected by a robust mutex is not recoverable."
            }
            BsdError::EOWNERDEAD => {
                "Previous owner died. The owner of a robust mutex terminated while holding the \
                 mutex lock."
            }
        }
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for BsdError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self, self.description())
    }
}


#[test]
fn bsd_error() {
    assert_eq!(BsdError::from_i32(1), Some(BsdError::EPERM));
    assert_eq!(BsdError::from_errno(), None);
}

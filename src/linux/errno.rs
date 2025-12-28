#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Errno(pub isize);

impl Errno {
    pub const EPERM: Self = Self(1);
    pub const ENOENT: Self = Self(2);
    pub const ESRCH: Self = Self(3);
    pub const EINTR: Self = Self(4);
    pub const EIO: Self = Self(5);
    pub const EBADF: Self = Self(9);
    pub const ECHILD: Self = Self(10);
    pub const EAGAIN: Self = Self(11);
    pub const ENOMEM: Self = Self(12);
    pub const EACCES: Self = Self(13);
    pub const EFAULT: Self = Self(14);
    pub const EBUSY: Self = Self(16);
    pub const EEXIST: Self = Self(17);
    pub const ENOTDIR: Self = Self(20);
    pub const EINVAL: Self = Self(22);
    pub const ENOSPC: Self = Self(28);
    pub const EROFS: Self = Self(30);
    pub const ENOSYS: Self = Self(38);

    #[inline(always)]
    pub fn is(&self, code: isize) -> bool {
        self.0 == code
    }
}

impl core::fmt::Debug for Errno {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match *self {
            Self::EPERM => "EPERM",
            Self::ENOENT => "ENOENT",
            Self::ESRCH => "ESRCH",
            Self::EINTR => "EINTR",
            Self::EIO => "EIO",
            Self::EBADF => "EBADF",
            Self::ECHILD => "ECHILD",
            Self::EAGAIN => "EAGAIN",
            Self::ENOMEM => "ENOMEM",
            Self::EACCES => "EACCES",
            Self::EFAULT => "EFAULT",
            Self::EBUSY => "EBUSY",
            Self::EEXIST => "EEXIST",
            Self::ENOTDIR => "ENOTDIR",
            Self::EINVAL => "EINVAL",
            Self::ENOSPC => "ENOSPC",
            Self::EROFS => "EROFS",
            Self::ENOSYS => "ENOSYS",
            _ => "UNKNOWN",
        };
        write!(f, "{}({})", name, self.0)
    }
}

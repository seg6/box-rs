use crate::linux::raw::{self, SyscallResult};
use core::ops::BitOr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CloneFlags(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct WaitFlags(usize);

impl CloneFlags {
    pub const EMPTY: Self = Self(0);
    pub const NEWNS: Self = Self(raw::CLONE_NEWNS);
    pub const NEWUTS: Self = Self(raw::CLONE_NEWUTS);
    pub const NEWPID: Self = Self(raw::CLONE_NEWPID);
    pub const NEWNET: Self = Self(raw::CLONE_NEWNET);
    pub const NEWIPC: Self = Self(raw::CLONE_NEWIPC);
    pub const SIGCHLD: Self = Self(raw::SIGCHLD);
}

impl BitOr for CloneFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl WaitFlags {
    pub const EMPTY: Self = Self(0);
    pub const NOHANG: Self = Self(raw::WNOHANG);
    pub const UNTRACED: Self = Self(raw::WUNTRACED);
    pub const WALL: Self = Self(raw::__WALL);
}

impl BitOr for WaitFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[inline(always)]
pub fn clone(flags: CloneFlags, stack: Option<usize>) -> SyscallResult {
    unsafe { raw::clone(flags.0, stack.unwrap_or(0), 0, 0, 0) }
}

#[inline(always)]
pub fn wait4(pid: isize, status: Option<&mut i32>, options: WaitFlags) -> SyscallResult {
    unsafe {
        raw::wait4(
            pid,
            status.map(|s| s as *mut i32 as usize).unwrap_or(0),
            options.0,
        )
    }
}

#[inline(always)]
pub fn write(fd: usize, buf: &[u8]) -> SyscallResult {
    unsafe { raw::write(fd, buf.as_ptr() as usize, buf.len()) }
}

#[inline(always)]
pub fn exit(code: isize) -> ! {
    unsafe { raw::exit(code) }
}

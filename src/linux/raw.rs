pub type SyscallResult = core::result::Result<usize, super::errno::Errno>;

#[derive(Copy, Clone)]
#[repr(usize)]
enum Syscall {
    Write = 1,
    Clone = 56,
    Exit = 60,
    Wait4 = 61,
}

#[inline(always)]
pub fn check_syscall_ret(ret: isize) -> SyscallResult {
    if ret < 0 && ret >= -4095 {
        Err(super::errno::Errno(-ret))
    } else {
        Ok(ret as usize)
    }
}

#[macro_export]
macro_rules! syscall {
    ($id:expr, $a1:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
    ($id:expr, $a1:expr, $a2:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                in("rsi") $a2,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
    ($id:expr, $a1:expr, $a2:expr, $a3:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                in("rsi") $a2,
                in("rdx") $a3,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
    ($id:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                in("rsi") $a2,
                in("rdx") $a3,
                in("r10") $a4,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
    ($id:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                in("rsi") $a2,
                in("rdx") $a3,
                in("r10") $a4,
                in("r8") $a5,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
    ($id:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        crate::linux::raw::check_syscall_ret({
            let ret: isize;
            core::arch::asm!(
                "syscall",
                in("rax") $id as usize,
                in("rdi") $a1,
                in("rsi") $a2,
                in("rdx") $a3,
                in("r10") $a4,
                in("r8")  $a5,
                in("r9")  $a6,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret
            );
            ret
        })
    };
}

pub const CLONE_NEWNS: usize = 0x00020000;
pub const CLONE_NEWUTS: usize = 0x04000000;
pub const CLONE_NEWPID: usize = 0x20000000;
pub const CLONE_NEWNET: usize = 0x40000000;
pub const CLONE_NEWIPC: usize = 0x08000000;

pub const SIGCHLD: usize = 17;

#[inline(always)]
pub unsafe fn clone(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    child_tid: usize,
    tls: usize,
) -> SyscallResult {
    unsafe { crate::syscall!(Syscall::Clone, flags, stack, parent_tid, child_tid, tls) }
}

pub const WNOHANG: usize = 1;
pub const WUNTRACED: usize = 2;
pub const __WALL: usize = 0x40000000;

#[inline(always)]
pub unsafe fn wait4(pid: isize, status: usize, options: usize) -> SyscallResult {
    unsafe { crate::syscall!(Syscall::Wait4, pid, status, options, 0) }
}

#[inline(always)]
pub unsafe fn write(fd: usize, buf: usize, len: usize) -> SyscallResult {
    unsafe { crate::syscall!(Syscall::Write, fd, buf, len) }
}

#[inline(always)]
pub unsafe fn exit(code: isize) -> ! {
    unsafe { let _ = crate::syscall!(Syscall::Exit, code); }
    unsafe { core::hint::unreachable_unchecked(); }
}

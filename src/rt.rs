#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "mov rdi, [rsp]",
        "lea rsi, [rsp + 8]",
        "and rsp, -16",
        "call {start_rust}",
        start_rust = sym start_rust,
    );
}

unsafe extern "C" fn start_rust(argc: usize, argv: *const *const u8) -> ! {
    let args = unsafe { crate::utils::Args::from_raw(argc, argv) };
    crate::linux::exit(crate::main(args));
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::eprint!("box runtime panicked");
    if let Some(loc) = info.location() {
        crate::eprint!(" at {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    crate::eprint!(" >> {}\n", info.message());
    crate::linux::exit(1);
}

// LLVM optimizes certain operations into libc calls even with -nodefaultlibs.
// So we provide our own implementations to satisfy the linker.
// How do I do `-fno-builtin` again?!

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep movsb",
            inout("rdi") dest => _,
            inout("rsi") src => _,
            inout("rcx") n => _,
            options(nostack, preserves_flags)
        );
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        core::arch::asm!(
            "rep stosb",
            inout("rdi") s => _,
            in("al") c as u8,
            inout("rcx") n => _,
            options(nostack, preserves_flags)
        );
    }
    s
}

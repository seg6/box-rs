use crate::linux::{self, Errno, SyscallResult};

pub struct FdWriter(pub usize);

impl FdWriter {
    pub const STDOUT: Self = Self(1);
    pub const STDERR: Self = Self(2);

    pub fn new(fd: usize) -> Self {
        Self(fd)
    }

    pub fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Errno> {
        while !buf.is_empty() {
            match linux::write(self.0, buf)? {
                0 => return Err(Errno::EIO),
                n => buf = &buf[n..],
            }
        }
        Ok(())
    }
}

impl core::fmt::Write for FdWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

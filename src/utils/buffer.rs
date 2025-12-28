pub type BufferResult<T> = core::result::Result<T, BufferError>;

#[derive(Debug, Clone, Copy)]
pub enum BufferError {
    BufferFull,
    CStrErr(core::ffi::FromBytesWithNulError),
}

pub struct Buffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn remaining(&self) -> usize {
        N - self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn push_byte(&mut self, b: u8) -> BufferResult<()> {
        if self.len >= N {
            return Err(BufferError::BufferFull);
        }
        self.buf[self.len] = b;
        self.len += 1;
        Ok(())
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> BufferResult<()> {
        if self.len + bytes.len() > N {
            return Err(BufferError::BufferFull);
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    pub fn push_str(&mut self, s: &str) -> BufferResult<()> {
        self.push_bytes(s.as_bytes())
    }

    pub fn push_nul(&mut self) -> BufferResult<()> {
        self.push_byte(0)
    }

    pub fn as_cstr(&self) -> BufferResult<&core::ffi::CStr> {
        core::ffi::CStr::from_bytes_with_nul(self.as_bytes()).map_err(BufferError::CStrErr)
    }
}

impl<const N: usize> core::fmt::Write for Buffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s).map_err(|_| core::fmt::Error)
    }
}

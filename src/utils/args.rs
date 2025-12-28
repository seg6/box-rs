pub struct Args {
    argv: *const *const u8,
    argc: usize,
}

impl Args {
    pub unsafe fn from_raw(argc: usize, argv: *const *const u8) -> Self {
        Self { argv, argc }
    }

    pub fn len(&self) -> usize {
        self.argc
    }

    pub fn is_empty(&self) -> bool {
        self.argc == 0
    }

    pub fn get(&self, index: usize) -> Option<Result<&str, core::str::Utf8Error>> {
        if index >= self.argc {
            return None;
        }

        unsafe {
            let ptr = *self.argv.add(index);
            if ptr.is_null() {
                return None;
            }

            Some(core::str::from_utf8(core::slice::from_raw_parts(ptr, crate::rt::strlen(ptr))))
        }
    }

    pub fn iter(&self) -> ArgsIter<'_> {
        ArgsIter {
            args: self,
            index: 0,
        }
    }
}

pub struct ArgsIter<'a> {
    args: &'a Args,
    index: usize,
}

impl<'a> Iterator for ArgsIter<'a> {
    type Item = Result<&'a str, core::str::Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.args.argc {
            return None;
        }
        let result = self.args.get(self.index)?;
        self.index += 1;
        Some(result)
    }
}

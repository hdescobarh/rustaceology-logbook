use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R: Read> {
    operations: usize,
    bytes: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            operations: 0,
            bytes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let readed_bytes = self.wrapped.read(buf)?;
        self.bytes += readed_bytes;
        self.operations += 1;
        Ok(readed_bytes)
    }
}

pub struct WriteStats<W>(::std::marker::PhantomData<W>);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        todo!()
    }

    pub fn get_ref(&self) -> &W {
        todo!()
    }

    pub fn bytes_through(&self) -> usize {
        todo!()
    }

    pub fn writes(&self) -> usize {
        todo!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        todo!("Collect statistics about this call writing {buf:?}")
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}

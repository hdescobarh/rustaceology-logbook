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
        let read_bytes = self.wrapped.read(buf)?;
        self.bytes += read_bytes;
        self.operations += 1;
        Ok(read_bytes)
    }
}

pub struct WriteStats<W: Write> {
    operations: usize,
    bytes: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            operations: 0,
            bytes: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written_bytes = self.wrapped.write(buf)?;
        self.bytes += written_bytes;
        self.operations += 1;
        Ok(written_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}

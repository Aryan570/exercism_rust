use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

// pub struct ReadStats<R>(::std::marker::PhantomData<R>);
#[derive(Clone, Copy)]
pub struct ReadStats<R>{
    data : R,
    no_reads : usize,
    bytes_th : usize
}
impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats { data: wrapped , no_reads : 0 ,bytes_th : 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_th
    }

    pub fn reads(&self) -> usize {
        self.no_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_reads = self.data.read(buf)?;
        self.no_reads += 1; self.bytes_th += bytes_reads;
        Ok(bytes_reads)
    }
}

pub struct WriteStats<W>{
    data : W,
    bytes_th : usize,
    no_writes : usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats { data: wrapped ,bytes_th : 0 ,no_writes : 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_th
    }

    pub fn writes(&self) -> usize {
        self.no_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_writes = self.data.write(buf)?;
        self.no_writes += 1; self.bytes_th += bytes_writes;
        Ok(bytes_writes)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}

use std::io;
use io::{Read, Result};

pub struct PeekyRead<'a, R: Read + 'a> {
    inner: &'a mut R,
    peeked: Option<u8>,
}

impl<'a, R: Read + 'a> Read for PeekyRead<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.peeked {
            Some(c) => {
                buf[0] = c;
                self.peeked = None;
                Ok(1)
            }
            None => self.inner.read(buf),
        }
    }
}

impl<'a, R: Read> PeekyRead<'a, R> {
    pub fn new(inner: &mut R) -> PeekyRead<R> {
        PeekyRead {
            inner,
            peeked: None,
        }
    }

    pub fn check_eof(&mut self) -> Result<bool> {
        if self.peeked.is_some() {
            // we have something more to return; read won't return 0 bytes
            return Ok(false);
        }

        let mut buf = [0; 1];
        Ok(match self.inner.read(&mut buf)? {
            0 => true,
            1 => {
                self.peeked = Some(buf[0]);
                false
            }
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests;

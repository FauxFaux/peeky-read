//! Wrap an `io::Read` and provide `check_eof()`.
//! Returns errors iff the underlying reader does.
//!
//! # Examples
//! ```
//! use std::io::Read;
//! use peeky_read::PeekyRead;
//! let mut underlying = std::io::Cursor::new([0u8; 1]);
//! let mut reader = PeekyRead::new(&mut underlying);
//!
//! // We're not at the EOF as there's bytes to read.
//! assert_eq!(false, reader.check_eof().unwrap());
//! let mut buf = [0u8; 32];
//! assert_eq!(1, reader.read(&mut buf).unwrap());
//!
//! // We've read the only byte in the reader, so we're now at the EOF.
//! assert_eq!(true, reader.check_eof().unwrap());
//! ```

use std::io;
use io::{Read, Result};

pub struct PeekyRead<'a, R: Read + 'a> {
    inner: &'a mut R,
    peeked: Option<u8>,
}

impl<'a, R: Read + 'a> Read for PeekyRead<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

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
    /// Wrap a reader. No `read` will happen immediately.
    pub fn new(inner: &mut R) -> PeekyRead<R> {
        PeekyRead {
            inner,
            peeked: None,
        }
    }

    /// Check that `read`ing from this reader will return at least one byte.
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

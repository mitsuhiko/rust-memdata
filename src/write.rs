use std::io;
use std::mem;
use std::fmt;
use std::slice;
use std::io::Write;

use num_traits::cast::{cast, NumCast};

use types::Seg;

/// Writes a value into a writer
pub fn write_value<T: Copy, W: Write>(writer: W, value: &T) -> io::Result<usize> {
    let seg: Seg<u8, usize, usize> = ByteWriter::new(writer).write_value(value)?;
    Ok(seg.len())
}

/// A helper for writing bytes into a file
pub struct ByteWriter<W: Write> {
    pos: usize,
    writer: W,
}

fn err(msg: &'static str) -> io::Error{
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

impl<W: Write> ByteWriter<W> {
    /// Creates a new byte writer.
    pub fn new(writer: W) -> ByteWriter<W> {
        ByteWriter {
            pos: 0,
            writer: writer,
        }
    }

    /// Returns the current position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Write bytes into a file and return a segment.
    pub fn write_bytes<Length, Offset>(&mut self, bytes: &[u8])
        -> io::Result<Seg<u8, Length, Offset>>
        where Length: Copy + fmt::Debug + NumCast,
              Offset: Copy + fmt::Debug + NumCast
    {
        let offset = self.pos;
        self.pos += bytes.len();
        self.writer.write_all(bytes)?;
        Ok(Seg::new(
            cast(offset)
                .ok_or_else(|| err("out of range: byte segment offset"))?,
            cast(bytes.len())
                .ok_or_else(|| err("out of range: byte segment length"))?
        ))
    }

    /// Writes a single value into the file
    #[inline(always)]
    pub fn write_value<T, Length, Offset>(&mut self, x: &T)
        -> io::Result<Seg<u8, Length, Offset>>
        where T: Copy,
              Length: Copy + fmt::Debug + NumCast,
              Offset: Copy + fmt::Debug + NumCast
    {
        unsafe {
            let bytes: *const u8 = mem::transmute(x);
            let size = mem::size_of_val(x);
            self.write_bytes(slice::from_raw_parts(bytes, size))
        }
    }

    /// Writes a slice as segment into the file
    pub fn write_seg<T, Length, Offset>(&mut self, x: &[T])
        -> io::Result<Seg<T, Length, Offset>>
        where T: Copy,
              Length: Copy + fmt::Debug + NumCast,
              Offset: Copy + fmt::Debug + NumCast
    {
        let mut first_seg: Option<Seg<u8, Length, Offset>> = None;
        for item in x {
            let seg = self.write_value(item)?;
            if first_seg.is_none() {
                first_seg = Some(seg);
            }
        }
        Ok(Seg::new(
            cast(first_seg.map(|x| x.offset()).unwrap_or(0))
                .ok_or_else(|| err("out of range: item segment offset"))?,
            cast(x.len())
                .ok_or_else(|| err("out of range: item segment length"))?
        ))
    }
}

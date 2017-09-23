use std::io;
use std::str;
use std::mem;
use std::fmt;
use std::slice;
use std::ops::Range;
use std::marker::PhantomData;

use num_traits::cast::{cast, NumCast};


/// Generic way to represent a segment.
#[repr(C, packed)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
pub struct Seg<T, Length=u32, Offset=u32>
    where Length: NumCast + Copy,
          Offset: NumCast + Copy
{
    offset: Offset,
    len: Length,
    _ty: PhantomData<*const T>,
}

impl<T, Length, Offset> Seg<T, Length, Offset> 
    where Length: Copy + fmt::Debug + NumCast,
          Offset: Copy + fmt::Debug + NumCast
{
    /// Constructs a new segment with the given values.
    #[inline(always)]
    pub fn new(offset: Offset, len: Length) -> Self {
        Seg {
            offset: offset,
            len: len,
            _ty: PhantomData,
        }
    }

    /// Returns the offset of the segment as usize in bytes.
    #[inline(always)]
    pub fn offset(&self) -> usize {
        cast(self.offset).unwrap_or(0)
    }

    /// Returns the offset of the segment as usize as count.
    #[inline(always)]
    pub fn len(&self) -> usize {
        cast(self.len).unwrap_or(0)
    }

    /// Returns the size in bytes of a single item.
    #[inline(always)]
    pub fn item_size(&self) -> usize {
        mem::size_of::<T>()
    }

    /// Returns the byte range for this slice.
    ///
    /// This can return `None` if an overflow ocurred while calculating
    /// the range.
    #[inline(always)]
    pub fn byte_range(&self) -> Option<Range<usize>> {
        self.offset()
            .checked_add(self.item_size() * self.len())
            .map(|end| Range { start: self.offset(), end: end })
    }

    /// Given payload bytes returns the segment as slice.
    ///
    /// When the payload of a file is given this resolves the segment into
    /// the source slice it represents.  This method does not panic but can
    /// fail if the file is out of bounds.
    #[inline(always)]
    pub fn to_slice<'a>(&self, data: &'a [u8]) -> io::Result<&'a [T]> {
        self.byte_range()
            .and_then(|range| data.get(range))
            .map(|b| unsafe {
                slice::from_raw_parts(mem::transmute(b.as_ptr()), self.len())
            })
            .ok_or_else(#[inline(never)] || {
                io::Error::new(io::ErrorKind::InvalidInput,
                               "segment out of range")
            })
    }
}

impl<Length, Offset> Seg<u8, Length, Offset> 
    where Length: Copy + fmt::Debug + NumCast,
          Offset: Copy + fmt::Debug + NumCast
{
    /// Given payload bytes returns the segment as str.
    ///
    /// This is similar to `to_slice` and is only implemented for segments
    /// over u8 data.  Instead of returning the data as u8 array a string
    /// is returned.  In case the data is malformed an error is returned.
    pub fn to_str<'a>(&self, data: &'a [u8]) -> io::Result<&'a str> {
        let bytes = self.to_slice(data)?;
        str::from_utf8(bytes)
            .map_err(|err| io::Error::new(
                io::ErrorKind::InvalidInput, err))
    }
}

impl<T, Length, Offset> fmt::Debug for Seg<T, Length, Offset> 
    where Length: Copy + fmt::Debug + NumCast,
          Offset: Copy + fmt::Debug + NumCast
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Seg")
            .field("offset", &self.offset())
            .field("len", &self.len())
            .finish()
    }
}

use std::io;
use std::mem;


pub fn split_header<T>(data: &[u8]) -> io::Result<(&T, &[u8])> {
    let header_size = mem::size_of::<T>();
    if header_size > data.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "out of range"));
    }
    let (header_bytes, payload) = data.split_at(header_size);
    Ok((
        unsafe { mem::transmute(header_bytes.as_ptr()) },
        payload
    ))
}

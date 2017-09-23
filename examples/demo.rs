extern crate memdata;

use std::io;

use memdata::{Seg, ByteWriter, write_value, split_header};


#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
struct FileHeader {
    version: u32,
    strings: Seg<Seg<u8, u16>>,
}


fn write_file() -> io::Result<Vec<u8>> {
    let mut out = vec![0u8; 0];
    let mut header = FileHeader::default();
    header.version = 1;

    // start by writing the header into the file once
    let header_size = write_value(&mut out, &header)?;

    // write the strings
    {
        let mut writer = ByteWriter::new(&mut out);
        let mut strings = vec![];
        strings.push(writer.write_bytes(b"Hello")?);
        strings.push(writer.write_bytes(b"World")?);
        header.strings = writer.write_seg(&strings)?;
    }

    // overwrite the header now that it has a segment
    write_value(&mut out[..header_size], &header)?;

    Ok(out)
}

fn read_file(data: &[u8]) -> io::Result<()> {
    let (header, payload) = split_header::<FileHeader>(data)?;

    println!("{:?}", header);
    println!("Strings in the file:");
    for string_seg in header.strings.to_slice(payload)? {
        println!(" > {:?}", string_seg.to_str(payload)?);
    }
    Ok(())
}


pub fn main() {
    let out = write_file().unwrap();
    println!("Written bytes: {:?}", &out[..]);
    read_file(&out[..]).unwrap();
}

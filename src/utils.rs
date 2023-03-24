use std::{fs::File, io::{self, Seek, Read}};

pub fn padding(f: &mut File, len: i64) -> io::Result<()> {
    f.seek(io::SeekFrom::Current(len))?;
    Ok(())
}

pub fn read_offset(f: &mut File, offset: u64, len: usize) -> io::Result<Vec<u8>> {
    let old_offset = f.stream_position()?;
    f.seek(io::SeekFrom::Start(offset.into()))?;
    let mut buf = vec![0u8; len];
    f.read_exact(&mut buf)?;
    f.seek(io::SeekFrom::Start(old_offset))?;

    Ok(buf)
}

pub fn read_u32(f: &mut File) -> io::Result<u32> {
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    let result = u32::from_le_bytes(buff);

    Ok(result)
}
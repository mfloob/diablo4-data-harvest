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

pub fn read_f32(f: &mut File) -> io::Result<f32> {
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    let result = f32::from_le_bytes(buff);

    Ok(result)
}

pub fn go_to(f: &mut File, offset: u64) -> io::Result<()> {
    f.seek(io::SeekFrom::Start(offset.into()))?;
    Ok(())
}

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn load_or_pick_data_file(file_name: &str) -> Option<Vec<u8>> {
    let buf = match read_file(file_name) {
        Ok(data) => Some(data),
        Err(_) => {
            match rfd::FileDialog::new()
                .add_filter(".json files", &["json"])
                .set_file_name(file_name)
                .pick_file() {
                    Some(path) => {
                        let path = path.display().to_string();
                        let buf = read_file(&path).unwrap();
                        Some(buf)
                    },
                    _ => None
            }
        }
    };

    buf
}
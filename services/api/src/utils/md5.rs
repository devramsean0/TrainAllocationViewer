use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn md5_of_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut context = md5::Context::new();
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }
    let digest = context.finalize();
    Ok(format!("{:x}", digest))
}

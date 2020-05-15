use png::Decoder;
use std::error::Error;
use std::fs::File;
use std::io;

pub fn load_image_rgba(filepath: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // open png file
    let decoder = Decoder::new(File::open(filepath)?);
    // reader implements std::reader
    let (info, mut reader) = decoder.read_info()?;
    let mut buff: Vec<u8> = Vec::with_capacity(info.buffer_size());

    // since a png consists of only one frame, we only need to call this function once
    reader.next_frame(&mut buff)?;

    Ok(buff)
}

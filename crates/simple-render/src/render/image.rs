use std::{fs::File, io::BufReader};

use png::{Decoder, DecodingError};

use crate::render::buffer::{Buffer, BufferSize, RawBuffer};

pub struct Image(pub Buffer);

impl Image {
    pub const fn new(buffer: Buffer) -> Self {
        Image(buffer)
    }

    // Problems with RGB/RGBA
    pub fn from_png(file: File) -> Result<Self, DecodingError> {
        let decoder = Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info()?;

        let mut buffer = vec![0; reader.output_buffer_size().unwrap()];

        let info = reader.next_frame(&mut buffer).unwrap();

        let buffer = Vec::from(&buffer[..info.buffer_size()]);

        let size = BufferSize::new(info.width as usize, info.height as usize);

        let raw_buffer = RawBuffer::new(
            buffer
            .chunks_exact(3)
            .map(|chunk| {
                let (r, g, b) = (chunk[0] as u32, chunk[1] as u32, chunk[2] as u32);
                (r << 16) | (g << 8) | b
            })
            .collect()
        );

        Ok(Image::new(Buffer::new(raw_buffer, size)))
    }
}

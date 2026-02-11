use std::{fs::File, io::{BufReader, BufWriter}, path::PathBuf};

use png::{Decoder, DecodingError, Encoder, EncodingError};

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

    pub fn to_png(&self, path: PathBuf) -> Result<(), EncodingError> {

        let file = File::create(path)?;

        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(
            w,
            self.0.size.width as u32,
            self.0.size.height as u32
        );

        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = self.0.raw_buffer.0
        .iter().flat_map(|value| {
            [
                (*value >> 16) as u8,
                (*value >> 8) as u8,
                (*value) as u8
            ]
        }).collect();

        writer.write_image_data(&data)?;

        Ok(())
    }
}

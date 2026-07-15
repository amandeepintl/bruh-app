use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct BruhImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl BruhImage {
    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("image dimensions must be greater than zero".into());
        }
        let expected_len = (width as usize) * (height as usize) * 4;
        if rgba.len() != expected_len {
            return Err(format!(
                "expected {} RGBA bytes, found {}",
                expected_len, rgba.len()
            ));
        }
        Ok(Self { width, height, rgba })
    }

    pub fn from_image_file(path: &Path) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| format!("failed to open image: {e}"))?;
        let rgba_img = img.to_rgba8();
        Self::new(rgba_img.width(), rgba_img.height(), rgba_img.into_raw())
    }

    pub fn encode_to_bruh(&self, path: &Path) -> Result<(), String> {
        let mut output = Vec::new();
        output.extend_from_slice(b"BTTR");
        output.push(1);
        output.push(0x03);
        output.extend_from_slice(&self.width.to_le_bytes());
        output.extend_from_slice(&self.height.to_le_bytes());
        output.extend_from_slice(&self.rgba);
        fs::write(path, output).map_err(|e| format!("failed to write .bruh file: {e}"))
    }

    pub fn decode_from_bruh(path: &Path) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| format!("failed to read .bruh file: {e}"))?;
        if bytes.len() < 14 {
            return Err("file is too small to be a valid .bruh image".into());
        }
        if &bytes[0..4] != b"BTTR" {
            return Err("invalid .bruh magic header".into());
        }
        let width = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let height = u32::from_le_bytes(bytes[10..14].try_into().unwrap());
        let rgba_len = (width as usize) * (height as usize) * 4;
        if bytes.len() != 14 + rgba_len {
            return Err(format!("expected {} payload bytes, found {}", rgba_len, bytes.len() - 14));
        }
        let rgba = bytes[14..].to_vec();
        Self::new(width, height, rgba)
    }

    pub fn save_as_png(&self, path: &Path) -> Result<(), String> {
        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(self.width, self.height, self.rgba.clone())
            .ok_or_else(|| "failed to rebuild image buffer".to_string())?;
        img.save(path).map_err(|e| format!("failed to write png: {e}"))
    }
}

pub fn compile_image(input: &Path, output: &Path) -> Result<(), String> {
    let image = BruhImage::from_image_file(input)?;
    image.encode_to_bruh(output)
}

pub fn decode_image(input: &Path, output: &Path) -> Result<(), String> {
    let image = BruhImage::decode_from_bruh(input)?;
    image.save_as_png(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_png_to_bruh_and_back() {
        let temp_dir = std::env::temp_dir().join("bruh-test");
        let _ = fs::create_dir_all(&temp_dir);
        let bruh = temp_dir.join("input.bruh");
        let output = temp_dir.join("restored.png");

        let mut buffer = vec![0u8; 16];
        buffer[0..4].copy_from_slice(&[255, 0, 0, 255]);
        buffer[4..8].copy_from_slice(&[0, 255, 0, 255]);
        buffer[8..12].copy_from_slice(&[0, 0, 255, 255]);
        buffer[12..16].copy_from_slice(&[255, 255, 255, 128]);

        let image = BruhImage::new(2, 2, buffer).unwrap();
        image.encode_to_bruh(&bruh).unwrap();
        let decoded = BruhImage::decode_from_bruh(&bruh).unwrap();
        assert_eq!(decoded.width, 2);
        assert_eq!(decoded.height, 2);
        assert_eq!(decoded.rgba, image.rgba);
        decoded.save_as_png(&output).unwrap();
        assert!(output.exists());
    }
}

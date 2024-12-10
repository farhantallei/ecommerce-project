use std::path::Path;
use mime::Mime;

fn is_image(content_type: &str, allowed_types: &[Mime]) -> bool {
  allowed_types.contains(&content_type.parse().unwrap())
}

fn check_magic_bytes(file: &[u8], allowed_types: &[Mime]) -> bool {
  let jpeg_magic = [0xFF, 0xD8, 0xFF];
  let png_magic = [0x89, 0x50, 0x4E, 0x47];
  let gif_magic = [0x47, 0x49, 0x46, 0x38];
  let webp_magic = [0x52, 0x49, 0x46, 0x46];

  if allowed_types.contains(&mime::IMAGE_JPEG) && file.starts_with(&jpeg_magic) {
    return true;
  }

  if allowed_types.contains(&mime::IMAGE_PNG) && file.starts_with(&png_magic) {
    return true;
  }

  if allowed_types.contains(&mime::IMAGE_GIF) && file.starts_with(&gif_magic) {
    return true;
  }

  if allowed_types.contains(&"image/webp".parse().unwrap()) && file.starts_with(&webp_magic) {
    return true;
  }

  false
}

pub fn is_valid_image(file_path: &Path, content_type: &str, allowed_types: &[Mime]) -> bool {
  let file = &std::fs::read(file_path).unwrap();

  if !is_image(content_type, allowed_types) {
    return false;
  }

  check_magic_bytes(file, allowed_types)
}

use actix_web::web::{BufMut as _};

pub fn setup() {
  dotenv::from_filename(".env.testing").ok();
}

const CRLF: &[u8] = b"\r\n";
const CRLF_CRLF: &[u8] = b"\r\n\r\n";
const HYPHENS: &[u8] = b"--";
const BOUNDARY_PREFIX: &str = "------------------------";

pub struct Part {
  pub name: String,
  pub filename: Option<String>,
  pub content_type: Option<mime::Mime>,
  pub file: actix_web::web::Bytes,
}

pub fn create_multipart_form_data(fields: Vec<(&str, Part)>, boundary: &str) -> (actix_web::web::Bytes, actix_web::http::header::HeaderMap) {
  let total_size = fields.iter().fold(0, |acc, (_, part)| {
    let part_size = part.file.len() + 128;
    acc + part_size
  });

  let mut buf = actix_web::web::BytesMut::with_capacity(total_size);

  let boundary_str = [BOUNDARY_PREFIX, boundary].concat();
  let boundary = boundary_str.as_bytes();

  for (key, mut part) in fields {
    buf.put(HYPHENS);
    buf.put(boundary);
    buf.put(CRLF);

    buf.put(format!("Content-Disposition: form-data; name=\"{}\"", key).as_bytes());
    if let Some(filename) = part.filename {
      buf.put(format!("; filename=\"{}\"", filename).as_bytes());
    }
    buf.put(CRLF);

    if let Some(content_type) = part.content_type {
      buf.put(format!("Content-Type: {}", content_type).as_bytes());
      buf.put(CRLF);
    }

    buf.put(format!("Content-Length: {}", part.file.len()).as_bytes());
    buf.put(CRLF_CRLF);

    buf.put(&mut part.file);
    buf.put(CRLF);
  }

  buf.put(HYPHENS);
  buf.put(boundary);
  buf.put(HYPHENS);
  buf.put(CRLF);

  let mut headers = actix_web::http::header::HeaderMap::new();
  headers.insert(
    actix_web::http::header::CONTENT_TYPE,
    format!("multipart/form-data; boundary=\"{}\"", boundary_str)
      .parse()
      .unwrap(),
  );

  (buf.freeze(), headers)
}

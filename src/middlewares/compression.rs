/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use flate2::{write::GzEncoder, Compression};
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    Request, Response,
};

pub struct CompressionMiddleware;

#[async_trait]
impl Fairing for CompressionMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Response Compression",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        if let Some(body) = res.body_mut() {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            let _ = std::io::copy(body, &mut encoder);
            let compressed_data = encoder.finish().unwrap();

            res.set_header(("Content-Encoding", "gzip"));
            res.set_sized_body(compressed_data.len(), std::io::Cursor::new(compressed_data));
        }
    }
}

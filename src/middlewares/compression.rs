/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use std::io::{copy, Cursor};

use flate2::{write::GzEncoder, Compression};
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

/// Response compression middleware
pub struct Compressor;

#[async_trait]
impl Fairing for Compressor {
    /// Returns the name and kind of the middleware
    fn info(&self) -> Info {
        Info {
            name: "Response Compression",
            kind: Kind::Response,
        }
    }

    /// Compresses the response body with Gzip
    ///
    /// # Arguments
    /// * `req` - The incoming request
    /// * `res` - The outgoing response
    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        if !res.headers().contains("Content-Encoding") {
            let mut body = res.body_mut().take();
            let mut data = Vec::new();
            if body.read_to_end(&mut data).await.is_err() {
                return;
            }

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            if copy(&mut data.as_slice(), &mut encoder).is_err() {
                return;
            }

            if let Ok(compressed_data) = encoder.finish() {
                res.set_header(Header::new("Content-Encoding", "gzip"));
                res.set_sized_body(compressed_data.len(), Cursor::new(compressed_data));
            }
        }
    }
}

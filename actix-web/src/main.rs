use std::io;
use actix_web::{get, web, App, HttpServer};
use actix_web::web::BufMut;
use tokio::fs::File;
use tokio_util::codec::{FramedRead, Decoder, Encoder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn index() -> io::Result<actix_web::HttpResponse> {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).await?;
    let stream = FramedRead::with_capacity(file, BytesCodec::new(), 1024 * 16);
    Ok(actix_web::HttpResponse::Ok()
       .force_close()
       .insert_header(("cache-control", "no-store"))
       .insert_header(("content-type", "video/MP2T"))
       .streaming(stream))
}

// Implement Decoder for Bytes because tokio_util::codec::BytesCodec implements
// Decoder for BytesMut.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BytesCodec(());

impl BytesCodec {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> BytesCodec {
        BytesCodec(())
    }
}

impl Decoder for BytesCodec {
    type Item = web::Bytes;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut web::BytesMut) -> Result<Option<web::Bytes>, io::Error> {
        if !buf.is_empty() {
            let len = buf.len();
            Ok(Some(buf.split_to(len).freeze()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<web::Bytes> for BytesCodec {
    type Error = io::Error;

    fn encode(&mut self, data: web::Bytes, buf: &mut web::BytesMut) -> Result<(), io::Error> {
        buf.reserve(data.len());
        buf.put(data);
        Ok(())
    }
}

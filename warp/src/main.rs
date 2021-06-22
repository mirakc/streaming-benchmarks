use tokio::fs::File;
use tokio_util::codec::{FramedRead, BytesCodec};
use warp::Filter;
use warp::http::Response;
use warp::hyper::Body;

#[tokio::main]
async fn main() {
    let routes = warp::any().and_then(|| async {
        let args: Vec<String> = std::env::args().collect();
        match File::open(&args[1]).await {
            Ok(file) => {
                let stream = FramedRead::with_capacity(file, BytesCodec::new(), 1024 * 16);
                Ok(Response::builder()
                   .status(200)
                   .header("cache-control", "no-store")
                   .header("content-type", "video/MP2T")
                   .header("connection", "close")
                   .body(Body::wrap_stream(stream)))
            }
            Err(_) => Err(warp::reject::not_found()),
        }
    });
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

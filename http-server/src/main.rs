use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod routes;

async fn routes(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = request.method();
    let path = request.uri().path();

    match (method, path) {
        (&Method::GET, "/") => routes::users::index(request),
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::from(Bytes::from("404 Not Found")))
                .unwrap();
            Ok(response)
        }
    }
}

async fn run_server(addr: SocketAddr) {
    
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(routes))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr).await;
    Ok(())
}
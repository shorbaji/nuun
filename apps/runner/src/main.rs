use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

struct Runner {
}

impl Runner {
    pub fn new() -> Self {
        Runner {}
    }

    pub async fn eval(&self, code: &str) -> Result<dal::Object, Box<dyn std::error::Error + Send + Sync>> {
        let mut machine = dal::Machine::new();
        machine.eval(code).await
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let runner = Runner::new();
    _ = runner.eval("(+ 1 2)").await;

    Ok(Response::new(Full::from(Bytes::from("Hello, world!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error: {}", err);
            }
        });
    }
}

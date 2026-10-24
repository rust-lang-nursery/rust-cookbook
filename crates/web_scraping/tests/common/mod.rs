//! A tiny in-process HTTP server so the tests never touch the real network.

use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Serves `routes` (path -> (status line, body)) on an ephemeral localhost
/// port. Unknown paths get a 404.
pub async fn serve(routes: &[(&str, &str, &str)]) -> SocketAddr {
    serve_on("127.0.0.1:0".parse().expect("addr"), routes).await
}

/// Like [`serve`], but binds a specific address.
pub async fn serve_on(addr: SocketAddr, routes: &[(&str, &str, &str)]) -> SocketAddr {
    let routes: HashMap<String, (String, String)> = routes
        .iter()
        .map(|(path, status, body)| (path.to_string(), (status.to_string(), body.to_string())))
        .collect();
    let listener = TcpListener::bind(addr).await.expect("bind");
    let addr = listener.local_addr().expect("local addr");

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let routes = routes.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let Ok(n) = socket.read(&mut buf).await else {
                    return;
                };
                let request = String::from_utf8_lossy(&buf[..n]);
                let path = request.split_whitespace().nth(1).unwrap_or("/");
                let (status, body) = routes
                    .get(path)
                    .cloned()
                    .unwrap_or_else(|| ("404 Not Found".into(), String::new()));
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.shutdown().await;
            });
        }
    });

    addr
}


use rust_http::{
    http1::handler::Http1Socket, traits::HttpSocket
};

#[tokio::main]
async fn main() {
    println!("listenening http://0.0.0.0:4096/");
    rust_http::listener::http_listener("0.0.0.0:4096",listener).await.unwrap();
}

async fn listener(mut conn: Http1Socket){
    conn.close(b"after a long time").await.unwrap();
}
use axum::{
    Router,
    routing::{get,post},
    response::Html,
    // http::HeaderMap,
    extract::Multipart,
};
use std::net::SocketAddr;
// use serde::{Serialize,Deserialize};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(my_index))
        .route("/upload", post(upload));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}


async fn my_index() ->Html<&'static str> {
    Html(include_str!("../html/index.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
        println!("Done!");
    }
}
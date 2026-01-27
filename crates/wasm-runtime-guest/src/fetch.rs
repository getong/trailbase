use wstd::http::Client;
use wstd::io::empty;

pub use http::{Request, Uri};

pub async fn fetch<B: wstd::http::Body>(request: Request<B>) -> Result<Vec<u8>, wstd::http::Error> {
  let client = Client::new();
  println!("fetch 0");
  let response = client.send(request).await?;
  println!("fetch 1");
  let x = response.into_body().bytes().await;
  println!("fetch 2");
  return x;
}

pub async fn get(uri: impl Into<http::Uri>) -> Result<Vec<u8>, wstd::http::Error> {
  return fetch(
    Request::builder()
      .uri(uri.into())
      .body(empty())
      .expect("static"),
  )
  .await;
}

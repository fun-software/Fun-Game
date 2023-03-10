mod fbs;
mod utils;

use dotenv::{dotenv, var};
use std::net::SocketAddr;
use tokio::runtime;

use hyper::{
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Error,
};

use utils::{
  http_service::http_service,
  state::{AsyncState, State},
};

#[allow(non_snake_case)]
fn main() {
  dotenv().ok();
  let api_port = var("LISTEN_PORT").unwrap_or("8080".to_string());
  let api_url = var("SERVER_URL").unwrap_or("127.0.0.1".to_string());
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

  let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("could not create tokio runtime");

  let http_api_address: SocketAddr = format!("{api_url}:{api_port}").parse().unwrap();

  rt.block_on(async {
    let state: AsyncState = State::new();

    // define the http service
    let local_state = state.clone();
    let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
      let remote_addr = addr_stream.remote_addr();
      let local_state = local_state.clone();

      async move {
        let local_state = local_state.clone();
        Ok::<_, Error>(service_fn(move |req| {
          let local_state = local_state.clone();
          async move {
            let local_state = local_state.clone();
            http_service(req, remote_addr, local_state).await
          }
        }))
      }
    });

    // start the http service
    let server = hyper::server::Server::bind(&http_api_address)
      .serve(make_svc)
      .await;

    if let Err(err) = server {
      log::error!("server error: {}", err);
    }
  });
}

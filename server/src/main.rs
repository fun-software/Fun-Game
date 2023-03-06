mod fbs;
mod utils;

use std::net::SocketAddr;

use hyper::{
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Error,
};
use tokio::runtime;

use dotenv::dotenv;

use utils::http_service::http_service;

#[allow(non_snake_case)]
fn main() {
  dotenv().ok();

  let listen_port: String = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set.");
  let server_url: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set.");

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("could not create tokio runtime");

  let http_api_address: SocketAddr = format!("{server_url}:{listen_port}").parse().unwrap();

  rt.block_on(async {
    let state = utils::state::new_state();

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
    hyper::server::Server::bind(&http_api_address)
      .serve(make_svc)
      .await
      .expect("HTTP session server has died");
  });
}

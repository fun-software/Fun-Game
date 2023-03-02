mod fbs;
mod utils;

use std::net::SocketAddr;

use hyper::{
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Error,
};
use tokio::{
  runtime,
  time::{self, Duration},
};
use webrtc_unreliable::Server;

use dotenv::dotenv;

use utils::{handler::handle_msg, http_service::http_service};

#[allow(non_snake_case)]
fn main() {
  dotenv().ok();

  let PUBLIC_PORT: String = std::env::var("PUBLIC_PORT").expect("PUBLIC_PORT must be set.");
  let WEBRTC_PORT: String = std::env::var("WEBRTC_PORT").expect("WEBRTC_PORT must be set.");
  let LISTEN_PORT: String = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set.");

  let SERVER_URL: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set.");

  let TICK_RATE: u64 = std::env::var("TICK_RATE").unwrap().parse::<u64>().unwrap();

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

  let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("could not create tokio runtime");

  let public_webrtc_addr: SocketAddr = format!("{SERVER_URL}:{PUBLIC_PORT}").parse().unwrap();
  let webrtc_listen_addr: SocketAddr = format!("{SERVER_URL}:{WEBRTC_PORT}").parse().unwrap();
  let session_listen_addr: SocketAddr = format!("{SERVER_URL}:{LISTEN_PORT}").parse().unwrap();

  rt.block_on(async {
    tokio::spawn(async move {
      let mut interval = time::interval(Duration::from_millis(TICK_RATE));

      loop {
        interval.tick().await;

        // flush the queue, make state updates
        // println!("Tick")
      }
    });

    let state = utils::state::new_state();

    let mut rtc_server = Server::new(webrtc_listen_addr, public_webrtc_addr)
      .await
      .expect("could not start RTC server");

    let session_endpoint = rtc_server.session_endpoint();

    let local_state = state.clone();
    let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
      let session_endpoint = session_endpoint.clone();
      let remote_addr = addr_stream.remote_addr();
      let local_state = local_state.clone();

      async move {
        let local_state = local_state.clone();
        Ok::<_, Error>(service_fn(move |req| {
          let local_state = local_state.clone();
          let mut session_endpoint = session_endpoint.clone();
          async move { http_service(req, remote_addr, &mut session_endpoint, local_state).await }
        }))
      }
    });

    tokio::spawn(async move {
      hyper::server::Server::bind(&session_listen_addr)
        .serve(make_svc)
        .await
        .expect("HTTP session server has died");
    });

    let mut message_buf = Vec::<u8>::new();
    loop {
      let received = match rtc_server.recv().await {
        Ok(received) => {
          message_buf.clear();
          message_buf.extend(received.message.as_ref());
          Some((received.message_type, received.remote_addr))
        }
        Err(err) => {
          log::warn!("could not receive RTC message: {:?}", err);
          None
        }
      };

      if let Some((message_type, remote_addr)) = received {
        let local_state = state.clone();
        let res = handle_msg(&message_buf, message_type, remote_addr, local_state);
        message_buf.clear();

        message_buf.extend(res);

        match rtc_server
          .send(&message_buf, message_type, &remote_addr)
          .await
        {
          Ok(_) => {}
          Err(err) => {
            log::warn!("could not send RTC message: {:?}", err);
          }
        }
      }
    }
  });
}

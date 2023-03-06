use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::time::{self, Duration};
use webrtc_unreliable::Server;

use super::state::ArcState;

pub async fn web_rtc_service(state: ArcState, game_id: String) {
  dotenv().ok();

  let public_port: String = std::env::var("PUBLIC_PORT").expect("PUBLIC_PORT must be set.");
  let webrtc_port: String = std::env::var("WEBRTC_PORT").expect("WEBRTC_PORT must be set.");
  let server_url: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set.");
  let tick_rate: u64 = std::env::var("TICK_RATE").unwrap().parse::<u64>().unwrap();

  let public_webrtc_addr: SocketAddr = format!("{server_url}:{public_port}").parse().unwrap();
  let webrtc_listen_addr: SocketAddr = format!("{server_url}:{webrtc_port}").parse().unwrap();

  let mut rtc_server = Server::new(webrtc_listen_addr, public_webrtc_addr)
    .await
    .expect("could not start RTC server");

  let session_endpoint = rtc_server.session_endpoint();

  state
    .read()
    .await
    .web_rtc_sessions
    .write()
    .await
    .insert(game_id, session_endpoint);

  tokio::spawn(async move {
    let mut interval = time::interval(Duration::from_millis(tick_rate));

    loop {
      interval.tick().await;

      // flush the queue, make state updates
      // println!("Tick")
    }
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
      match rtc_server
        .send(&message_buf, message_type, &remote_addr)
        .await
      {
        Ok(_) => {
          // echo the message back to the client
          message_buf.clear();
        }
        Err(err) => {
          log::warn!("could not send RTC message: {:?}", err);
        }
      }
    }
  }
}

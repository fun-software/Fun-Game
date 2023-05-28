use dotenv::var;
use std::net::SocketAddr;
use tokio::{
  net::TcpListener,
  time::{self, Duration},
};
use webrtc_unreliable::Server;

use super::state::AsyncState;

pub async fn web_rtc_service(listener: TcpListener, state: AsyncState, game_id: &String) {
  let tick_rate: u64 = var("TICK_RATE")
    .unwrap_or("16".to_string())
    .parse()
    .unwrap();

  // host the WebRTC server on same address both publicly and locally
  let webrtc_host: SocketAddr = listener.local_addr().unwrap();
  let mut rtc_server = Server::new(webrtc_host, webrtc_host)
    .await
    .expect("could not start RTC server");

  let session_endpoint = rtc_server.session_endpoint();

  let mut inner_state = state.write().await;
  inner_state
    .web_rtc_sessions
    .insert(game_id.to_string(), session_endpoint);

  let inner_game_id = game_id.clone();
  let state_clone1 = inner_state.clone();

  drop(inner_state);
  tokio::spawn(async move {
    let mut interval = time::interval(Duration::from_millis(tick_rate));

    let current_game = &state_clone1.game_state_map[&inner_game_id];

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
          // echoed the message back to the client
          message_buf.clear();
        }
        Err(err) => {
          log::warn!("could not send RTC message: {:?}", err);
        }
      }
    }
  }
}

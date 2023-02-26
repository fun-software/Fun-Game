extern crate tokio;
use clap::{Arg, Command};
use hyper::{
    header::{self, HeaderValue},
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Body, Error, Method, Response, StatusCode,
};
use tokio::{
    runtime,
};

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let matches = Command::new("echo-server")
            .arg(
                Arg::new("data")
                    .short('d')
                    .long("data")
                    .takes_value(true)
                    .required(true)
                    .help("listen on the specified address/port for UDP WebRTC data channels"),
            )
            .arg(
                Arg::new("public")
                    .short('p')
                    .long("public")
                    .takes_value(true)
                    .required(true)
                    .help("advertise the given address/port as the public WebRTC address/port"),
            )
            .arg(
                Arg::new("http")
                    .short('h')
                    .long("http")
                    .takes_value(true)
                    .required(true)
                    .help("listen on the specified address/port for incoming HTTP (session reqeusts and test page"),
            )
            .get_matches();

        tokio::spawn(async move {
            // receieve messages on channel, handle it
        });

        tokio::spawn(async move {
            // wait for tick, update state from state_queue (flush)
        });

        let webrtc_listen_addr = matches
            .value_of("data")
            .unwrap()
            .parse()
            .expect("could not parse WebRTC data address/port");

        let public_webrtc_addr = matches
            .value_of("public")
            .unwrap()
            .parse()
            .expect("could not parse advertised public WebRTC data address/port");

        let session_listen_addr = matches
            .value_of("http")
            .unwrap()
            .parse()
            .expect("could not parse HTTP address/port");

        let mut rtc_server =
            webrtc_unreliable::Server::new(webrtc_listen_addr, public_webrtc_addr)
                .await
                .expect("could not start RTC server");

        let session_endpoint = rtc_server.session_endpoint();
        let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
            let session_endpoint = session_endpoint.clone();
            let remote_addr = addr_stream.remote_addr();
            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let mut session_endpoint = session_endpoint.clone();
                    async move {
                        if req.uri().path() == "/offer" && req.method() == Method::POST
                        {
                            log::info!("WebRTC session request from {}", remote_addr);
                            match session_endpoint.http_session_request(req.into_body()).await {
                                Ok(mut resp) => {
                                    resp.headers_mut().insert(
                                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                        HeaderValue::from_static("*"),
                                    );
                                    Ok(resp.map(Body::from))
                                }
                                Err(err) => {
                                    log::warn!("bad rtc session request: {:?}", err);
                                    Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("error: {:?}", err)))
                                }
                            }
                        } else {
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("not found"))
                        }
                    }
                }))
            }
        });

        tokio::spawn(async move {
            Server::bind(&session_listen_addr)
                .serve(make_svc)
                .await
                .expect("HTTP session server has died");
        });

        let mut message_buf = Vec::new();
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
                if let Err(err) = rtc_server
                    .send(&message_buf, message_type, &remote_addr)
                    .await
                {
                    log::warn!("could not send message to {}: {:?}", remote_addr, err);
                }
            }
        }
    });
}

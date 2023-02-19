extern crate futures;
extern crate tokio;
extern crate websocket;

pub mod types;

use tokio::runtime;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;
use websocket::server::r#async::Server;

use futures::{Future, Stream, Sink};
use futures::future::{self, Loop};

fn main() {
    let runtime  = runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let server   = Server::bind("localhost:8080", &runtime.reactor()).expect("Failed to create server");

    let connections = Arc::new(RwLock::new(HashMap::new()));
    let entities    = Arc::new(RwLock::new(HashMap::new()));
    let counter     = Arc::new(RwLock::new(0));

    let connections_inner = connections.clone();
    let entities_inner    = entities.clone();
    let executor_inner    = executor.clone();

    let connection_handler = server.incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(move |(upgrade, _)| {
            let connections_inner = connections_inner.clone();
            let entities          = entities_inner.clone();
            let counter_inner     = counter.clone();
            let executor_2inner   = executor_inner.clone();

            let accept = upgrade.accept().and_then(move |(framed,_)| {
                let (sink, stream) = framed.split();

                {
                    let mut c = counter_inner.write().unwrap();
                    *c += 1;
                }

                let id = *counter_inner.read().unwrap();
                connections_inner.write().unwrap().insert(id,sink);
                entities.write().unwrap().insert(id, types::Entity{id, pos:(0,0)} );
                let c = *counter_inner.read().unwrap();

                let f = stream.for_each(move |msg| {
                    process_message(c, &msg, entities.clone());
                    Ok(())
                }).map_err(|_| ());

                executor_2inner.spawn(f);

                Ok(())
            }).map_err(|_| ());

            executor_inner.spawn(accept);
            Ok(())
        })
        .map_err(|_| ());

    let send_handler = future::loop_fn((), move |_| {
        let connections_inner = connections.clone();
        let executor          = executor.clone();
        let entities_inner    = entities.clone();

        tokio::timer::Delay::new(Instant::now() + Duration::from_millis(100))
            .map_err(|_| ())
            .and_then(move |_| {
                let mut conn = connections_inner.write().unwrap();
                let ids = conn.iter().map(|(k, _)| { k.clone() }).collect::<Vec<_>>();

                for id in ids.iter() {
                    // Nit: need to remove (k, v) from hashmap or else we can't
                    // take ownership of it. If anyone knows a cleaner way, please edit
                    let sink = conn.remove(id).unwrap();

                    // For now, serializing this into json.
                    // Will need to move this into a flatbuffer
                    let entities = entities_inner.read().unwrap();
                    let first = match entities.iter().take(1).next() {
                        Some((_,e)) => e,
                        None => return Ok(Loop::Continue(())),
                    };
                    let serial_entities = format!("[{}]", entities.iter().skip(1)
                                                  .map(|(_,e)| e.to_json())
                                                  .fold(first.to_json(), |acc,s| format!("{},{}",s,acc)));
                    let connections = connections_inner.clone();
                    let id = id.clone();

                    let f = sink
                        .send(OwnedMessage::Text(serial_entities))
                        .and_then(move |sink| {
                            connections.write().unwrap().insert( id.clone(), sink );
                            Ok(())
                        })
                        .map_err(|_| ());

                    executor.spawn(f);
                }

                match true {
                    true => Ok(Loop::Continue(())),
                    false => Ok(Loop::Break(())),
                }
            })
    });

    runtime
        .block_on_all(connection_handler.select(send_handler))
        .map_err(|_| println!("Error conn-send loop"))
        .unwrap();
}

fn process_message(
    id: i32,
    msg: &OwnedMessage,
    entities: Arc<RwLock<HashMap<i32,types::Entity>>>
) {
    if let OwnedMessage::Text(ref action) = *msg {
        println!("Received msg '{}' from id {}", action, id);

        if action == "right" {
            entities.write().unwrap().entry(id).and_modify(|e| { e.pos.0 += 10 });
        }
        else if action == "left" {
            entities.write().unwrap().entry(id).and_modify(|e| { e.pos.0 -= 10 });
        }
        else if action == "down" {
            entities.write().unwrap().entry(id).and_modify(|e| { e.pos.1 += 10 });
        }
        else if action == "up" {
            entities.write().unwrap().entry(id).and_modify(|e| { e.pos.1 -= 10 });
        }
    }
}

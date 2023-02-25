extern crate futures;
extern crate tokio;
extern crate websocket;

pub mod types;

use tokio::{
    net::UdpSocket,
    runtime,
    sync::{
        broadcast::{self, Receiver},
        RwLock,
    },
    time::{self, Duration},
};
use types::Entities;

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    net::SocketAddr,
    sync::Arc,
};

// Maps player_id -> receiver for broadcasts
type PlayerReceiverMap = HashMap<u64, Receiver<(Vec<u8>, Option<SocketAddr>)>>;

fn main() {
    let entities = Arc::new(RwLock::new(types::Entities::new()));
    let client_receivers = Arc::new(RwLock::new(PlayerReceiverMap::new()));

    let (sender_channel, mut receiver_channel) =
        broadcast::channel::<(Vec<u8>, Option<SocketAddr>)>(16);

    // spawn a new task to notify on msgs sent to the channel receiver
    tokio::spawn(async move {
        loop {
            match receiver_channel.recv().await {
                Ok((msg, player_addr)) => match player_addr {
                    Some(addr) => match sender_socket.send_to(&msg, addr).await {
                        Ok(len) => {
                            println!("Sent {} bytes to {}", len, addr);
                        }
                        Err(e) => {
                            println!("Error sending to socket: {}", e);
                        }
                    },
                    None => (),
                },
                Err(e) => {
                    println!("Error receiving from channel: {}", e);
                }
            }
        }
    });

    let broadcast_entities = entities.read().await.clone();
    let broadcast_sender = sender_channel.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;

            let msg = broadcast_entities
                .iter()
                .fold(String::new(), |acc, (_, v)| {
                    return acc + &format!("{}: {}", v.id, v.pos.to_json());
                });

            match broadcast_sender.send((msg.as_bytes().to_vec(), None)) {
                Ok(_) => println!("Sent broadcast: {}", msg),
                Err(e) => {
                    println!("Error sending to channel: {}", e);
                }
            }
        }
    });

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = match receiver_socket.recv_from(&mut buf).await {
            Ok((len, addr)) => (len, addr),
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                continue;
            }
        };

        // identify sender
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        addr.hash(&mut hasher);
        let id = hasher.finish();
        let is_playing = entities.read().await.contains_key(&id);

        let msg = &buf[..len];

        match msg {
            b"play\n" => match is_playing {
                true => {
                    println!("Player {} is already playing", id);
                    continue;
                }
                false => {
                    register_player(
                        id,
                        sender_channel.subscribe(),
                        &client_receivers,
                        &entities,
                    )
                    .await;
                    println!("Registered player {} with address {}\n", id, addr)
                }
            },
            b"quit\n" => match is_playing {
                true => {
                    unregister_player(id, &client_receivers, &entities).await;
                    println!("Unregistered player {} with address {}\n", id, addr)
                }
                false => {
                    println!("Player {} is not playing", id);
                    continue;
                }
            },
            b"w\n" | b"s\n" | b"a\n" | b"d\n" => match is_playing {
                false => {
                    continue;
                }
                true => process_move(msg[0] as char, id, &entities).await,
            },
            _ => println!("Unknown command: {}", String::from_utf8_lossy(msg)),
        };
    }

    runtime
        .block_on_all(connection_handler.select(send_handler))
        .map_err(|_| println!("Error conn-send loop"))
        .unwrap();
}

async fn register_player(
    id: u64,
    receiver: Receiver<(Vec<u8>, Option<SocketAddr>)>,
    client_receivers: &Arc<RwLock<PlayerReceiverMap>>,
    entities: &Arc<RwLock<Entities>>,
) {
    entities.write().await.insert(
        id,
        types::Entity {
            id,
            pos: types::Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    );

    client_receivers.write().await.insert(id, receiver);
}

async fn unregister_player(
    id: u64,
    client_receivers: &Arc<RwLock<PlayerReceiverMap>>,
    entities: &Arc<RwLock<Entities>>,
) {
    entities.write().await.remove(&id);

    client_receivers.write().await.remove(&id);
}

async fn process_move(dir: char, id: u64, entities: &Arc<RwLock<Entities>>) {
    entities
        .write()
        .await
        .entry(id)
        .and_modify(|player| match dir {
            'w' => player.pos.z += 1.0,
            's' => player.pos.z -= 1.0,
            'a' => player.pos.x -= 1.0,
            'd' => player.pos.x += 1.0,
            _ => (),
        });

    println!("Player {} moved {}", id, dir);
}
>>>>>>> e0fa536 (why server broadcasts wont work)

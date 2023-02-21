extern crate futures;
extern crate tokio;
extern crate websocket;

pub mod types;

use tokio::{
    net::UdpSocket,
    runtime,
    sync::{mpsc, RwLock},
};
use types::Entities;

use std::{
    hash::{Hash, Hasher},
    net::SocketAddr,
    sync::Arc,
};

fn main() {
    // Create a new tokio runtime with io and time drivers enabled
    // https://docs.rs/tokio/latest/tokio/runtime/index.html#resource-drivers
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // start the runtime
    rt.block_on(async {
        // Create a new socket connection
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();

        let r = Arc::new(sock);
        let s = r.clone();

        let entities = Arc::new(RwLock::new(types::Entities::new()));

        // multiple producers, single consumer channel
        // tx -> sender, rx -> receiver
        let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1024);

        // spawn a new task to notify on msgs sent to the channel rx
        tokio::spawn(async move {
            println!("Waiting for messages... ");
            while let Some((msg, addr)) = rx.recv().await {
                let len = s.send_to(&msg, addr).await.unwrap();
                println!("Sent {} bytes to {}", len, addr);
            }
        });

        let mut buf = [0u8; 1024];
        // handle incoming messages
        loop {
            let (len, addr) = r.recv_from(&mut buf).await.unwrap();
            println!("Received {} bytes from {}", len, addr);
            let msg = &buf[..len];
            let res = match msg {
                b"play\n" => process_play(addr, &entities).await,
                b"quit\n" => process_quit(addr, &entities).await,
                b"w\n" | b"s\n" | b"a\n" | b"d\n" => {
                    process_move(msg[0] as char, addr, &entities).await
                }
                _ => b"magoo\n".to_vec(),
            };

            tx.send((res, addr)).await.unwrap();
        }
    });
}

async fn process_play(addr: SocketAddr, entities: &Arc<RwLock<Entities>>) -> Vec<u8> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    addr.hash(&mut hasher);
    let id = hasher.finish();
    if entities.read().await.contains_key(&id) {
        return b"Already playing\n".to_vec();
    } else {
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

        return format!("You are player {} with address {}\n", id, addr)
            .as_bytes()
            .to_vec();
    }
}

async fn process_quit(addr: SocketAddr, entities: &Arc<RwLock<Entities>>) -> Vec<u8> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    addr.hash(&mut hasher);
    let id = hasher.finish();
    if entities.read().await.contains_key(&id) {
        entities.write().await.remove(&id);
        return format!("Player {} quit.\n", id).as_bytes().to_vec();
    }

    return b"Not playing\n".to_vec();
}

async fn process_move(dir: char, addr: SocketAddr, entities: &Arc<RwLock<Entities>>) -> Vec<u8> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    addr.hash(&mut hasher);
    let id = hasher.finish();
    if !entities.read().await.contains_key(&id) {
        return b"Try typing 'play' first\n".to_vec();
    }

    let r_entities = entities.read().await.clone();
    let player = r_entities.get(&id).unwrap();

    let mut pos = player.pos.clone();
    match dir {
        'w' => pos.z += 1.0,
        's' => pos.z -= 1.0,
        'a' => pos.x -= 1.0,
        'd' => pos.x += 1.0,
        _ => (),
    }

    entities.write().await.insert(id, types::Entity { id, pos });
    let ret = pos.to_json() + "\n";

    return ret.as_bytes().to_vec();
}

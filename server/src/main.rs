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
        let socket = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();

        let receiver_socket = Arc::new(socket);
        let sender_socket = receiver_socket.clone();

        let entities = Arc::new(RwLock::new(types::Entities::new()));
        let mut client_sockets = Arc::new(RwLock::new(Vec::new()));

        let (sender_channel, mut receiver_channel) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1024);

        // spawn a new task to notify on msgs sent to the channel receiver
        tokio::spawn(async move {
            println!("Waiting for messages... ");
            while let Some((msg, player_addr)) = receiver_channel.recv().await {
                let len = sender_socket.send_to(&msg, player_addr).await.unwrap();
                //let len = sender_socket.send(&msg).await.unwrap();
                println!("Sent {} bytes to {}", len, player_addr);
            }
        });

        let mut buf = [0u8; 1024];
        loop {
            let (len, addr) = receiver_socket.recv_from(&mut buf).await.unwrap();
            println!("Received {} bytes from {}", len, addr);
            let msg = &buf[..len];
            let game_state = match msg {
                b"play\n" => process_play(addr, &client_sockets, &entities).await,
                b"quit\n" => process_quit(addr, &entities).await,
                b"w\n" | b"s\n" | b"a\n" | b"d\n" => {
                    process_move(msg[0] as char, addr, &entities).await
                }
                _ => b"Not implemented\n".to_vec(),
            };

            // NOTE: probably don't need this. Imo, functionality should be:
            // user makes a change, and we end up in the above loop. We
            // process the change, and then all players receive an update
            // on the next ticket. This isn't like a normal server that's based
            // on send/response schema. It's send, make a change to global state,
            // update on subsquent tick. Not sure the proper housing for that
            // functionality, however. I'm guesing you're in agreement and that
            // this is just a relic of the boilerplate you found
            //
            // We need to figure out a way to broadcast state updates to all clients,
            // but I can't really figure out the best method for that. I see a broadcast
            // method on the UdpSocket docs, but I'm not really understanding what in the world
            // it's talking about. My current idea is to just store the player socket addrs in a
            // connections vector, which we iterate over periodically with another tokio
            // thread.
            sender_channel.send((game_state, addr)).await.unwrap();
        }
    });
}

async fn process_play(addr: SocketAddr, client_sockets: &Arc<RwLock<Vec<SocketAddr>>>, entities: &Arc<RwLock<Entities>>) -> Vec<u8> {
    client_sockets.write().await.push(addr);
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

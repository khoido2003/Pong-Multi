use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use super::player::Player;

#[derive(Debug)]
pub struct Room {
    pub id: Uuid,
    pub players: HashMap<SocketAddr, Arc<Mutex<Player>>>,
}

impl Room {
    pub fn new(
        player1: Arc<Mutex<Player>>,
        player2: Arc<Mutex<Player>>,
    ) -> (Uuid, Arc<Mutex<Self>>) {
        let room_id = Uuid::new_v4();
        let mut players: HashMap<SocketAddr, Arc<Mutex<Player>>> = HashMap::new();

        let p1 = player1.lock().unwrap();
        let p2 = player2.lock().unwrap();

        players.insert(p1.addr, player1.clone());
        players.insert(p2.addr, player2.clone());

        drop(p1);
        drop(p2);

        let room = Arc::new(Mutex::new(Self {
            id: room_id,
            players,
        }));

        (room_id, room)
    }
    pub async fn start(self: Arc<Self>) {}
}

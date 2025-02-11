use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;
use uuid::Uuid;

use super::{
    player::{Player, PlayerStatus},
    room::Room,
};

#[derive(Debug)]
pub struct MatchMaker {
    pub queue: Arc<Mutex<VecDeque<SocketAddr>>>,
    pub rooms: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Room>>>>>,
    pub players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>>,
    pub tx: mpsc::Sender<(usize, SocketAddr, Vec<u8>)>,

    pub player_room_map: Arc<Mutex<HashMap<SocketAddr, Uuid>>>,
}

impl MatchMaker {
    pub fn new(
        players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>>,
        tx: mpsc::Sender<(usize, SocketAddr, Vec<u8>)>,
        rooms: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Room>>>>>,
        player_room_map: Arc<Mutex<HashMap<SocketAddr, Uuid>>>,
    ) -> Self {
        Self {
            players,
            queue: Arc::new(Mutex::new(VecDeque::new())),
            rooms,
            tx,
            player_room_map,
        }
    }

    pub fn add_to_queue(&self, addr: &SocketAddr) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(*addr);

        println!("Player {:?} added to the matchmaking queue", addr);

        drop(queue);
        self.try_create_room();
    }

    fn try_create_room(&self) {
        let mut queue = self.queue.lock().unwrap();
        let mut rooms = self.rooms.lock().unwrap();
        let mut player_room_map = self.player_room_map.lock().unwrap();
        let players_map = self.players.lock().unwrap();

        if queue.len() >= 2 {
            let addr1 = queue.pop_front().unwrap();
            let addr2 = queue.pop_front().unwrap();

            if let (Some(player1), Some(player2)) =
                (players_map.get(&addr1), players_map.get(&addr2))
            {
                let mut p1 = player1.lock().unwrap();
                let mut p2 = player2.lock().unwrap();

                p1.status = PlayerStatus::InMatch;
                p2.status = PlayerStatus::InMatch;

                // Create new room
                let (id, room) = Room::new(player1.clone(), player2.clone());

                // Add the room to room_manegement
                rooms.insert(id, room.clone());

                // Add player address and room_id to navigate later
                player_room_map.insert(addr1, id);
                player_room_map.insert(addr2, id);

                drop(player_room_map);

                let room = room.lock().unwrap();
                println!(
                    "Room {} created with player {:?} and {:?}",
                    room.id, addr1, addr2
                );
                drop(room);
            }
        }
        drop(queue);
        drop(rooms);
    }
}

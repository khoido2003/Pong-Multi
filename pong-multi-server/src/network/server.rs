use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    net::UdpSocket,
    sync::{mpsc, Semaphore},
};

use serde_json::Value;
use uuid::Uuid;

use super::{match_maker::MatchMaker, player::Player, room::Room};

#[derive(Debug, Clone)]
pub struct Server {
    pub socket: Arc<UdpSocket>,
    pub message_queue: mpsc::Sender<(usize, SocketAddr, Vec<u8>)>,
    pub players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>>,
    pub match_maker: Arc<MatchMaker>,
    pub rooms: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Room>>>>>,
    pub player_room_map: Arc<Mutex<HashMap<SocketAddr, Uuid>>>,
}

impl Server {
    pub async fn new(addr: &str) -> Arc<Self> {
        let socket = Arc::new(UdpSocket::bind(addr).await.expect("Failed to bind socket"));

        // Create a queue_message to receive messae from client send in UDP
        let (tx, rx) = mpsc::channel::<(usize, SocketAddr, Vec<u8>)>(1000);

        // Init the players list current online
        let players: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Player>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Init the rooms list
        let rooms: Arc<Mutex<HashMap<Uuid, Arc<Mutex<Room>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Init player_room_map
        let player_room_map: Arc<Mutex<HashMap<SocketAddr, Uuid>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Init match_maker to create match for player
        let match_maker = Arc::new(MatchMaker::new(
            players.clone(),
            tx.clone(),
            rooms.clone(),
            player_room_map.clone(),
        ));

        // Create the server
        let server = Arc::new(Server {
            socket: socket.clone(),
            message_queue: tx.clone(),
            players,
            match_maker: match_maker.clone(),
            rooms: rooms.clone(),
            player_room_map: player_room_map.clone(),
        });

        // Spawn the receiver worker
        let server_clone = server.clone();
        tokio::spawn(async move {
            server_clone.receive_loop().await;
        });

        // Spawn worker pool for processing task
        let server_clone = server.clone();
        tokio::spawn(async move {
            server_clone.task_worker(rx).await;
        });

        server
    }

    // Receive message from client and push to message_queue to process later
    async fn receive_loop(self: Arc<Self>) {
        let mut buf = [0; 1024];

        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let message_buf = buf[..len].to_vec();

                    if let Err(_) = self.message_queue.send((len, addr, message_buf)).await {
                        eprint!("Task queue is full! Dropping packet from {:?}", addr);
                    }
                }
                Err(e) => {
                    eprint!("Error receiving packet: {:?}", e);
                }
            }
        }
    }

    // Read message from the message_queue and pass to the process method
    async fn task_worker(self: Arc<Self>, mut rx: mpsc::Receiver<(usize, SocketAddr, Vec<u8>)>) {
        let semaphore = Arc::new(Semaphore::new(1000));

        while let Some((len, addr, buf)) = rx.recv().await {
            let server_clone = self.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            tokio::spawn(async move {
                server_clone.process(len, addr, buf).await;
                drop(permit);
            });
        }
    }

    // Process user request
    async fn process(self: Arc<Self>, len: usize, addr: SocketAddr, buf: Vec<u8>) {
        println!("Bytes received: {:?} from {:?}", len, addr);

        let received_str = String::from_utf8_lossy(&buf);

        println!("Message received: {:?}", received_str);

        match serde_json::from_str::<Value>(&received_str) {
            Ok(json) => {
                if let Some(action) = json["action"].as_str() {
                    match action {
                        "enter" => self.handle_enter(&addr).await,

                        "join" => self.handle_join(&addr).await,

                        "leave" => self.handle_leave().await,

                        "move" => self.handle_move().await,

                        _ => println!("Unknow action : {action}"),
                    }
                }
            }

            Err(_) => println!("Invalid JSON received from {:?}", addr),
        }
    }

    async fn handle_enter(&self, addr: &SocketAddr) {
        let mut players = self.players.lock().unwrap();
        if !players.contains_key(addr) {
            let player = Player::new(*addr);
            players.insert(*addr, player.clone());

            println!("New player connected: {:?}", addr);
        }
    }

    async fn handle_join(&self, addr: &SocketAddr) {
        self.match_maker.add_to_queue(addr);
    }

    async fn handle_move(&self) {}

    async fn handle_leave(&self) {}
}

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub enum PlayerStatus {
    #[default]
    Available,
    InMatch,
}

#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub position: (f32, f32),
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(addr: SocketAddr) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            id: Uuid::new_v4(),
            addr,
            status: PlayerStatus::default(),
            position: (0.0, 0.0),
        }))
    }
}

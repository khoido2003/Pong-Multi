use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::UdpSocket,
    sync::{mpsc, Semaphore},
};

#[derive(Debug, Clone)]
pub struct Server {
    socket: Arc<UdpSocket>,
    message_queue: mpsc::Sender<(usize, SocketAddr, Vec<u8>)>,
}

impl Server {
    pub async fn new(addr: &str) -> Arc<Self> {
        let socket = Arc::new(UdpSocket::bind(addr).await.expect("Failed to bind socket"));

        let (tx, rx) = mpsc::channel::<(usize, SocketAddr, Vec<u8>)>(1000);

        let server = Arc::new(Server {
            socket: socket.clone(),
            message_queue: tx.clone(),
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

    async fn process(self: Arc<Self>, len: usize, addr: SocketAddr, buf: Vec<u8>) {
        println!("Bytes received: {:?} from {:?}", len, addr);

        let received_str = String::from_utf8_lossy(&buf);

        println!("Message received: {:?}", received_str);
    }

    // Old method
    async fn run(self: Arc<Self>) {
        let mut buf = [0; 1024];

        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let message_buf = buf[..len].to_vec();
                    let server_clone = self.clone();
                    let _ =
                        tokio::spawn(
                            async move { server_clone.process(len, addr, message_buf).await },
                        );
                }
                Err(e) => {
                    eprint!("Something went wrong!\n {:?}", e);
                }
            };
        }
    }
}

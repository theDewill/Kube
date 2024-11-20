use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;

const DISCOVERY_PORT: u16 = 8338;
const BROADCAST_INTERVAL: u64 = 5; // seconds
const NODE_TIMEOUT: u64 = 15; // seconds

#[derive(Serialize, Deserialize, Clone, Debug)]
struct NodeInfo {
    id: String,
    ip: IpAddr,
    port: u16,
    last_seen: u64,
}

#[derive(Serialize, Deserialize)]
enum DiscoveryMessage {
    Announcement { node_id: String, service_port: u16 },
    Response { node_id: String, service_port: u16 },
}

pub struct NetworkDiscovery {
    socket: UdpSocket,
    node_id: String,
    service_port: u16,
    nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,
}

impl NetworkDiscovery {
    pub fn new(node_id: String, service_port: u16) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT))?;
        socket.set_broadcast(true)?;

        Ok(NetworkDiscovery {
            socket,
            node_id,
            service_port,
            nodes: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let socket_clone = self.socket.try_clone()?;
        let nodes_clone = Arc::clone(&self.nodes);
        let node_id_clone = self.node_id.clone();
        let service_port = self.service_port;

        // Spawn broadcast task
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(BROADCAST_INTERVAL));
            loop {
                interval.tick().await;
                let announcement = DiscoveryMessage::Announcement {
                    node_id: node_id_clone.clone(),
                    service_port,
                };
                let message = bincode::serialize(&announcement).unwrap();
                let _ = socket_clone.send_to(&message, ("255.255.255.255", DISCOVERY_PORT));
            }
        });

        // Listen for incoming messages
        let mut buf = [0u8; 1024];
        loop {
            let (size, addr) = self.socket.recv_from(&mut buf)?;
            let message: DiscoveryMessage = bincode::deserialize(&buf[..size])?;

            match message {
                DiscoveryMessage::Announcement {
                    node_id,
                    service_port,
                } => {
                    self.handle_announcement(node_id, addr.ip(), service_port)
                        .await;
                    // Send response
                    let response = DiscoveryMessage::Response {
                        node_id: self.node_id.clone(),
                        service_port: self.service_port,
                    };
                    let message = bincode::serialize(&response).unwrap();
                    let _ = self.socket.send_to(&message, addr);
                }
                DiscoveryMessage::Response {
                    node_id,
                    service_port,
                } => {
                    self.handle_announcement(node_id, addr.ip(), service_port)
                        .await;
                }
            }
        }
    }

    async fn handle_announcement(&self, node_id: String, ip: IpAddr, service_port: u16) {
        let now = Instant::now().elapsed().as_secs();
        let mut nodes = self.nodes.lock().unwrap();

        nodes.insert(
            node_id.clone(),
            NodeInfo {
                id: node_id,
                ip,
                port: service_port,
                last_seen: now,
            },
        );

        // Clean up old nodes
        nodes.retain(|_, info| now - info.last_seen < NODE_TIMEOUT);
    }

    pub fn get_active_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.lock().unwrap();
        nodes.values().cloned().collect()
    }
}

// Helper function to get local IP addresses
pub fn get_local_ips() -> Vec<IpAddr> {
    let mut ips = Vec::new();
    if let Ok(interfaces) = if_addrs::get_if_addrs() {
        for interface in interfaces {
            if !interface.is_loopback() {
                ips.push(interface.ip());
            }
        }
    }
    ips
}

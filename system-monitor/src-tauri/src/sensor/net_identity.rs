use sysinfo::Networks;
use std::net::UdpSocket;
use crate::sensor::ping_sensor::PingSensor;

#[derive(serde::Serialize, Clone)]
pub struct IdentityStats {
    pub net_ping: u32,
    pub net_interface: String,
    pub net_ip: String,
}

pub struct NetIdentity {
    ping_sensor: PingSensor,
    tick_counter: u8,
    cached_interface: String,
    cached_ip: String,
}

impl NetIdentity {
    pub fn new() -> Self {
        Self {
            ping_sensor: PingSensor::new(),
            tick_counter: 0,
            cached_interface: String::from("Loading..."),
            cached_ip: String::from("..."),
        }
    }

    pub fn read(&mut self, networks: &Networks) -> IdentityStats {
        if self.tick_counter % 5 == 0 {
            self.cached_interface = Self::find_primary_interface(networks);
            self.cached_ip = Self::fetch_local_ip();
        }
        self.tick_counter = self.tick_counter.wrapping_add(1);
        IdentityStats {
            net_ping: self.ping_sensor.read(),
            net_interface: self.cached_interface.clone(),
            net_ip: self.cached_ip.clone(),
        }
    }

    fn find_primary_interface(networks: &Networks) -> String {
        let mut best_iface = String::from("Disconnected");
        let mut max_traffic = 0;

        for (name, data) in networks {
            let name_lower = name.to_lowercase();
            if !name_lower.contains("loopback") && name != "lo" {
                let score = (data.received() + data.transmitted()) * 1000 + 
                            (data.total_received() + data.total_transmitted());
                if score > max_traffic {
                    max_traffic = score;
                    best_iface = name.clone(); 
                }
            }
        }
        best_iface
    }

    fn fetch_local_ip() -> String {
        UdpSocket::bind("0.0.0.0:0")
            .and_then(|socket| {
                socket.connect("8.8.8.8:80")?;
                socket.local_addr()
            })
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|_| "127.0.0.1".to_string())
    }
}
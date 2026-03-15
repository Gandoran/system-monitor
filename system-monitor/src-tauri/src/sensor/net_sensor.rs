use sysinfo::Networks;
use super::net_identity::{NetIdentity, IdentityStats};

#[derive(serde::Serialize, Clone)]
pub struct TrafficStats{
    pub net_history_download: Vec<f32>,
    pub net_history_upload: Vec<f32>,
    pub tot_download: f32,
    pub tot_upload: f32,
}

#[derive(serde::Serialize, Clone)]
pub struct NetStats {
    #[serde(flatten)]
    pub traffic: TrafficStats,
    #[serde(flatten)]
    pub identity: IdentityStats,
}

pub struct NetSensor {
    networks: Networks,
    net_history_download: Vec<f32>,
    net_history_upload: Vec<f32>,
    identity_sensor: NetIdentity,
}

impl NetSensor{
    pub fn new()->Self{
        Self{
            networks: Networks::new_with_refreshed_list(),
            net_history_download: vec![0.0;60],
            net_history_upload: vec![0.0;60],
            identity_sensor: NetIdentity::new(),
        }
    }

    pub fn read(&mut self)->NetStats{
        self.networks.refresh(true);
        let identity = self.identity_sensor.read(&self.networks);
        let traffic = self.calc_traffic();
        NetStats {
            traffic,
            identity,
        }
    }

    fn calc_traffic(&mut self) -> TrafficStats {
        let (mut current_rx, mut current_tx, mut total_rx, mut total_tx) = (0, 0, 0, 0);

        for (_name, data) in &self.networks {
            current_rx += data.received();        
            current_tx += data.transmitted(); 
            total_rx += data.total_received();    
            total_tx += data.total_transmitted();
        }

        let download_mb = current_rx as f32 / 1_048_576.0;
        let upload_mb = current_tx as f32 / 1_048_576.0;

        if self.net_history_download.len() >= 60 {
            self.net_history_download.remove(0);
            self.net_history_upload.remove(0);
        }
        self.net_history_download.push(download_mb);
        self.net_history_upload.push(upload_mb);

        TrafficStats {
            net_history_download: self.net_history_download.clone(),
            net_history_upload: self.net_history_upload.clone(),
            tot_download: total_rx as f32 / 1_073_741_824.0,
            tot_upload: total_tx as f32 / 1_073_741_824.0,
        }
    }
}
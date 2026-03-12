use sysinfo::Networks;

#[derive(serde::Serialize, Clone)]
pub struct NetStats{
    pub net_history_download: Vec<f32>,
    pub net_history_upload: Vec<f32>
}

pub struct NetSensor {
    networks: Networks,
    net_history_download: Vec<f32>,
    net_history_upload: Vec<f32>
}

impl NetSensor{
    pub fn new()->Self{
        Self{
            networks: Networks::new_with_refreshed_list(),
            net_history_download: vec![0.0;60],
            net_history_upload: vec![0.0;60]
        }
    }

    pub fn read(&mut self)->NetStats{
        self.networks.refresh(true);
        let (mut total_rx,mut total_tx) = (0,0);
        for (_interface_name, data) in &self.networks{
            total_rx += data.received();
            total_tx += data.transmitted();
        }
        let download_mb = total_rx as f32 / 1_048_576.0;
        let upload_mb = total_tx as f32 / 1_048_576.0;
        if self.net_history_download.len() >= 60 {
            self.net_history_download.remove(0);
            self.net_history_upload.remove(0);
        }
        self.net_history_download.push(download_mb);
        self.net_history_upload.push(upload_mb);
        NetStats{
            net_history_download: self.net_history_download.clone(),
            net_history_upload: self.net_history_upload.clone(),
        }
    }
}
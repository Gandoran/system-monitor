use std::net::{TcpStream, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub struct PingSensor{
    last_ping_ms: Arc<AtomicU32>,
}

impl PingSensor{
    pub fn new() -> Self{
        let last_ping_ms = Arc::new(AtomicU32::new(0));
        let ping_clone = Arc::clone(&last_ping_ms);
        
        thread::spawn(move ||{
            let target: SocketAddr = "8.8.8.8:53".parse().unwrap();
            let timeout = Duration::from_millis(1000);
            loop {
                let start = Instant::now();
                match TcpStream::connect_timeout(&target, timeout) {
                    Ok(_) => {
                        let elapsed = start.elapsed().as_millis() as u32;
                        ping_clone.store(elapsed, Ordering::Relaxed);
                    }
                    Err(_) => { ping_clone.store(999, Ordering::Relaxed);}
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
        Self { last_ping_ms }
    }

    pub fn read(&self) -> u32 {
        self.last_ping_ms.load(Ordering::Relaxed)
    }
}
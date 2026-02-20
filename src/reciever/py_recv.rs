use super::*;

use crate::logger::logger;

use std::{
    net::UdpSocket,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering
        },
    },
    time::Duration,
    io::ErrorKind,
    str::from_utf8,
};

impl PyCodeUdpReceiver{
    pub fn listen(&self, stop: Arc<AtomicBool>) -> Result<Vec<String>, String> {
        let addr = format!("{}:{}", self.host, self.port);
        let socket = UdpSocket::bind(&addr).map_err(|e| format!("Error at PyCodeUpdReciever: {}", e))?;

        socket.set_read_timeout(Some(Duration::from_millis(100))).map_err(|e| format!("Error at PyCodeUpdReciever socket: {}", e))?;
        logger().info(format!("Listening for messages on {}", addr));

        let mut buf = [0; 65535];
        let mut messages = Vec::new();

        while !stop.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((len, src)) => {
                    let data = &buf[..len];
                    if let Ok(msg) = from_utf8(data) {
                        logger().info(format!("[{}] {}", src, msg));
                        messages.push(msg.to_string());
                    } else {
                        logger().info(format!("[{}] Binary data: {:?}", src, data));
                        messages.push(format!("{:?}", data));
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => continue,
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(messages)
    }
}

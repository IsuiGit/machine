mod py_recv;

#[derive(Debug)]
pub struct PyCodeUdpReceiver{
    host: String,
    port: u16,
}

impl PyCodeUdpReceiver {
    pub fn new(host: &str, port: u16) -> Self {
        PyCodeUdpReceiver {
            host: host.to_string(),
            port: port,
        }
    }
}

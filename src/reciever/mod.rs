mod py_recv;

#[derive(Debug)]
pub struct PyCodeUdpReceiver{
    host: String,
    port: String,
}

impl PyCodeUdpReceiver {
    pub fn new() -> Self {
        PyCodeUdpReceiver {
            host: "127.0.0.1".to_string(),
            port: "9000".to_string(),
        }
    }
}

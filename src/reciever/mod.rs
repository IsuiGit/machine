mod py_recv;

#[derive(Debug)]
pub struct PyCodeUdpReceiver{
    host: String,
    port: String,
}

impl PyCodeUdpReceiver {
    pub fn new(host: String, port: String) -> Self {
        PyCodeUdpReceiver {
            host: host,
            port: port,
        }
    }
}

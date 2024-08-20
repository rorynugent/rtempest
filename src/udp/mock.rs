//! Functions supporting a mock tempest device

use std::net::UdpSocket;

pub struct MockSender {
    socket: UdpSocket,
}

impl MockSender {
    /// Bind to localhost with system assigned port
    pub fn bind() -> Self {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Unable to bind to address");

        MockSender { socket }
    }

    /// Send buffer to localhost with provided port
    pub fn send(&self, buffer: Vec<u8>, port: u16) {
        self.socket
            .send_to(&buffer, format!("127.0.0.1:{port}"))
            .expect("couldn't send data");
    }
}

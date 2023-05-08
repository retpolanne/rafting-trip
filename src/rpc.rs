use std::net::TcpListener;

pub struct RPC {
    pub bind_ip: String,
    pub bind_port: String,
}

impl RPC {
    pub fn listen(&self) {
        let listener = TcpListener::bind(
            format!("{}:{}", self.bind_ip, self.bind_port)
        ).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
        }
    }
}

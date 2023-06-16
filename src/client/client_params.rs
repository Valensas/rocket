#![allow(non_snake_case)]

pub struct ClientParams {
    pub ip_addr: String,
    pub port: String,
}

impl Clone for ClientParams {
    fn clone(&self) -> Self {
        Self {
            ip_addr: self.ip_addr.clone(),
            port: self.port.clone(),
        }
    }
}

impl ClientParams {
    pub fn new(ip_addr: String, port: String) -> Self {
        Self { ip_addr, port }
    }
}

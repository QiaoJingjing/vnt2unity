use std::process;


use vnt::handle::callback::{ConnectInfo, ErrorType};
use vnt::{DeviceInfo, ErrorInfo, HandshakeInfo, RegisterInfo, VntCallback};

#[derive(Clone)]
pub struct VntHandler {}

pub struct VntRunResult {
    pub result_code: i16,
    pub message: String,
}


impl VntCallback for VntHandler {
    fn success(&self){
        println!("====== Connect Successfully ======" );
    }

    fn create_tun(&self, info: DeviceInfo)  {
        println!("create_tun {}", info)
    }

    fn connect(&self, info: ConnectInfo)  {
        println!("connect {}", info)
    }

    fn handshake(&self, info: HandshakeInfo) -> bool  {
        println!("handshake {}", info);
        true
    }

    fn register(&self, info: RegisterInfo) -> bool  {
        println!("register {}", info);
        true
    }

    fn error(&self, info: ErrorInfo)  {
        match info.code {
            ErrorType::TokenError
            | ErrorType::AddressExhausted
            | ErrorType::IpAlreadyExists
            | ErrorType::InvalidIp
            | ErrorType::LocalIpExists => {
                self.stop();
            }
            _ => {}
        }

    }

    fn stop(&self)  {
        println!("stopped");

    }
}

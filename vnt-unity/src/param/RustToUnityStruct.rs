#[derive(Debug)]
pub struct ConnectInfo {
    pub count: i64,
    pub address: String,
}

#[derive(Debug)]
pub struct DeviceConfig {
    pub virtualIp: i32,
    pub virtualNetmask: i32,
    pub virtualGateway: i32,
    pub virtualNetwork: i32,
    pub externalRoute: Vec<String>,
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub enum ErrorCodeEnum {
    TokenError,
    Disconnect,
    AddressExhausted,
    IpAlreadyExists,
    InvalidIp,
    Unknown,
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub code: ErrorCodeEnum,
    pub msg: String,
}

#[derive(Debug)]
pub struct HandshakeInfo {
    pub publicKey: String,
    pub finger: String,
    pub version: String,
}

#[derive(Debug)]
pub struct PeerClientInfo {
    pub virtualIp: i32,
    pub name: String,
    pub online: bool,
    pub clientSecret: bool,
}

#[derive(Debug)]
pub struct RegisterInfo {
    pub virtualIp: i32,
    pub virtualNetmask: i32,
    pub virtualGateway: i32,
}
use std::sync::Arc;
use std::os::raw::c_void;
use spki::der::pem::LineEnding;
use spki::EncodePublicKey;
use vnt::handle::callback::ConnectInfo;
use vnt::DeviceInfo;
use vnt::{ErrorInfo, HandshakeInfo, PeerClientInfo, RegisterInfo, VntCallback};

pub struct CallBack {
   connect_Info:ConnectInfo,
   handshake_info:HandshakeInfo,
   error_info:ErrorInfo,
   register_info:RegisterInfo,
   device_config:DeviceConfig,
   peer_client_Info:PeerClientInfo,
   device_Info:DeviceInfo,
}
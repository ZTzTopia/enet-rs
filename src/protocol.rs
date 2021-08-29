use libc::c_int;

use crate::types::{enet_uint8, enet_uint16, enet_uint32};

/**
 * protocol.rs
 *
 * ENet protocol
 */

pub const ENET_PROTOCOL_MINIMUM_MTU: c_int = 576;
pub const ENET_PROTOCOL_MAXIMUM_MTU: c_int = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: c_int = 32;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: c_int = 4096;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: c_int = 65536;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: c_int = 1;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: c_int = 255;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: c_int = 4095;
pub const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: c_int = 1048576;

/*pub enum
{
   ENET_PROTOCOL_MINIMUM_MTU             = 576,
   ENET_PROTOCOL_MAXIMUM_MTU             = 4096,
   ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS = 32,
   ENET_PROTOCOL_MINIMUM_WINDOW_SIZE     = 4096,
   ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE     = 65536,
   ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT   = 1,
   ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT   = 255,
   ENET_PROTOCOL_MAXIMUM_PEER_ID         = 0xFFF,
   ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT  = 1024 * 1024
}*/

#[derive(Debug)]
pub enum ENetProtocolCommand {
    ENET_PROTOCOL_COMMAND_NONE = 0,
    ENET_PROTOCOL_COMMAND_ACKNOWLEDGE = 1,
    ENET_PROTOCOL_COMMAND_CONNECT = 2,
    ENET_PROTOCOL_COMMAND_VERIFY_CONNECT = 3,
    ENET_PROTOCOL_COMMAND_DISCONNECT = 4,
    ENET_PROTOCOL_COMMAND_PING = 5,
    ENET_PROTOCOL_COMMAND_SEND_RELIABLE = 6,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE = 7,
    ENET_PROTOCOL_COMMAND_SEND_FRAGMENT = 8,
    ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED = 9,
    ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT = 10,
    ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE = 11,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT = 12,
    ENET_PROTOCOL_COMMAND_COUNT = 13,

    ENET_PROTOCOL_COMMAND_MASK = 0x0F,
}

#[derive(Debug)]
pub enum ENetProtocolFlag {
    ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE = (1 << 7),
    ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED = (1 << 6),

    ENET_PROTOCOL_HEADER_FLAG_COMPRESSED = (1 << 14),
    ENET_PROTOCOL_HEADER_FLAG_SENT_TIME = (1 << 15),
    ENET_PROTOCOL_HEADER_FLAG_MASK = crate::protocol::ENetProtocolFlag::ENET_PROTOCOL_HEADER_FLAG_COMPRESSED | crate::protocol::ENetProtocolFlag::ENET_PROTOCOL_HEADER_FLAG_SENT_TIME,

    ENET_PROTOCOL_HEADER_SESSION_MASK = (3 << 12),
    ENET_PROTOCOL_HEADER_SESSION_SHIFT = 12,
}

#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolHeader {
    pub peerID: enet_uint16,
    pub sentTime: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolCommandHeader {
    pub command: enet_uint8,
    pub channelID: enet_uint8,
    pub reliableSequenceNumber: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: enet_uint16,
    pub receivedSentTime: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub connectID: enet_uint32,
    pub data: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub connectID: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: enet_uint32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: enet_uint16,
    pub dataLength: enet_uint16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
    pub fragmentCount: enet_uint32,
    pub fragmentNumber: enet_uint32,
    pub totalLength: enet_uint32,
    pub fragmentOffset: enet_uint32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ENetProtocol {
    pub header: ENetProtocolCommandHeader,
    pub acknowledge: ENetProtocolAcknowledge,
    pub connect: ENetProtocolConnect,
    pub verifyConnect: ENetProtocolVerifyConnect,
    pub disconnect: ENetProtocolDisconnect,
    pub ping: ENetProtocolPing,
    pub sendReliable: ENetProtocolSendReliable,
    pub sendUnreliable: ENetProtocolSendUnreliable,
    pub sendUnsequenced: ENetProtocolSendUnsequenced,
    pub sendFragment: ENetProtocolSendFragment,
    pub bandwidthLimit: ENetProtocolBandwidthLimit,
    pub throttleConfigure: ENetProtocolThrottleConfigure,
}


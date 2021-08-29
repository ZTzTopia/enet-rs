/**
 * enet.rs
 *
 * ENet public header file
 */

use crate::{
    types::{enet_uint8, enet_uint16, enet_uint32},
    callbacks::ENetCallbacks,
    header::{ENetBuffer, ENetSocket, ENetSocketSet},
    list::{ENetListNode, ENetList},
    protocol::ENetProtocol
};

use libc::{c_void, c_char, c_int, c_uint, size_t};

pub const ENET_VERSION_MAJOR: u32 = 1;
pub const ENET_VERSION_MINOR: u32 = 3;
pub const ENET_VERSION_PATCH: u32 = 17;

#[macro_export]
macro_rules! ENET_VERSION_CREATE {
    ($major:expr,$minor:expr,$patch:expr) => {
        ((($major)<<16) | (($minor)<<8) | ($patch))
    };
}

#[macro_export]
macro_rules! ENET_VERSION_GET_MAJOR {
    ($version:expr) => {
        ((($version)>>16)&0xFF)
    };
}

#[macro_export]
macro_rules! ENET_VERSION_GET_MINOR {
    ($version:expr) => {
        ((($version)>>8)&0xFF)
    };
}

#[macro_export]
macro_rules! ENET_VERSION_GET_PATCH {
    ($version:expr) => {
        (($version)&0xFF)
    };
}

#[macro_export]
macro_rules! ENET_VERSION {
    () => {
        $crate::ENET_VERSION_CREATE!($crate::enet::ENET_VERSION_MAJOR, $crate::enet::ENET_VERSION_MINOR, $crate::enet::ENET_VERSION_PATCH)
    };
}

pub type ENetVersion = enet_uint32;

#[repr(C)]
#[derive(Debug)]
pub enum ENetSocketType
{
    ENET_SOCKET_TYPE_STREAM = 1,
    ENET_SOCKET_TYPE_DATAGRAM = 2,
}

#[derive(Debug)]
pub enum ENetSocketWait
{
    ENET_SOCKET_WAIT_NONE = 0,
    ENET_SOCKET_WAIT_SEND = (1 << 0),
    ENET_SOCKET_WAIT_RECEIVE = (1 << 1),
    ENET_SOCKET_WAIT_INTERRUPT = (1 << 2),
}

#[repr(C)]
#[derive(Debug)]
pub enum ENetSocketOption
{
    ENET_SOCKOPT_NONBLOCK = 1,
    ENET_SOCKOPT_BROADCAST = 2,
    ENET_SOCKOPT_RCVBUF = 3,
    ENET_SOCKOPT_SNDBUF = 4,
    ENET_SOCKOPT_REUSEADDR = 5,
    ENET_SOCKOPT_RCVTIMEO = 6,
    ENET_SOCKOPT_SNDTIMEO = 7,
    ENET_SOCKOPT_ERROR = 8,
    ENET_SOCKOPT_NODELAY = 9,
}

#[repr(C)]
#[derive(Debug)]
pub enum ENetSocketShutdown
{
    ENET_SOCKET_SHUTDOWN_READ = 0,
    ENET_SOCKET_SHUTDOWN_WRITE = 1,
    ENET_SOCKET_SHUTDOWN_READ_WRITE = 2,
}

pub const ENET_HOST_ANY: c_uint = 0;
pub const ENET_HOST_BROADCAST: c_uint = 0xFFFFFFFF;
pub const ENET_PORT_ANY: c_uint = 0;

/**
 * Portable internet address structure.
 *
 * The host must be specified in network byte-order, and the port must be in host
 * byte-order. The constant ENET_HOST_ANY may be used to specify the default
 * server host. The constant ENET_HOST_BROADCAST may be used to specify the
 * broadcast address (255.255.255.255).  This makes sense for enet_host_connect,
 * but not for enet_host_create.  Once a server responds to a broadcast, the
 * address is updated from ENET_HOST_BROADCAST to the server's actual IP address.
 */
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetAddress
{
    pub host: enet_uint32,
    pub port: enet_uint16,
}

/**
 * Packet flag bit constants.
 *
 * The host must be specified in network byte-order, and the port must be in
 * host byte-order. The constant ENET_HOST_ANY may be used to specify the
 * default server host.
 *
 * [`ENetPacket`]
 *
 * [`ENetPacket`]: ENetPacket
 */
#[derive(Debug)]
pub enum ENetPacketFlag
{
    /**
     * packet must be received by the target peer and resend attempts should be
     * made until the packet is delivered */
    ENET_PACKET_FLAG_RELIABLE = (1 << 0),
    /**
     * packet will not be sequenced with other packets
     * not supported for reliable packets
     */
    ENET_PACKET_FLAG_UNSEQUENCED = (1 << 1),
    /** packet will not allocate data, and user must supply it instead */
    ENET_PACKET_FLAG_NO_ALLOCATE = (1 << 2),
    /**
     * packet will be fragmented using unreliable (instead of reliable) sends
     * if it exceeds the MTU
     */
    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT = (1 << 3),

    /** whether the packet has been sent from all queues it has been entered into */
    ENET_PACKET_FLAG_SENT = (1<<8)
}

pub type ENetPacketFreeCallback = Option<unsafe extern "C" fn(packet: *mut ENetPacket)>;

/**
 * ENet packet structure.
 *
 * An ENet data packet that may be sent to or received from a peer. The shown
 * fields should only be read and never modified. The data field contains the
 * allocated data for the packet. The dataLength fields specifies the length
 * of the allocated data.  The flags field is either 0 (specifying no flags),
 * or a bitwise-or of any combination of the following flags:
 *
 *    ENET_PACKET_FLAG_RELIABLE - packet must be received by the target peer
 *    and resend attempts should be made until the packet is delivered
 *
 *    ENET_PACKET_FLAG_UNSEQUENCED - packet will not be sequenced with other packets
 *    (not supported for reliable packets)
 *
 *    ENET_PACKET_FLAG_NO_ALLOCATE - packet will not allocate data, and user must supply it instead
 *
 *    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT - packet will be fragmented using unreliable
 *    (instead of reliable) sends if it exceeds the MTU
 *
 *    ENET_PACKET_FLAG_SENT - whether the packet has been sent from all queues it has been entered into
 *
 * [`ENetPacketFlag`]
 *
 * [`ENetPacketFlag`]: ENetPacketFlag
 */
#[repr(C)]
#[derive(Debug)]
pub struct ENetPacket
{
    /** internal use only */
    pub referenceCount: size_t,
    /** bitwise-or of ENetPacketFlag constants */
    pub flag: enet_uint32,
    /** allocated data for packet */
    pub data: *mut enet_uint8,
    /** length of data */
    pub dataLength: size_t,
    /** function to be called when the packet is no longer in use */
    pub freeCallback: ENetPacketFreeCallback,
    /** application private data, may be freely modified */
    pub userData: *mut c_void,
}

#[repr(C)]
pub struct ENetAcknowledgement
{
    pub acknowledgementList: ENetListNode,
    pub sentTime: enet_uint32,
    pub command: ENetProtocol,
}

#[repr(C)]
pub struct ENetOutgoingCommand
{
    pub outgoingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub sentTime: enet_uint32,
    pub roundTripTimeout: enet_uint32,
    pub roundTripTimeoutLimit: enet_uint32,
    pub fragmentOffset: enet_uint32,
    pub fragmentLength: enet_uint16,
    pub sendAttempts: enet_uint16,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}

#[repr(C)]
pub struct ENetIncomingCommand
{
    pub incomingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub command: ENetProtocol,
    pub fragmentCount: enet_uint32,
    pub fragmentsRemaining: enet_uint32,
    pub fragments: *mut enet_uint32,
    pub packet: *mut ENetPacket,
}

#[repr(C)]
#[derive(Debug)]
pub enum ENetPeerState
{
    ENET_PEER_STATE_DISCONNECTED = 0,
    ENET_PEER_STATE_CONNECTING = 1,
    ENET_PEER_STATE_ACKNOWLEDGING_CONNECT = 2,
    ENET_PEER_STATE_CONNECTION_PENDING = 3,
    ENET_PEER_STATE_CONNECTION_SUCCEEDED = 4,
    ENET_PEER_STATE_CONNECTED = 5,
    ENET_PEER_STATE_DISCONNECT_LATER = 6,
    ENET_PEER_STATE_DISCONNECTING = 7,
    ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT = 8,
    ENET_PEER_STATE_ZOMBIE = 9,
}

#[macro_export]
macro_rules! ENET_BUFFER_MAXIMUM {
    () => {
       (1 + 2 * $crate::enet::protocol::ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS)
    };
}

pub const ENET_HOST_RECEIVE_BUFFER_SIZE: c_int = 262144;
pub const ENET_HOST_SEND_BUFFER_SIZE: c_int = 262144;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: c_int = 1000;
pub const ENET_HOST_DEFAULT_MTU: c_int = 1400;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: c_int = 33554432;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: c_int = 33554432;
pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: c_int = 500;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: c_int = 32;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: c_int = 32;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: c_int = 7;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: c_int = 2;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: c_int = 2;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: c_int = 5000;
pub const ENET_PEER_PACKET_LOSS_SCALE: c_int = 65536;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: c_int = 10000;
pub const ENET_PEER_WINDOW_SIZE_SCALE: c_int = 65536;
pub const ENET_PEER_TIMEOUT_LIMIT: c_int = 32;
pub const ENET_PEER_TIMEOUT_MINIMUM: c_int = 5000;
pub const ENET_PEER_TIMEOUT_MAXIMUM: c_int = 30000;
pub const ENET_PEER_PING_INTERVAL: c_int = 500;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: c_int = 64;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: c_int = 1024;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: c_int = 32;
pub const ENET_PEER_RELIABLE_WINDOWS: c_int = 16;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: c_int = 4096;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: c_int = 8;

/*pub enum
{
    ENET_HOST_RECEIVE_BUFFER_SIZE = 256 * 1024,
    ENET_HOST_SEND_BUFFER_SIZE = 256 * 1024,
    ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL = 1000,
    ENET_HOST_DEFAULT_MTU = 1400,
    ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE = 32 * 1024 * 1024,
    ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA = 32 * 1024 * 1024,

    ENET_PEER_DEFAULT_ROUND_TRIP_TIME = 500,
    ENET_PEER_DEFAULT_PACKET_THROTTLE = 32,
    ENET_PEER_PACKET_THROTTLE_SCALE = 32,
    ENET_PEER_PACKET_THROTTLE_COUNTER = 7,
    ENET_PEER_PACKET_THROTTLE_ACCELERATION = 2,
    ENET_PEER_PACKET_THROTTLE_DECELERATION = 2,
    ENET_PEER_PACKET_THROTTLE_INTERVAL = 5000,
    ENET_PEER_PACKET_LOSS_SCALE = (1 << 16),
    ENET_PEER_PACKET_LOSS_INTERVAL = 10000,
    ENET_PEER_WINDOW_SIZE_SCALE = 64 * 1024,
    ENET_PEER_TIMEOUT_LIMIT = 32,
    ENET_PEER_TIMEOUT_MINIMUM = 5000,
    ENET_PEER_TIMEOUT_MAXIMUM = 30000,
    ENET_PEER_PING_INTERVAL = 500,
    ENET_PEER_UNSEQUENCED_WINDOWS = 64,
    ENET_PEER_UNSEQUENCED_WINDOW_SIZE = 1024,
    ENET_PEER_FREE_UNSEQUENCED_WINDOWS = 32,
    ENET_PEER_RELIABLE_WINDOWS = 16,
    ENET_PEER_RELIABLE_WINDOW_SIZE = 0x1000,
    ENET_PEER_FREE_RELIABLE_WINDOWS = 8,
}*/

#[repr(C)]
#[derive(Debug)]
pub struct ENetChannel
{
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub outgoingUnreliableSequenceNumber: enet_uint16,
    pub usedReliableWindows: enet_uint16,
    pub reliableWindows: [enet_uint16; 16usize],
    pub incomingReliableSequenceNumber: enet_uint16,
    pub incomingUnreliableSequenceNumber: enet_uint16,
    pub incomingReliableCommands: ENetList,
    pub incomingUnreliableCommands: ENetList,
}

#[derive(Debug)]
pub enum ENetPeerFlag
{
    ENET_PEER_FLAG_NEEDS_DISPATCH = (1 << 0)
}

/**
 * An ENet peer which data packets may be sent or received from.
 *
 * No fields should be modified unless otherwise specified.
 */
#[repr(C)]
#[derive(Debug)]
pub struct ENetPeer
{
    pub dispatchList: ENetListNode,
    pub host: *mut ENetHost,
    pub outgoingPeerID: enet_uint16,
    pub incomingPeerID: enet_uint16,
    pub connectID: enet_uint32,
    pub outgoingSessionID: enet_uint8,
    pub incomingSessionID: enet_uint8,
    /** Internet address of the peer */
    pub address: ENetAddress,
    /** Application private data, may be freely modified */
    pub data: *mut c_void,
    pub state: ENetPeerState,
    pub channels: *mut ENetChannel,
    /** Number of channels allocated for communication with peer */
    pub channelCount: size_t,
    /** Downstream bandwidth of the client in bytes/second */
    pub incomingBandwidth: enet_uint32,
    /** Upstream bandwidth of the client in bytes/second */
    pub outgoingBandwidth: enet_uint32,
    pub incomingBandwidthThrottleEpoch: enet_uint32,
    pub outgoingBandwidthThrottleEpoch: enet_uint32,
    pub incomingDataTotal: enet_uint32,
    pub outgoingDataTotal: enet_uint32,
    pub lastSendTime: enet_uint32,
    pub lastReceiveTime: enet_uint32,
    pub nextTimeout: enet_uint32,
    pub earliestTimeout: enet_uint32,
    pub packetLossEpoch: enet_uint32,
    pub packetsSent: enet_uint32,
    pub packetsLost: enet_uint32,
    /** mean packet loss of reliable packets as a ratio with respect to the constant ENET_PEER_PACKET_LOSS_SCALE */
    pub packetLoss: enet_uint32,
    pub packetLossVariance: enet_uint32,
    pub packetThrottle: enet_uint32,
    pub packetThrottleLimit: enet_uint32,
    pub packetThrottleCounter: enet_uint32,
    pub packetThrottleEpoch: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub pingInterval: enet_uint32,
    pub timeoutLimit: enet_uint32,
    pub timeoutMinimum: enet_uint32,
    pub timeoutMaximum: enet_uint32,
    pub lastRoundTripTime: enet_uint32,
    pub lowestRoundTripTime: enet_uint32,
    pub lastRoundTripTimeVariance: enet_uint32,
    pub highestRoundTripTimeVariance: enet_uint32,
    /** mean round trip time (RTT), in milliseconds, between sending a reliable packet and receiving its acknowledgement */
    pub roundTripTime: enet_uint32,
    pub roundTripTimeVariance: enet_uint32,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub reliableDataInTransit: enet_uint32,
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub acknowledgements: ENetList,
    pub sentReliableCommands: ENetList,
    pub sentUnreliableCommands: ENetList,
    pub outgoingCommands: ENetList,
    pub dispatchedCommands: ENetList,
    pub flags: enet_uint16,
    pub reserved: enet_uint16,
    pub incomingUnsequencedGroup: enet_uint16,
    pub outgoingUnsequencedGroup: enet_uint16,
    pub unsequencedWindow: [enet_uint32; 32usize],
    pub eventData: enet_uint32,
    pub totalWaitingData: size_t,
}

/** An ENet packet compressor for compressing UDP packets before socket sends or receives.
 */
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ENetCompressor
{
    /** Context data for the compressor. Must be non-NULL. */
    pub context: *mut c_void,
    /** Compresses from inBuffers[0:inBufferCount-1], containing inLimit bytes, to outData, outputting at most outLimit bytes. Should return 0 on failure. */
    pub compress: Option<unsafe extern "C" fn(context: *mut c_void, inBuffers: *const ENetBuffer, inBufferCount: size_t, inLimit: size_t, outData: *mut enet_uint8, outLimit: size_t) -> size_t>,
    /** Decompresses from inData, containing inLimit bytes, to outData, outputting at most outLimit bytes. Should return 0 on failure. */
    pub decompress: Option<unsafe extern "C" fn(context: *mut c_void, inData: *const enet_uint8, inLimit: size_t, outData: *mut enet_uint8, outLimit: size_t) -> size_t>,
    /** Destroys the context when compression is disabled or the host is destroyed. May be NULL. */
    pub destroy: Option<unsafe extern "C" fn(context: *mut c_void)>,
}

/** Callback that computes the checksum of the data held in buffers[0:bufferCount-1] */
pub type ENetChecksumCallback = Option<unsafe extern "C" fn(buffers: *const ENetBuffer, bufferCount: size_t) -> enet_uint32>;

/** Callback for intercepting received raw UDP packets. Should return 1 to intercept, 0 to ignore, or -1 to propagate an error. */
pub type ENetInterceptCallback = Option<unsafe extern "C" fn(host: *mut ENetHost, event: *mut ENetHost) -> c_int>;

/**
 * An ENet host for communicating with peers.
 *
 * No fields should be modified unless otherwise stated.
 *
 * [`enet_host_create`]
 *
 * [`enet_host_destroy`]
 *
 * [`enet_host_connect`]
 *
 * [`enet_host_service`]
 *
 * [`enet_host_flush`]
 *
 * [`enet_host_broadcast`]
 *
 * [`enet_host_compress`]
 *
 * [`enet_host_compress_with_range_coder`]
 *
 * [`enet_host_channel_limit`]
 *
 * [`enet_host_bandwidth_limit`]
 *
 * [`enet_host_bandwidth_throttle`]
 *
 * [`enet_host_create`]: enet_host_create
 * [`enet_host_destroy`]: enet_host_destroy
 * [`enet_host_connect`]: enet_host_connect
 * [`enet_host_service`]: enet_host_service
 * [`enet_host_flush`]: enet_host_flush
 * [`enet_host_broadcast`]: enet_host_broadcast
 * [`enet_host_compress`]: enet_host_compress
 * [`enet_host_compress_with_range_coder`]: enet_host_compress_with_range_coder
 * [`enet_host_channel_limit`]: enet_host_channel_limit
 * [`enet_host_bandwidth_limit`]: enet_host_bandwidth_limit
 * [`enet_host_bandwidth_throttle`]: enet_host_bandwidth_throttle
 */
#[repr(C)]
#[derive(Clone)]
pub struct ENetHost
{
    pub socket: ENetSocket,
    /** Internet address of the host */
    pub address: ENetAddress,
    /** downstream bandwidth of the host */
    pub incomingBandwidth: enet_uint32,
    /** upstream bandwidth of the host */
    pub outgoingBandwidth: enet_uint32,
    pub bandwidthThrottleEpoch: enet_uint32,
    pub mtu: enet_uint32,
    pub randomSeed: enet_uint32,
    pub recalculateBandwidthLimits: c_int,
    /** array of peers allocated for this host */
    pub peers: *mut ENetPeer,
    /** number of peers allocated for this host */
    pub peerCount: size_t,
    /** maximum number of channels allowed for connected peers */
    pub channelLimit: size_t,
    pub serviceTime: enet_uint32,
    pub dispatchQueue: ENetList,
    pub continueSending: c_int,
    pub packetSize: size_t,
    pub headerFlags: enet_uint16,
    pub commands: [ENetProtocol; 32usize],
    pub commandCount: size_t,
    pub buffers: [ENetBuffer; 65usize],
    pub bufferCount: size_t,
    /** callback the user can set to enable packet checksums for this host */
    pub checksum: ENetChecksumCallback,
    pub compressor: ENetCompressor,
    pub packetData: [[enet_uint8; 4096usize]; 2usize],
    pub receivedAddress: ENetAddress,
    pub receivedData: *mut enet_uint8,
    pub receivedDataLength: size_t,
    /** total data sent, user should reset to 0 as needed to prevent overflow */
    pub totalSentData: enet_uint32,
    /** total UDP packets sent, user should reset to 0 as needed to prevent overflow */
    pub totalSentPackets: enet_uint32,
    /** total data received, user should reset to 0 as needed to prevent overflow */
    pub totalReceivedData: enet_uint32,
    /** total UDP packets received, user should reset to 0 as needed to prevent overflow */
    pub totalReceivedPackets: enet_uint32,
    /** callback the user can set to intercept received raw UDP packets */
    pub intercept: ENetInterceptCallback,
    pub connectedPeers: size_t,
    pub bandwidthLimitedPeers: size_t,
    /** optional number of allowed peers from duplicate IPs, defaults to ENET_PROTOCOL_MAXIMUM_PEER_ID */
    pub duplicatePeers: size_t,
    /** the maximum allowable packet size that may be sent or received on a peer */
    pub maximumPacketSize: size_t,
    /** the maximum aggregate amount of buffer space a peer may use waiting for packets to be delivered */
    pub maximumWaitingData: size_t,
}

/**
 * An ENet event type, as specified in [`ENetEvent`].
 *
 * [`ENetEvent`]: ENetEvent
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ENetEventType
{
    /** no event occurred within the specified time limit */
    ENET_EVENT_TYPE_NONE       = 0,

    /**
     * a connection request initiated by enet_host_connect has completed.
     * The peer field contains the peer which successfully connected.
     */
    ENET_EVENT_TYPE_CONNECT    = 1,

    /**
     * a peer has disconnected.  This event is generated on a successful
     * completion of a disconnect initiated by enet_peer_disconnect, if
     * a peer has timed out, or if a connection request intialized by
     * enet_host_connect has timed out.  The peer field contains the peer
     * which disconnected. The data field contains user supplied data
     * describing the disconnection, or 0, if none is available.
     */
    ENET_EVENT_TYPE_DISCONNECT = 2,

    /**
     * a packet has been received from a peer.  The peer field specifies the
     * peer which sent the packet.  The channelID field specifies the channel
     * number upon which the packet was received.  The packet field contains
     * the packet that was received; this packet must be destroyed with
     * enet_packet_destroy after use.
     */
    ENET_EVENT_TYPE_RECEIVE    = 3
}

/**
 * An ENet event as returned by enet_host_service().
 *
 * [`enet_host_service`]
 *
 * [`enet_host_service`]: enet_host_service
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ENetEvent
{
    /** type of the event */
    pub type_: ENetEventType,
    /** peer that generated a connect, disconnect or receive event */
    pub peer: *mut ENetPeer,
    /** channel on the peer that generated the event, if appropriate */
    pub channelID: enet_uint8,
    /** data associated with the event, if appropriate */
    pub data: enet_uint32,
    /** packet associated with the event, if appropriate */
    pub packet: *mut ENetPacket,
}

extern "C" {
    /* global ENet global functions */

    /**
     * Initializes ENet globally.  Must be called prior to using any functions in
     * ENet.
     * ### Returns
     * ⠀⠀⠀⠀⠀*0 on success, < 0 on failure*
     */
    pub fn enet_initialize() -> c_int;

    /**
     * Initializes ENet globally and supplies user-overridden callbacks. Must be called prior to using any functions in ENet. Do not use enet_initialize() if you use this variant. Make sure the ENetCallbacks structure is zeroed out so that any additional callbacks added in future versions will be properly ignored.
     * ### Paramenters
     * ⠀⠀⠀⠀⠀*version: the constant ENET_VERSION should be supplied so ENet knows which version of ENetCallbacks struct to use \
     * ⠀⠀⠀⠀ inits: user-overridden callbacks where any NULL callbacks will use ENet's defaults*
     * ### Returns
     * ⠀⠀⠀⠀⠀*0 on success, < 0 on failure*
     */
    pub fn enet_initialize_with_callbacks(version: ENetVersion, inits: *const ENetCallbacks) -> c_int;

    /**
     * Shuts down ENet globally.  Should be called when a program that has
     * initialized ENet exits.
     */
    pub fn enet_deinitialize();

    /**
     * Gives the linked version of the ENet library.
     * ### Returns
     * ⠀⠀⠀⠀⠀*the version number*
     */
    pub fn enet_linked_version() -> ENetVersion;

    /* private ENet private implementation functions */

    /**
     * Returns the wall-time in milliseconds.  Its initial value is unspecified
     * unless otherwise set.
     */
    pub fn enet_time_get() -> enet_uint32;

    /**
     * Sets the current wall-time in milliseconds.
     */
    pub fn enet_time_set(newTimeBase: enet_uint32);

    /* socket ENet socket functions */

    pub fn enet_socket_create(arg: ENetSocketType) -> ENetSocket;
    pub fn enet_socket_bind(socket: ENetSocket, address: *const ENetAddress) -> c_int;
    pub fn enet_socket_get_address(socket: ENetSocket, address: *mut ENetAddress) -> c_int;
    pub fn enet_socket_listen(socket: ENetSocket, arg2: c_int) -> c_int;
    pub fn enet_socket_accept(socket: ENetSocket, address: *mut ENetAddress) -> ENetSocket;
    pub fn enet_socket_send(socket: ENetSocket, address: *const ENetAddress, buffers: *const ENetBuffer, bufferCount: size_t) -> c_int;
    pub fn enet_socket_receive(socket: ENetSocket, address: *const ENetAddress, buffers: *const ENetBuffer, bufferCount: size_t) -> c_int;
    pub fn enet_socket_wait(socket: ENetSocket, condition: *const enet_uint32, timeout: enet_uint32) -> c_int;
    pub fn enet_socket_set_option(socket: ENetSocket, option: ENetSocketOption, value: c_int) -> c_int;
    pub fn enet_socket_get_option(socket: ENetSocket, option: ENetSocketOption, value: *const c_int) -> c_int;
    pub fn enet_socket_shutdown(socket: ENetSocket, how: ENetSocketShutdown) -> c_int;
    pub fn enet_socket_destroy(socket: ENetSocket);
    pub fn enet_socketset_select(maxSocket: ENetSocket, readSet: *const ENetSocketSet, writeSet: *const ENetSocketSet, timeout: enet_uint32) -> c_int;

    /* Address ENet address functions */

    /**
     * Attempts to parse the printable form of the IP address in the parameter hostName
     * and sets the host field in the address parameter if successful.
     * ### Paramenters
     * ⠀⠀⠀⠀⠀*address: destination to store the parsed IP address \
     * ⠀⠀⠀⠀⠀hostName: IP address to parse*
     * ### Return
     * ⠀⠀⠀⠀⠀*0 on success \
     * ⠀⠀⠀⠀⠀< 0 on failure*
     * ### Returns
     * ⠀⠀⠀⠀⠀*the address of the given hostName in address on success*
     */
    pub fn enet_address_set_host_ip(address: *mut ENetAddress, hostName: *const c_char) -> c_int;

    /**
     * Gives the printable form of the IP address specified in the address parameter.
     * * ### Paramenters
     * ⠀⠀⠀⠀⠀*address:        address printed \
     * ⠀⠀⠀⠀⠀hostName:       destination for name, must not be NULL \
     * ⠀⠀⠀⠀⠀nameLength:     maximum length of hostName.*
     * ### Return
     * ⠀⠀⠀⠀⠀*0 on success \
     * ⠀⠀⠀⠀⠀< 0 on failure*
     * ### Returns
     * ⠀⠀⠀⠀⠀*the null-terminated name of the host in hostName on success*
     */
    pub fn enet_address_set_host(address: *mut ENetAddress, hostName: *const c_char) -> c_int;

    /**
     * Gives the printable form of the IP address specified in the address parameter.
     * ### Paramenters
     * ⠀⠀⠀⠀⠀*address: address printed \
     * ⠀⠀⠀⠀⠀hostName: destination for name, must not be NULL \
     * ⠀⠀⠀⠀⠀nameLength: maximum length of hostName.*
     * ### Return
     * ⠀⠀⠀⠀⠀*0 on success \
     * ⠀⠀⠀⠀⠀< 0 on failure*
     * ### Returns
     * ⠀⠀⠀⠀⠀*the null-terminated name of the host in hostName on success*
     */
    pub fn enet_address_get_host_ip(address: *const ENetAddress, hostName: *mut c_char, nameLength: size_t) -> c_int;

    /**
     * Attempts to do a reverse lookup of the host field in the address parameter.
     * ### Paramenters
     * ⠀⠀⠀⠀⠀*address: address used for reverse lookup \
     * ⠀⠀⠀⠀⠀hostName: destination for name, must not be NULL \
     * ⠀⠀⠀⠀⠀nameLength: maximum length of hostName.*
     * ### Return
     * ⠀⠀⠀⠀⠀*0 on success \
     * ⠀⠀⠀⠀⠀< 0 on failure*
     * ### Returns
     * ⠀⠀⠀⠀⠀*the null-terminated name of the host in hostName on success*
     */
    pub fn enet_address_get_host(address: *const ENetAddress, hostName: *mut c_char, nameLength: size_t) -> c_int;

    pub fn enet_packet_create(data: *const c_void, dataLength: size_t, flags: enet_uint32) -> *mut ENetPacket;
    pub fn enet_packet_destroy(packet: *mut ENetPacket);
    pub fn enet_packet_resize(packet: *mut ENetPacket, dataLength: size_t) -> c_int;

    pub fn enet_crc32(buffers: *const ENetBuffer, bufferCount: size_t) -> enet_uint32;

    pub fn enet_host_create(address: *const ENetAddress, peerCount: size_t, channelLimit: size_t, incomingBandwidth: enet_uint32, outgoingBandwidth: enet_uint32) -> *mut ENetHost;
    pub fn enet_host_destroy(host: *mut ENetHost);
    pub fn enet_host_connect(host: *mut ENetHost, address: *const ENetAddress, channelCount: size_t, data: enet_uint32) -> *mut ENetPeer;
    pub fn enet_host_check_events(host: *mut ENetHost, event: *mut ENetEvent) -> c_int;
    pub fn enet_host_service(host: *mut ENetHost, event: *mut ENetEvent, timeout: enet_uint32) -> c_int;
    pub fn enet_host_flush(host: *mut ENetHost);
    pub fn enet_host_broadcast(host: *mut ENetHost, channelID: enet_uint8, packet: *mut ENetPacket);
    pub fn enet_host_compress(host: *mut ENetHost, arg2: *const ENetCompressor);
    pub fn enet_host_compress_with_range_coder(compressor: *mut ENetHost) -> c_int;
    pub fn enet_host_channel_limit(host: *mut ENetHost, channelLimit: size_t);
    pub fn enet_host_bandwidth_limit(host: *mut ENetHost, incomingBandwidth: enet_uint32, outgoingBandwidth: enet_uint32);
    pub fn enet_host_bandwidth_throttle(host: *mut ENetHost);
    pub fn enet_host_random_seed() -> enet_uint32;

    pub fn enet_peer_send(peer: *mut ENetPeer, channelID: enet_uint8, packet: *mut ENetPacket) -> c_int;
    pub fn enet_peer_receive(peer: *mut ENetPeer, channelID: *mut enet_uint8) -> *mut ENetPacket;
    pub fn enet_peer_ping(peer: *mut ENetPeer);
    pub fn enet_peer_ping_interval(peer: *mut ENetPeer, pingInterval: enet_uint32);
    pub fn enet_peer_timeout(peer: *mut ENetPeer, timeoutLimit: enet_uint32, timeoutMinimum: enet_uint32, timeoutMaximum: enet_uint32);
    pub fn enet_peer_reset(peer: *mut ENetPeer);
    pub fn enet_peer_disconnect(peer: *mut ENetPeer, data: enet_uint32);
    pub fn enet_peer_disconnect_now(peer: *mut ENetPeer, data: enet_uint32);
    pub fn enet_peer_disconnect_later(peer: *mut ENetPeer, data: enet_uint32);
    pub fn enet_peer_throttle_configure(peer: *mut ENetPeer, interval: enet_uint32, acceleration: enet_uint32, deceleration: enet_uint32);
    pub fn enet_peer_throttle(peer: *mut ENetPeer, rtt: enet_uint32) -> c_int;
    pub fn enet_peer_reset_queues(peer: *mut ENetPeer);
    pub fn enet_peer_setup_outgoing_command(peer: *mut ENetPeer, outgoingCommand: *mut ENetOutgoingCommand);
    pub fn enet_peer_queue_outgoing_command(peer: *mut ENetPeer, command: *const ENetProtocol, packet: *mut ENetPacket, offset: enet_uint32, length: enet_uint16) -> *mut ENetOutgoingCommand;
    pub fn enet_peer_queue_incoming_command(peer: *mut ENetPeer, command: *const ENetProtocol, data: *const c_void, dataLength: size_t, flags: enet_uint32, fragmentCount: enet_uint32) -> *mut ENetIncomingCommand;
    pub fn enet_peer_queue_acknowledgement(peer: *mut ENetPeer, command: *const ENetProtocol, sentTime: enet_uint16) -> *mut ENetAcknowledgement;
    pub fn enet_peer_dispatch_incoming_unreliable_commands(peer: *mut ENetPeer, channel: *mut ENetChannel, queuedCommand: *mut ENetIncomingCommand);
    pub fn enet_peer_dispatch_incoming_reliable_commands(peer: *mut ENetPeer, channel: *mut ENetChannel, queuedCommand: *mut ENetIncomingCommand);
    pub fn enet_peer_on_connect(peer: *mut ENetPeer);
    pub fn enet_peer_on_disconnect(peer: *mut ENetPeer);

    pub fn enet_range_coder_create() -> *mut c_void;
    pub fn enet_range_coder_destroy(context: *mut c_void);
    pub fn enet_range_coder_compress(context: *mut c_void, inBuffers: *const ENetBuffer, inBufferCount: size_t, inLimit: size_t, outData: *mut enet_uint8, outLimit: size_t) -> size_t;
    pub fn enet_range_coder_decompress(context: *mut c_void, inData: *const enet_uint8, inLimit: size_t, outData: *mut enet_uint8, outLimit: size_t) -> size_t;

    pub fn enet_protocol_command_size(commandNumber: enet_uint8) -> size_t;
}
extern crate byteorder;
extern crate crc;
extern crate num;
extern crate time;

//Crates
use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use self::crc::{crc32, Hasher32};
use self::num::FromPrimitive;

//Std libs
use std::io;
use std::io::{ErrorKind};
use std::io::prelude::*;
use std::net::{Ipv4Addr,Ipv6Addr,TcpStream};
use std::string::FromUtf8Error;
//There will be no padding between the elements and the elements will be sent in the order they appear
//const CEPH_BANNER: str = "ceph v027";
/*
CEPH_BANNER "ceph v027"
CEPH_BANNER_MAX_LEN 30
*/
#[cfg(test)]
mod tests{
    use std::io::Cursor;
    use std::io::prelude::*;
    use std::net::{Ipv4Addr,TcpStream};
    use super::CephPrimitive;
    use crypto;

    //Replay captured data and test results
    #[test]
    fn test_connect(){
        //Connect to monitor port
        /*
        let mut stream = TcpStream::connect("10.0.3.144:6789").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        //recv banner
        (&mut stream).take(9).read_to_end(&mut buf).unwrap();
        println!("Banner received: {}", String::from_utf8(buf).unwrap()); //we're on a roll :D

        //send banner
        println!("Writing banner back to Ceph");
        let mut bytes_written = super::send_banner(&mut stream).unwrap();
        println!("Wrote {} bytes back to Ceph", bytes_written);

        //Send sockaddr_storage
        //let my_addr = super::EntityAddr{

        //};
        //let my_addr = Ipv4Addr::new(192,168,1,6);
        //let mut bytes_written = super::send_addr_info(&mut stream, Some(my_addr), None).unwrap();
        //println!("Wrote {} sock_addr bytes back to Ceph", bytes_written);

        //Get server sockaddr_storage
        //let server_entity_addr = super::recv_addr_info(&mut stream).unwrap();
        //println!("Server entity_addr: {:?}", server_entity_addr);

        let connect = super::CephMsgConnect{
            features: super::CEPH_CLIENT_DEFAULT, //Wireshark is showing not all bits are set
            host_type: super::CephEntity::Client,
            global_seq: 1,
            connect_seq: 0,
            protocol_version: super::Protocol::MonProtocol,
            authorizer_protocol: super::CephAuthProtocol::CephAuthUnknown,
            authorizer_len: 0,
            flags: 1u8<<0, //TODO I think this means lossy
            authorizer: Vec::new(),
        };
        let connect_bytes = connect.write_to_wire().unwrap();
        println!("Writing CephMsgConnect to Ceph {:?}", &connect_bytes);
        bytes_written = stream.write(&connect_bytes).unwrap();
        println!("Wrote {} CephMsgConnect bytes", bytes_written);

        //Is this another sockaddr_storage response from the server??
        let mut bogus_buffer = Vec::new();
        (&mut stream).take(136).read_to_end(&mut bogus_buffer).unwrap();
        println!("Bogus data? bytes: {:?}", &bogus_buffer);

        //Get the connection reply
        let mut msg_reply_buffer = Vec::new();
        (&mut stream).take(26).read_to_end(&mut msg_reply_buffer).unwrap();
        println!("Ceph Msg Reply bytes: {:?}", &msg_reply_buffer);

        //Decode it
        let mut ceph_msg_reply_cursor = Cursor::new(&mut msg_reply_buffer[..]);
        let ceph_msg_reply = super::CephMsgConnectReply::read_from_wire(&mut ceph_msg_reply_cursor);
        println!("CephMsgConnectReply: {:?}", ceph_msg_reply);

        //Create a KeepAlive2
        let keep_alive = super::CephMsgKeepAlive2::new();
        let keep_alive_bytes = keep_alive.write_to_wire().unwrap();

        //Send it
        println!("Writing KeepAlive2 to Ceph {:?}", &keep_alive_bytes);
        bytes_written = stream.write(&keep_alive_bytes).unwrap();
        println!("Wrote {:?} KeepAlive2 bytes", bytes_written);

        //I think I need to setup the authorizer stuff now and negotiate a cephx connection
        let auth_client_ticket = crypto::AuthTicket::new(600.0);
        let auth_ticket_bytes = auth_client_ticket.write_to_wire().unwrap();

        bytes_written = stream.write(&auth_ticket_bytes).unwrap();
        println!("Wrote {:?} auth ticket bytes", bytes_written);
        */
    }

    #[test]
    fn test_connect_reply(){

    }

}

#[derive(Debug)]
pub enum SerialError {
	IoError(io::Error),
    ByteOrder(byteorder::Error),
	InvalidValue,
	InvalidType,
    FromUtf8Error(FromUtf8Error),
}

impl SerialError{
    pub fn new(err: String) -> SerialError {
        SerialError::IoError(
            io::Error::new(ErrorKind::Other, err)
        )
    }
}

impl From<FromUtf8Error> for SerialError {
    fn from(err: FromUtf8Error) -> SerialError {
        SerialError::FromUtf8Error(err)
    }
}

impl From<byteorder::Error> for SerialError {
    fn from(err: byteorder::Error) -> SerialError {
        SerialError::ByteOrder(err)
    }
}

impl From<io::Error> for SerialError {
    fn from(err: io::Error) -> SerialError {
        SerialError::IoError(err)
    }
}

pub trait CephPrimitive {
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>;
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>;
}

#[derive(Debug)]
pub struct CephMsgConnect{
    pub features: CephFeatures, //Composed of CephFeature bitflags
    pub host_type: CephEntity, //u32
    pub global_seq: u32,
    pub connect_seq: u32,
    pub protocol_version: Protocol,
    pub authorizer_protocol: CephAuthProtocol,
    pub authorizer_len: u32,
    pub flags: u8,
    pub authorizer: Vec<u8>,
}

impl CephPrimitive for CephMsgConnect{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let feature_bits = try!(cursor.read_u64::<LittleEndian>());
        let features = match CephFeatures::from_bits(feature_bits){
            Some(features) => features,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to features", feature_bits)));
            }
        };
        let host_type_bits = try!(cursor.read_u32::<LittleEndian>());
        let host_type = match CephEntity::from_u32(host_type_bits){
            Some(host_type) => host_type,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to host_type", host_type_bits)));
            }
        };
        let global_seq = try!(cursor.read_u32::<LittleEndian>());
        let connect_seq = try!(cursor.read_u32::<LittleEndian>());
        let protocol_bits = try!(cursor.read_u32::<LittleEndian>());
        let protocol_version = match Protocol::from_u32(protocol_bits){
            Some(procol_version) => procol_version,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to protocol_version", protocol_bits)));
            }
        };
        let authorizer_bits = try!(cursor.read_u32::<LittleEndian>());
        let authorizer_protocol = match CephAuthProtocol::from_u32(authorizer_bits){
            Some(authorizer_protocol) => authorizer_protocol,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to authorizer_protocol", authorizer_bits)));
            }
        };
        let authorizer_len = try!(cursor.read_u32::<LittleEndian>());
        let flags = try!(cursor.read_u8());

        return Ok(CephMsgConnect{
            features: features,
            host_type: host_type,
            global_seq: global_seq,
            connect_seq: connect_seq,
            protocol_version: protocol_version,
            authorizer_protocol: authorizer_protocol,
            authorizer_len: authorizer_len,
            flags: flags,
            authorizer: Vec::new()
        })
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        try!(buffer.write_u64::<LittleEndian>(self.features.bits));
        try!(buffer.write_u32::<LittleEndian>(self.host_type.clone() as u32));
        try!(buffer.write_u32::<LittleEndian>(self.global_seq));
        try!(buffer.write_u32::<LittleEndian>(self.connect_seq));
        try!(buffer.write_u32::<LittleEndian>(self.protocol_version.clone() as u32));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_protocol.clone() as u32));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_len));
        try!(buffer.write_u8(self.flags));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephMsgConnectReply{
    pub tag: CephMsg,
    pub features: CephFeatures,
    pub global_seq: u32,
    pub connect_seq: u32,
    pub protocol_version: Protocol,
    pub authorizer_len: u32,
    pub flags: u8,
    pub authorizer: Vec<u8>,
}

impl CephPrimitive for CephMsgConnectReply{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag_bits = try!(cursor.read_u8());
        let tag = match CephMsg::from_u8(tag_bits){
            Some(tag) => tag,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to tag", tag_bits)));
            }
        };

        let feature_bits = try!(cursor.read_u64::<LittleEndian>());
        let features = match CephFeatures::from_bits(feature_bits){
            Some(features) => features,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to features", feature_bits)));
            }
        };
        let global_seq = try!(cursor.read_u32::<LittleEndian>());
        let connect_seq = try!(cursor.read_u32::<LittleEndian>());
        let protocol_bits = try!(cursor.read_u32::<LittleEndian>());
        let protocol_version = match Protocol::from_u32(protocol_bits){
            Some(procol_version) => procol_version,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to protocol_version", protocol_bits)));
            }
        };
        let authorizer_len = try!(cursor.read_u32::<LittleEndian>());
        let flags = try!(cursor.read_u8());
        let authorizer = Vec::new();

        return Ok(CephMsgConnectReply{
            tag: tag,
            features: features,
            global_seq: global_seq,
            connect_seq: connect_seq,
            protocol_version: protocol_version,
            authorizer_len: authorizer_len,
            flags: flags,
            authorizer: authorizer
        });

    }

    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        try!(buffer.write_u8(self.tag.clone() as u8));
        try!(buffer.write_u64::<LittleEndian>(self.features.bits));
        try!(buffer.write_u32::<LittleEndian>(self.global_seq));
        try!(buffer.write_u32::<LittleEndian>(self.connect_seq));
        try!(buffer.write_u32::<LittleEndian>(self.protocol_version.clone() as u32));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_len));
        try!(buffer.write_u8(self.flags));
        for b in &self.authorizer{
            try!(buffer.write_u8(b.clone()));
        }
        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephMsgrMsg {
    pub tag: CephMsg,//    u8 tag = 0x07;
    pub header: CephMsgHeader,
    pub msg: Message,
    pub footer: CephMsgFooter,
}

impl CephMsgrMsg{
    fn new(header: CephMsgHeader, msg: Message, footer: CephMsgFooter)->CephMsgrMsg{
        return CephMsgrMsg{
            tag: CephMsg::Msg,
            header: header,
            msg: msg,
            footer: footer,
        }
    }
}

impl CephPrimitive for CephMsgrMsg{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag_bits = try!(cursor.read_u8());
        let tag = match CephMsg::from_u8(tag_bits){
            Some(tag) => tag,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to tag", tag_bits)));
            }
        };
        let header = try!(CephMsgHeader::read_from_wire(cursor));
        //println!("header: {:?}", &header);
        //CephMsg is sandwhiched between these two fields
        let msg = try!(read_message_from_wire(cursor, &header.msg_type));
        //println!("msg: {:?}", &msg);
        //Skip the footer for now
        //let footer = try!(CephMsgFooter::read_from_wire(cursor));
        //println!("footer: {:?}", &footer);

        return Ok(CephMsgrMsg{
            tag: tag,
            header: header,
            msg: msg,
            footer: CephMsgFooter{
                front_crc: 0,
                middle_crc: 0,
                data_crc: 0,
                crypto_sig: 0,
                flags: 0,
            },
        });
    }

    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        try!(buffer.write_u8(self.tag.clone() as u8));

        let header_bits = try!(self.header.write_to_wire());
        for b in header_bits{
            try!(buffer.write_u8(b.clone()));
        }

        //Encode Message

        let footer_bits = try!(self.footer.write_to_wire());

        for b in footer_bits{
            try!(buffer.write_u8(b.clone()));
        }

        return Ok(buffer);
    }
}

enum_from_primitive!{
//#[repr(u32)]
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum CephEntity{
    Mon=1,
    Mds=2,
    Osd=4,
    Client=8,
    Auth=20, //Used to setup a new CephX connection
    Any=255
}
}

#[derive(Debug, Clone)]
enum Crypto {
    None = 0,
    Aes = 1,
}

enum_from_primitive!{
#[repr(u32)]
#[derive(Debug, Clone)]
pub enum Protocol{
    OsdProtocol = 24, /*server/client*/
    MdsProtocol = 32, /*server/client*/
    MonProtocol = 15, /*server/client*/
}
}

bitflags!{
    flags CephFeatures: u64 {
        const CEPH_FEATURE_UID  = 1u64 <<0,
        const CEPH_FEATURE_NOSRCADDR = 1u64 <<1,
        const CEPH_FEATURE_MONCLOCKCHECK = 1u64 <<2,
        const CEPH_FEATURE_FLOCK = 1u64 << 3,
        const CEPH_FEATURE_SUBSCRIBE2 = 1u64 <<4,
        const CEPH_FEATURE_MONNAME = 1u64 <<5,
        const CEPH_FEATURE_RECONNECT_SEQ = 1u64 <<6,
        const CEPH_FEATURE_DIRLAYOUTHASH = 1u64 << 7,
        const CEPH_FEATURE_OBJECTLOCATOR = 1u64 << 8,
        const CEPH_FEATURE_PGID64 = 1u64 << 9,
        const CEPH_FEATURE_INCSUBOSDMAP = 1u64 << 10,
        const CEPH_FEATURE_PGPOOL3 = 1u64 << 11,
        const CEPH_FEATURE_OSDREPLYMUX = 1u64 << 12,
        const CEPH_FEATURE_OSDENC = 1u64 << 13,
        const CEPH_FEATURE_OMAP = 1u64 << 14,
        const CEPH_FEATURE_QUERY_T = 1u64 << 15,
        const CEPH_FEATURE_MONENC = 1u64 << 16,
        const CEPH_FEATURE_INDEP_PG_MAP = 1u64 << 17,
        const CEPH_FEATURE_CRUSH_TUNABLES = 1u64 << 18,
        const CEPH_FEATURE_CHUNKY_SCRUB = 1u64 << 19,
        const CEPH_FEATURE_MON_NULLROUTE = 1u64 << 20,
        const CEPH_FEATURE_MON_GV = 1u64 << 21,
        const CEPH_FEATURE_BACKFILL_RESERVATION = 1u64 << 22,
        const CEPH_FEATURE_MSG_AUTH = 1u64 << 23,
        const CEPH_FEATURE_RECOVERY_RESERVATION = 1u64 << 24,
        const CEPH_FEATURE_CRUSH_TUNABLES1 = 1u64 << 25,
        const CEPH_FEATURE_CREATEPOOLID = 1u64 << 26,
        const CEPH_FEATURE_REPLY_CREATE_INODE = 1u64 << 27,
        const CEPH_FEATURE_OSD_HBMSGS = 1u64 << 28,
        const CEPH_FEATURE_MDSENC = 1u64 << 29,
        const CEPH_FEATURE_OSDHASHPSPOOL = 1u64 << 30,
        const CEPH_FEATURE_MON_SINGLE_PAXOS = 1u64 << 31,
        const CEPH_FEATURE_OSD_SNAPMAPPER = 1u64 << 32,
        const CEPH_FEATURE_MON_SCRUB = 1u64 << 33,
        const CEPH_FEATURE_OSD_PACKED_RECOVERY = 1u64 << 34,
        const CEPH_FEATURE_OSD_CACHEPOOL = 1u64 << 35,
        const CEPH_FEATURE_CRUSH_V2 = 1u64 << 36,
        const CEPH_FEATURE_EXPORT_PEER = 1u64 << 37,
        const CEPH_FEATURE_OSD_ERASURE_CODES = 1u64 << 38,
        const CEPH_FEATURE_OSDMAP_ENC = 1u64 << 39,
        const CEPH_FEATURE_MDS_INLINE_DATA = 1u64 << 40,
        const CEPH_FEATURE_CRUSH_TUNABLES3 = 1u64 << 41,
        const CEPH_FEATURE_OSD_PRIMARY_AFFINITY = 1u64 << 41, //overlap with tunables3
        const CEPH_FEATURE_MSGR_KEEPALIVE2 = 1u64 << 42,
        const CEPH_FEATURE_OSD_POOLRESEND = 1u64 << 43,
        const CEPH_FEATURE_ERASURE_CODE_PLUGINS_V2 = 1u64 << 44,
        const CEPH_FEATURE_OSD_SET_ALLOC_HINT = 1u64 << 45,
        const CEPH_FEATURE_OSD_FADVISE_FLAGS = 1u64 << 46,
        const CEPH_FEATURE_OSD_REPOP = 1u64 << 46, //overlap with fadvice
        const CEPH_FEATURE_OSD_OBJECT_DIGEST = 1u64 << 46, //overlap with fadvice
        const CEPH_FEATURE_OSD_TRANSACTION_MAY_LAYOUT = 1u64 << 46, //overlap with fadvice
        const CEPH_FEATURE_MDS_QUOTA = 1u64 << 47,
        const CEPH_FEATURE_CRUSH_V4 = 1u64 << 48,
        const CEPH_FEATURE_OSD_MIN_SIZE_RECOVERY = 1u64 << 49, //overlap
    	const CEPH_FEATURE_OSD_PROXY_FEATURES = 1u64 << 49,
        const CEPH_FEATURE_MON_METADATA = 1u64 << 50,
        const CEPH_FEATURE_OSD_BITWISE_HOBJ_SORT = 1u64 << 51,
        const CEPH_FEATURE_ERASURE_CODE_PLUGINS_V3 = 1u64 << 52,
        const CEPH_FEATURE_OSD_PROXY_WRITE_FEATURES = 1u64 << 53,
        const CEPH_FEATURE_OSD_HITSET_GMT = 1u64 << 54,
    	const CEPH_FEATURE_RESERVED2 = 1u64 << 61,
    	const CEPH_FEATURE_RESERVED = 1u64 << 62,
    	const CEPH_FEATURE_RESERVED_BROKEN = 1u64 << 63,
        const CEPH_CLIENT_DEFAULT =  CEPH_FEATURE_UID.bits
            | CEPH_FEATURE_NOSRCADDR.bits
            | CEPH_FEATURE_MONCLOCKCHECK.bits
            | CEPH_FEATURE_FLOCK.bits
            | CEPH_FEATURE_SUBSCRIBE2.bits
            | CEPH_FEATURE_MONNAME.bits
            | CEPH_FEATURE_RECONNECT_SEQ.bits
            | CEPH_FEATURE_DIRLAYOUTHASH.bits

            | CEPH_FEATURE_OBJECTLOCATOR.bits
            | CEPH_FEATURE_PGID64.bits
            | CEPH_FEATURE_INCSUBOSDMAP.bits
            | CEPH_FEATURE_PGPOOL3.bits
            | CEPH_FEATURE_OSDREPLYMUX.bits
            | CEPH_FEATURE_OSDENC.bits
            | CEPH_FEATURE_OMAP.bits
            | CEPH_FEATURE_QUERY_T.bits

            | CEPH_FEATURE_MONENC.bits
            | CEPH_FEATURE_INDEP_PG_MAP.bits
            | CEPH_FEATURE_CRUSH_TUNABLES.bits
            | CEPH_FEATURE_CHUNKY_SCRUB.bits
            | CEPH_FEATURE_MON_NULLROUTE.bits
            | CEPH_FEATURE_MON_GV.bits
            | CEPH_FEATURE_BACKFILL_RESERVATION.bits
            | CEPH_FEATURE_MSG_AUTH.bits

            | CEPH_FEATURE_RECOVERY_RESERVATION.bits
            | CEPH_FEATURE_CRUSH_TUNABLES1.bits
            | CEPH_FEATURE_CREATEPOOLID.bits
            | CEPH_FEATURE_REPLY_CREATE_INODE.bits
            | CEPH_FEATURE_OSD_HBMSGS.bits
            | CEPH_FEATURE_MDSENC.bits
            | CEPH_FEATURE_OSDHASHPSPOOL.bits
            | CEPH_FEATURE_MON_SINGLE_PAXOS.bits

            | CEPH_FEATURE_OSD_SNAPMAPPER.bits
            | CEPH_FEATURE_MON_SCRUB.bits
            | CEPH_FEATURE_OSD_PACKED_RECOVERY.bits
            | CEPH_FEATURE_OSD_CACHEPOOL.bits
            | CEPH_FEATURE_CRUSH_V2.bits
            | CEPH_FEATURE_EXPORT_PEER.bits
            | CEPH_FEATURE_OSD_ERASURE_CODES.bits
            | CEPH_FEATURE_OSDMAP_ENC.bits,

        const CEPH_ALL = CEPH_FEATURE_UID.bits
            | CEPH_FEATURE_NOSRCADDR.bits
            | CEPH_FEATURE_MONCLOCKCHECK.bits
            | CEPH_FEATURE_FLOCK.bits
            | CEPH_FEATURE_SUBSCRIBE2.bits
            | CEPH_FEATURE_MONNAME.bits
            | CEPH_FEATURE_RECONNECT_SEQ.bits
            | CEPH_FEATURE_DIRLAYOUTHASH.bits
            | CEPH_FEATURE_OBJECTLOCATOR.bits
            | CEPH_FEATURE_PGID64.bits
            | CEPH_FEATURE_INCSUBOSDMAP.bits
            | CEPH_FEATURE_PGPOOL3.bits
            | CEPH_FEATURE_OSDREPLYMUX.bits
            | CEPH_FEATURE_OSDENC.bits
            | CEPH_FEATURE_OMAP.bits
            | CEPH_FEATURE_QUERY_T.bits
            | CEPH_FEATURE_MONENC.bits
            | CEPH_FEATURE_INDEP_PG_MAP.bits
            | CEPH_FEATURE_CRUSH_TUNABLES.bits
            | CEPH_FEATURE_CHUNKY_SCRUB.bits
            | CEPH_FEATURE_MON_NULLROUTE.bits
            | CEPH_FEATURE_MON_GV.bits
            | CEPH_FEATURE_BACKFILL_RESERVATION.bits
            | CEPH_FEATURE_MSG_AUTH.bits
            | CEPH_FEATURE_RECOVERY_RESERVATION.bits
            | CEPH_FEATURE_CRUSH_TUNABLES1.bits
            | CEPH_FEATURE_CREATEPOOLID.bits
            | CEPH_FEATURE_REPLY_CREATE_INODE.bits
            | CEPH_FEATURE_OSD_HBMSGS.bits
            | CEPH_FEATURE_MDSENC.bits
            | CEPH_FEATURE_OSDHASHPSPOOL.bits
            | CEPH_FEATURE_MON_SINGLE_PAXOS.bits
            | CEPH_FEATURE_OSD_SNAPMAPPER.bits
            | CEPH_FEATURE_MON_SCRUB.bits
            | CEPH_FEATURE_OSD_PACKED_RECOVERY.bits
            | CEPH_FEATURE_OSD_CACHEPOOL.bits
            | CEPH_FEATURE_CRUSH_V2.bits
            | CEPH_FEATURE_EXPORT_PEER.bits
            | CEPH_FEATURE_OSD_ERASURE_CODES.bits
            | CEPH_FEATURE_OSDMAP_ENC.bits
            | CEPH_FEATURE_MDS_INLINE_DATA.bits
            | CEPH_FEATURE_CRUSH_TUNABLES3.bits
            | CEPH_FEATURE_OSD_PRIMARY_AFFINITY.bits
            | CEPH_FEATURE_MSGR_KEEPALIVE2.bits
            | CEPH_FEATURE_OSD_POOLRESEND.bits
            | CEPH_FEATURE_ERASURE_CODE_PLUGINS_V2.bits
            | CEPH_FEATURE_OSD_SET_ALLOC_HINT.bits
            | CEPH_FEATURE_OSD_FADVISE_FLAGS.bits
            | CEPH_FEATURE_OSD_REPOP.bits
            | CEPH_FEATURE_OSD_OBJECT_DIGEST.bits
            | CEPH_FEATURE_OSD_TRANSACTION_MAY_LAYOUT.bits
            | CEPH_FEATURE_MDS_QUOTA.bits
            | CEPH_FEATURE_CRUSH_V4.bits
            | CEPH_FEATURE_OSD_MIN_SIZE_RECOVERY.bits
            | CEPH_FEATURE_OSD_PROXY_FEATURES.bits
            | CEPH_FEATURE_MON_METADATA.bits
            | CEPH_FEATURE_OSD_BITWISE_HOBJ_SORT.bits
            | CEPH_FEATURE_ERASURE_CODE_PLUGINS_V3.bits
            | CEPH_FEATURE_OSD_PROXY_WRITE_FEATURES.bits
            | CEPH_FEATURE_OSD_HITSET_GMT.bits,
    }
}

enum_from_primitive!{
#[repr(u32)]
#[derive(Debug, Clone)]
pub enum CephAuthProtocol{
    CephAuthUnknown = 0,
    CephAuthNone = 1,
    CephAuthCephx = 2,
}
}


enum_from_primitive!{
#[derive(Debug, Clone)]
pub enum CephPriority{
    Low = 64,
    Default = 127,
    High = 196,
    Highest = 255,
}
}

enum_from_primitive! {
#[derive(Debug, Clone)]
pub enum CephMsg{
    Ready = 1, /* server->client: ready for messages */
    Reset = 2, /* server->client: reset, try again */
    Wait = 3,  /* server->client: wait for racing incoming connection */
    RetrySession = 4, /* server->client + cseq: try again
	            			with higher cseq */
    RetryGlobal = 5,  /* server->client + gseq: try again
					       with higher gseq */
    Close = 6, /* closing pipe */
    Msg = 7,  /* message */
    Ack = 8,  /* message ack */
    KeepAlive = 9, /* just a keepalive byte! */
    BadProtocolVersion = 10, /* bad protocol version */
    BadAuthorizer = 11, /* bad authorizer */
    InsufficientFeatures = 12, /* insufficient features */
    Seq = 13, /* 64-bit int follows with seen seq number */
    KeepAlive2 = 14,
    KeepAlive2Ack = 15, /* keepalive reply */
    Unknown = 20,
}
}

#[derive(Debug)]
pub enum Message{
    Paxos(PaxosMessage),
    Command,
    CommandReply,
    WatchNotify,
    MsgForward,
    MsgRoute,
    MonCommand(MonCommand),
    MonCommandAck,
    Log,
    LogAck,
    Class,
    ClassAck,
    Getpoolstats,
    Getpoolstatsreply,
    GlobalId,
    MonScrub,
    MonElection,
    MonPaxos,
    MonProbe,
    MonJoin,
    MonSync,
    OsdAlive,
    OsdBoot,
    OsdFailure,
    OsdMarkMeDown,
    OsdMap,
    OsdOp(CephOsdOperation),
    OsdOpRepl(CephOsdOperationReply),
    OsdPing,
    OsdSubop(CephOsdOperation),
    OsdSubopReply(CephOsdOperationReply),
    OsdPgtemp,
    OsdPgNotify,
    OsdPgQuery,
    OsdPgSummary,
    OsdPgLog,
    OsdPgRemove,
    OsdPgInfo,
    OsdPgTrim,
    OsdScrub,
    OsdPgMissing,
    OsdRepScrub,
    OsdPgScan,
    OsdPgBackfill,
    Pgstats,
    Pgstatsack,
    OsdPgCreate,
    RemoveSnaps,
    OsdBackfillReserve,
    OsdRecoveryReserve,
    OsdPgPush,
    OsdPgPull,
    OsdPgPushReply,
    OsdEcWrite,
    OsdEcWriteReply,
    OsdEcRead,
    OsdEcReadReply,
    OsdRepop,
    OsdRepopreply,
    Timecheck,
    MonHealth,
    CrcData,
    CrcHeader,
    DataPing,
    Nop,
}

//Decode the msg from the wire and return the correct variant
fn read_message_from_wire<R: Read>(cursor: &mut R, msg_type: &CephMsgType) -> Result<Message, SerialError>{
    match msg_type{
        &CephMsgType::MsgOsdOp => {
            println!("osdop");
            let osdop = try!(CephOsdOperation::read_from_wire(cursor));
            println!("osdop parsed");
            return Ok(Message::OsdOp(osdop));
        },
        &CephMsgType::MsgOsdOpReply => {
            println!("opreply");
            let op_reply = try!(CephOsdOperationReply::read_from_wire(cursor));
            return Ok(Message::OsdOpRepl(op_reply));
        },
        &CephMsgType::MsgOsdSubop => {
            println!("subop");
            let osdop = try!(CephOsdOperation::read_from_wire(cursor));
            println!("subop: {:?}", &osdop);
            return Ok(Message::OsdSubop(osdop));
        },
        &CephMsgType::MsgOsdSubopReply => {
            println!("subopreply");
            let osdop = try!(CephOsdOperationReply::read_from_wire(cursor));
            return Ok(Message::OsdSubopReply(osdop));
        },
        &CephMsgType::MsgMonCommand =>{
            let mon_command = try!(MonCommand::read_from_wire(cursor));
            return Ok(Message::MonCommand(mon_command));
        }
        _ => {
            return Ok(Message::Nop)
        },
    }
}

fn write_message_to_wire(msg: Message) -> Result<Vec<u8>, SerialError>{
    match msg{
        Message::MonCommand(ref mon_cmd) =>{
            let bytes = try!(mon_cmd.write_to_wire());
            return Ok(bytes);
        },
        Message::OsdOp(ref osd_op) => {
            let bytes = try!(osd_op.write_to_wire());
            return Ok(bytes);
        },
        Message::OsdOpRepl(ref osd_op) => {
            let bytes = try!(osd_op.write_to_wire());
            return Ok(bytes);
        },
        _ => {
            return Ok(Vec::new());
        },
    }
}

enum_from_primitive! {
#[derive(Debug, Clone)]
pub enum CephMsgType{
    MsgPaxos = 40,
    MsgOsdMap = 41,
    MsgOsdOp = 42,
    MsgOsdOpReply = 43,
    MsgWatchNotify = 44,
    MsgForward = 46,
    MsgRoute = 47,

    MsgMonCommand = 50,
    MsgMonCommandAck = 51,
    MsgLog = 52,
    MsgLogAck = 53,
    //MsgMonObserve = 54,
    //MsgMonObserveNotify = 55,
    MsgClass = 56,
    MsgClassAck = 57,
    MsgGetpoolstats  = 58,
    MsgGetpoolstatsreply = 59,
    MsgMonGlobalId = 60,

    // monitor internal
    MsgMonScrub = 64,
    MsgMonElection = 65,
    MsgMonPaxos = 66,
    MsgMonProbe= 67,
    MsgMonJoin = 68,
    MsgMonSync = 69,


    MsgOsdPing = 70,
    MsgOsdBoot = 71,
    MsgOsdFailure = 72,
    MsgOsdAlive = 73,
    MsgOsdMarkMeDown = 74,
    MsgOsdSubop = 76,
    MsgOsdSubopReply = 77,
    MsgOsdPgtemp = 78,
    MsgOsdPgNotify = 80,
    MsgOsdPgQuery = 81,
    MsgOsdPgSummary = 82,
    MsgOsdPgLog = 83,
    MsgOsdPgRemove = 84,
    MsgOsdPgInfo = 85,
    MsgOsdPgTrim = 86,
    MsgPgstats = 87,
    MsgPgstatsack = 88,
    MsgOsdPgCreate = 89,
    MsgRemoveSnaps = 90,
    MsgOsdScrub = 91,
    MsgOsdPgMissing = 92,
    MsgOsdRepScrub = 93,
    MsgOsdPgScan = 94,
    MsgOsdPgBackfill = 95,
    MsgCommand = 97,
    MsgCommandReply = 98,
    MsgOsdBackfillReserve=99,
    MsgOsdRecoveryReserve=150,
    MsgOsdPgPush = 105,
    MsgOsdPgPull = 106,
    MsgOsdPgPushReply= 107,
    MsgOsdEcWrite =  108,
    MsgOsdEcWriteReply=109,
    MsgOsdEcRead = 110,
    MsgOsdEcReadReply =111,
    MsgOsdRepop = 112,
    MsgOsdRepopreply = 113,
    // *** generic ***
    MsgTimecheck = 0x600,
    MsgMonHealth = 0x601,
    // *** Message::encode() crcflags bits ***
    MsgCrcData = (1 << 0),
    MsgCrcHeader = (1 << 1),
    //MsgCrcAll = (MsgCrcData | MsgCrcHeader),
    // Xio Testing
    MsgDataPing = 0x602,
    MsgNop = 0x607,
}
}

bitflags!{
    flags OsdOp: u32 {
        const CEPH_OSD_FLAG_ACK =            0x0001,  /* want (or is) "ack" ack */
        const CEPH_OSD_FLAG_ONNVRAM =        0x0002,  /* want (or is) "onnvram" ack */
        const CEPH_OSD_FLAG_ACK_ONDISK =     0x0004,  /* want (or is) "ondisk" ack */
        const CEPH_OSD_FLAG_RETRY =          0x0008,  /* resend attempt */
        const CEPH_OSD_FLAG_READ =           0x0010,  /* op may read */
        const CEPH_OSD_FLAG_WRITE =          0x0020,  /* op may write */
        const CEPH_OSD_FLAG_ORDERSNAP =      0x0040,  /* EOLDSNAP if snapc is out of order */
        const CEPH_OSD_FLAG_PEERSTAT_OLD =   0x0080,  /* DEPRECATED msg includes osd_peer_stat */
        const CEPH_OSD_FLAG_BALANCE_READS =  0x0100,
        const CEPH_OSD_FLAG_PARALLELEXEC =   0x0200,  /* execute op in parallel */
        const CEPH_OSD_FLAG_PGOP =           0x0400,  /* pg op, no object */
        const CEPH_OSD_FLAG_EXEC =           0x0800,  /* op may exec */
        const CEPH_OSD_FLAG_EXEC_PUBLIC =    0x1000,  /* DEPRECATED op may exec (public) */
        const CEPH_OSD_FLAG_LOCALIZE_READS = 0x2000,  /* read from nearby replica, if any */
        const CEPH_OSD_FLAG_RWORDERED =      0x4000,  /* order wrt concurrent reads */
        const CEPH_OSD_FLAG_IGNORE_CACHE =   0x8000,  /* ignore cache logic */
        const CEPH_OSD_FLAG_SKIPRWLOCKS =   0x10000,  /* skip rw locks */
        const CEPH_OSD_FLAG_IGNORE_OVERLAY =0x20000,  /* ignore pool overlay */
        const CEPH_OSD_FLAG_FLUSH =         0x40000,  /* this is part of flush */
        const CEPH_OSD_FLAG_MAP_SNAP_CLONE =0x80000,  /* map snap direct to clone id */
        const CEPH_OSD_FLAG_ENFORCE_SNAPC  =0x100000,  /* use snapc provided even if */
        const CEPH_OSD_FLAG_REDIRECTED   = 0x200000,  /* op has been redirected */
        const CEPH_OSD_FLAG_KNOWN_REDIR = 0x400000,  /* redirect bit is authoritative */
    }
}

#[derive(Debug)]
pub struct ObjectLocator{
    pub encoding_version: u8,
    pub min_compat_version: u8,
    pub size: u32,
    pub pool: u64,
    pub namespace_size: u32,
    pub namespace_data: Vec<u8>,
}

impl CephPrimitive for ObjectLocator {
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let encoding_version = try!(cursor.read_u8());
        let min_compat_version = try!(cursor.read_u8());
        let size = try!(cursor.read_u32::<LittleEndian>());
        let pool = try!(cursor.read_u64::<LittleEndian>());
        //TODO: Wireshark skips 8 bytes here.  What is this?
        let _ = try!(cursor.read_u64::<LittleEndian>());
        let namespace_size = try!(cursor.read_u32::<LittleEndian>());
        let mut namespace_buf: Vec<u8> = Vec::new();
        for _ in 0 .. namespace_size{
            let b = try!(cursor.read_u8());
            namespace_buf.push(b);
        }
        //TODO: Wireshark skips 8 bytes here.  What is this?
        let _ = try!(cursor.read_u64::<LittleEndian>());

        return Ok(
            ObjectLocator{
                encoding_version: encoding_version,
                min_compat_version: min_compat_version,
                size: size,
                pool: pool,
                namespace_size: namespace_size,
                namespace_data: namespace_buf,
            }
        );
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u8(self.encoding_version));
        try!(buffer.write_u8(self.min_compat_version));
        try!(buffer.write_u32::<LittleEndian>(self.size));
        try!(buffer.write_u64::<LittleEndian>(self.pool));
        try!(buffer.write_u32::<LittleEndian>(self.namespace_size));

        for b in &self.namespace_data{
            try!(buffer.write_u8(*b));
        }
        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct PlacementGroup{
    pub group_version: u8,
    pub pool: u64,
    pub seed: u32,
    pub preferred: u32,
}

impl CephPrimitive for PlacementGroup {
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let group_version = try!(cursor.read_u8());
        let pool = try!(cursor.read_u64::<LittleEndian>());
        let seed = try!(cursor.read_u32::<LittleEndian>());
        let preferred = try!(cursor.read_u32::<LittleEndian>());

        return Ok(PlacementGroup{
            group_version:group_version,
            pool: pool,
            seed:seed,
            preferred:preferred,
        });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u8(self.group_version));
        try!(buffer.write_u64::<LittleEndian>(self.pool));
        try!(buffer.write_u32::<LittleEndian>(self.seed));
        try!(buffer.write_u32::<LittleEndian>(self.preferred));

        return Ok(buffer);
    }

}

#[derive(Debug)]
pub struct ObjectId{
    pub size: u32,
    pub data: Vec<u8>
}

impl CephPrimitive for ObjectId {
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let size = try!(cursor.read_u32::<LittleEndian>());
        let mut data_buf: Vec<u8> = Vec::new();
        for _ in 0 .. size{
            data_buf.push(try!(cursor.read_u8()));
        }
        return Ok(ObjectId{
            size: size,
            data: data_buf,
        });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u32::<LittleEndian>(self.size));

        for b in &self.data{
            try!(buffer.write_u8(*b));
        }

        return Ok(buffer);
    }

}

#[derive(Debug)]
pub struct Operation{
    pub operation: u16,
    pub flags: u32,
    pub offset: u64,
    pub size: u64,
    pub truncate_size: u64,
    pub truncate_seq: u32,
    pub payload_size: u32,
}

impl CephPrimitive for Operation {
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let operation = try!(cursor.read_u16::<LittleEndian>());
        let flags = try!(cursor.read_u32::<LittleEndian>());
        let offset = try!(cursor.read_u64::<LittleEndian>());
        let size = try!(cursor.read_u64::<LittleEndian>());
        let truncate_size = try!(cursor.read_u64::<LittleEndian>());
        let truncate_seq = try!(cursor.read_u32::<LittleEndian>());
        let payload_size = try!(cursor.read_u32::<LittleEndian>());

        return Ok(Operation{
            operation: operation,
            flags:flags,
            offset: offset,
            size: size,
            truncate_size: truncate_size,
            truncate_seq: truncate_seq,
            payload_size: payload_size,
        });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u16::<LittleEndian>(self.operation));
        try!(buffer.write_u32::<LittleEndian>(self.flags));
        try!(buffer.write_u64::<LittleEndian>(self.offset));
        try!(buffer.write_u64::<LittleEndian>(self.size));
        try!(buffer.write_u64::<LittleEndian>(self.truncate_size));
        try!(buffer.write_u32::<LittleEndian>(self.truncate_seq));
        try!(buffer.write_u32::<LittleEndian>(self.payload_size));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct ReplayVersion {
    version: u64,
    epoch: u32,
}

impl CephPrimitive for ReplayVersion {
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let version = try!(cursor.read_u64::<LittleEndian>());
        let epoch = try!(cursor.read_u32::<LittleEndian>());

        return Ok(ReplayVersion{
            version: version,
            epoch: epoch,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();

        try!(buffer.write_u64::<LittleEndian>(self.version));
        try!(buffer.write_u32::<LittleEndian>(self.epoch));
        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephOsdOperationReply{
    pub object_id: ObjectId,
    pub placement_group: PlacementGroup,
    pub flags: OsdOp,
    pub result: u32,
    pub bad_replay_version: ReplayVersion,
    pub osd_map_epoch: u32,
    pub operation_count: u32,
    pub operation: Operation,
    pub retry_attempt: u32,
    pub operation_return_value: u32,
    pub replay_version: ReplayVersion,
    pub user_version: u64,
}

impl CephPrimitive for CephOsdOperationReply{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let object_id = try!(ObjectId::read_from_wire(cursor));
        let pg = try!(PlacementGroup::read_from_wire(cursor));
        let flag_bits = try!(cursor.read_u32::<LittleEndian>());
        let flags = match OsdOp::from_bits(flag_bits){
            Some(flags) => flags,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to flags", flag_bits)));
            }
        };
        let result = try!(cursor.read_u32::<LittleEndian>());
        let bad_replay_version = try!(ReplayVersion::read_from_wire(cursor));
        let osd_map_epoch = try!(cursor.read_u32::<LittleEndian>());
        let operation_count = try!(cursor.read_u32::<LittleEndian>());
        let operation = try!(Operation::read_from_wire(cursor));
        let retry_attempt = try!(cursor.read_u32::<LittleEndian>());
        let operation_return_value = try!(cursor.read_u32::<LittleEndian>());
        let replay_version = try!(ReplayVersion::read_from_wire(cursor));
        let user_version = try!(cursor.read_u64::<LittleEndian>());

        return Ok(
            CephOsdOperationReply{
                object_id: object_id,
                placement_group: pg,
                flags: flags,
                result: result,
                bad_replay_version: bad_replay_version,
                osd_map_epoch: osd_map_epoch,
                operation_count: operation_count,
                operation: operation,
                retry_attempt: retry_attempt,
                operation_return_value: operation_return_value,
                replay_version: replay_version,
                user_version: user_version,
            });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();

        buffer.extend(try!(self.object_id.write_to_wire()));
        buffer.extend(try!(self.placement_group.write_to_wire()));
        try!(buffer.write_u32::<LittleEndian>(self.flags.bits));
        buffer.extend(try!(self.bad_replay_version.write_to_wire()));
        try!(buffer.write_u32::<LittleEndian>(self.osd_map_epoch));
        try!(buffer.write_u32::<LittleEndian>(self.operation_count));
        buffer.extend(try!(self.operation.write_to_wire()));
        try!(buffer.write_u32::<LittleEndian>(self.retry_attempt));
        try!(buffer.write_u32::<LittleEndian>(self.operation_return_value));
        buffer.extend(try!(self.replay_version.write_to_wire()));
        try!(buffer.write_u64::<LittleEndian>(self.user_version));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephOsdOperation{
    pub client: u32,
    pub map_epoch: u32,
    pub flags: OsdOp,
    pub modification_time: Utime,
    pub reassert_version: u64,
    pub reassert_epoch: u32,
    pub locator: ObjectLocator,
    pub placement_group: PlacementGroup,
    pub object_id: ObjectId,
    pub operation_count: u16,
    pub operation: Operation, //TODO: Change to Vec<Operation>,
    pub snapshot_id: u64,
    pub snapshot_seq: u64,
    pub snapshot_count: u32,
    pub retry_attempt: u32,
    pub payload: Vec<u8>,
}

impl CephPrimitive for CephOsdOperation{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let client = try!(cursor.read_u32::<LittleEndian>());
        let map_epoch = try!(cursor.read_u32::<LittleEndian>());
        let flag_bits = try!(cursor.read_u32::<LittleEndian>());
        let flags = match OsdOp::from_bits(flag_bits){
            Some(flags) => flags,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to flags", flag_bits)));
            }
        };
        let utime = try!(Utime::read_from_wire(cursor));
        let reassert_version = try!(cursor.read_u64::<LittleEndian>());
        let reassert_epoch = try!(cursor.read_u32::<LittleEndian>());
        let locator = try!(ObjectLocator::read_from_wire(cursor));
        let pg = try!(PlacementGroup::read_from_wire(cursor));
        let object_id = try!(ObjectId::read_from_wire(cursor));
        let operation_count = try!(cursor.read_u16::<LittleEndian>());
        let operation = try!(Operation::read_from_wire(cursor));

        let snapshot_id = try!(cursor.read_u64::<LittleEndian>());
        let snapshot_seq = try!(cursor.read_u64::<LittleEndian>());
        let snapshot_count = try!(cursor.read_u32::<LittleEndian>());
        let retry_attempt = try!(cursor.read_u32::<LittleEndian>());
        //TODO: maybe we should skip copying this into a buffer.  I don't really care
        //what the data contained is.  I mostly care about the READ/WRITE sizes.
        let payload_buffer: Vec<u8> = Vec::new();

        //Skipping the copy of the data because I only really care about the read/write sizes
        //We could add this in again if it's really critical to know exactly what is being written
        /*for _ in 0..operation.payload_size{
            let b = try!(cursor.read_u8());
            payload_buffer.push(b);
        }*/

        return Ok(
            CephOsdOperation{
                client: client,
                map_epoch: map_epoch,
                flags: flags,
                modification_time: utime,
                reassert_version: reassert_version,
                reassert_epoch: reassert_epoch,
                locator: locator,
                placement_group: pg,
                object_id: object_id,
                operation_count: operation_count,
                operation: operation,
                snapshot_id: snapshot_id,
                snapshot_seq: snapshot_seq,
                snapshot_count: snapshot_count,
                retry_attempt: retry_attempt,
                payload: payload_buffer,
            });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();

        try!(buffer.write_u32::<LittleEndian>(self.client));
        try!(buffer.write_u32::<LittleEndian>(self.map_epoch));
        try!(buffer.write_u32::<LittleEndian>(self.flags.bits));
        buffer.extend(try!(self.modification_time.write_to_wire()));
        try!(buffer.write_u64::<LittleEndian>(self.reassert_version));
        try!(buffer.write_u32::<LittleEndian>(self.reassert_epoch));
        buffer.extend(try!(self.locator.write_to_wire()));
        buffer.extend(try!(self.placement_group.write_to_wire()));
        buffer.extend(try!(self.object_id.write_to_wire()));
        try!(buffer.write_u16::<LittleEndian>(self.operation_count));
        buffer.extend(try!(self.operation.write_to_wire()));
        try!(buffer.write_u64::<LittleEndian>(self.snapshot_id));
        try!(buffer.write_u64::<LittleEndian>(self.snapshot_seq));
        try!(buffer.write_u32::<LittleEndian>(self.snapshot_count));
        try!(buffer.write_u32::<LittleEndian>(self.retry_attempt));

        for b in &self.payload{
            buffer.push(b.clone());
        }

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct PaxosMessage {
    pub version: u64,
    pub mon: u16,
    pub mon_tid: u64,
}

impl CephPrimitive for PaxosMessage{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let version = try!(cursor.read_u64::<LittleEndian>());
        let mon = try!(cursor.read_u16::<LittleEndian>());
        let mon_tid = try!(cursor.read_u64::<LittleEndian>());
        return Ok(PaxosMessage{
            version: version,
            mon: mon,
            mon_tid: mon_tid,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        try!(buffer.write_u64::<LittleEndian>(self.version));
        try!(buffer.write_u16::<LittleEndian>(self.mon));
        try!(buffer.write_u64::<LittleEndian>(self.mon_tid));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct MonCommand {
    pub paxos: PaxosMessage,
    pub fsid: String,
    pub argument_count: u32,
    pub arguments: Vec<String> //Size: u32, utf8 data
}

impl CephPrimitive for MonCommand{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let paxos = try!(PaxosMessage::read_from_wire(cursor));
        let fsid_len = try!(cursor.read_u32::<LittleEndian>());
        let mut fsid_buff = Vec::new();
        for _ in 0..fsid_len{
            fsid_buff.push(try!(cursor.read_u8()));
        }
        let fsid = try!(String::from_utf8(fsid_buff));
        let arg_count = try!(cursor.read_u32::<LittleEndian>());
        let mut args: Vec<String> = Vec::with_capacity(arg_count as usize);

        for _ in 0..arg_count{
            let mut buf = Vec::new();
            let size = try!(cursor.read_u32::<LittleEndian>());
            for _ in 0..size{
                buf.push(try!(cursor.read_u8()));
            }
            let arg = try!(String::from_utf8(buf));
            args.push(arg);
        }

        return Ok(MonCommand{
            paxos: paxos,
            fsid: fsid,
            argument_count: arg_count,
            arguments: args,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(try!(self.paxos.write_to_wire()));

        let fsid_copy = self.fsid.clone();
        for b in fsid_copy.into_bytes(){
            buffer.push(b.clone());
        }

        try!(buffer.write_u32::<LittleEndian>(self.argument_count));

        for arg in &self.arguments{
            let arg_copy = arg.clone();
            for b in arg_copy.into_bytes(){
                buffer.push(b.clone());
            }
        }

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephEntityName{
    pub entity_type: CephEntity,
    pub num: u64,
}

#[derive(Debug)]
pub struct Utime {
    pub tv_sec: u32,  // Seconds since epoch.
    pub tv_nsec: u32, // Nanoseconds since the last second.
}

impl Utime{
    pub fn new()->Self{
        let now: time::Timespec = time::now().to_timespec();
        return Utime {
            tv_sec: now.sec as u32,
            tv_nsec: now.nsec as u32,
        };
    }
}

impl CephPrimitive for Utime{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tv_sec = try!(cursor.read_u32::<LittleEndian>());
        let tv_nsec = try!(cursor.read_u32::<LittleEndian>());
        return Ok(Utime {
            tv_sec: tv_sec,
            tv_nsec: tv_nsec,
        });
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u32::<LittleEndian>(self.tv_sec));
        try!(buffer.write_u32::<LittleEndian>(self.tv_nsec));

        return Ok(buffer);
    }
}

// From src/include/msgr.h
#[derive(Debug)]
pub struct CephMsgHeader {
    pub sequence_num: u64,
    pub transaction_id: u64,
    pub msg_type: CephMsgType, //u16,  //CEPH_MSG_* or MSG_*
    pub priority: u16,
    pub version: u16,   //version of message encoding
    pub front_len: u32, // The size of the front section
    pub middle_len: u32,// The size of the middle section
    pub data_len: u32,  // The size of the data section
    pub data_off: u16,  // The way data should be aligned by the reciever
    pub entity_name: CephEntityName, // Information about the sender
    pub compat_version: u16, // Oldest compatible encoding version
    reserved: u16, // Unused
    crc: u32,  // CRC of header
}

impl CephPrimitive for CephMsgHeader{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let sequenece_num = try!(cursor.read_u64::<LittleEndian>());
        let transcation_id = try!(cursor.read_u64::<LittleEndian>());
        let msg_type_bits = try!(cursor.read_u16::<LittleEndian>());
        //println!("msg_type bits: {:?}", &msg_type_bits);
        let msg_type = match CephMsgType::from_u16(msg_type_bits){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to msg_type", msg_type_bits)));
            }
        };
        let priority_bits = try!(cursor.read_u16::<LittleEndian>());
        /*
        println!("priority_bits: {:?}", &priority_bits);
        let priority = match CephPriority::from_u16(priority_bits){
            Some(t) => t,
            None => {
                println!("Oops 2");
                return Err(SerialError::new(format!("Unable to convert {:?} to priority", priority_bits)));
            }
        };
        */
        let version = try!(cursor.read_u16::<LittleEndian>());
        let front_len = try!(cursor.read_u32::<LittleEndian>());
        let middle_len = try!(cursor.read_u32::<LittleEndian>());
        let data_len = try!(cursor.read_u32::<LittleEndian>());
        let data_off = try!(cursor.read_u16::<LittleEndian>());

        let entity_type_bits = try!(cursor.read_u8());
        let entity_type = match CephEntity::from_u8(entity_type_bits){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to entity_type", entity_type_bits)));
            }
        };
        let entity_id = try!(cursor.read_u64::<LittleEndian>());

        let compat_version = try!(cursor.read_u16::<LittleEndian>());
        let reserved = try!(cursor.read_u16::<LittleEndian>());
        let crc = try!(cursor.read_u32::<LittleEndian>());

        return Ok(
            CephMsgHeader{
            sequence_num: sequenece_num,
            transaction_id: transcation_id,
            msg_type: msg_type,
            priority: priority_bits,
            version: version,
            front_len: front_len,
            middle_len: middle_len,
            data_len: data_len,
            data_off: data_off,
            entity_name: CephEntityName{
                entity_type: entity_type,
                num: entity_id,
            },
            compat_version: compat_version,
            reserved: reserved,
            crc: crc,
            }
        );
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut digest = crc32::Digest::new(crc32::IEEE);

        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u64::<LittleEndian>(self.sequence_num));
        try!(buffer.write_u64::<LittleEndian>(self.transaction_id));
        try!(buffer.write_u16::<LittleEndian>(self.msg_type.clone() as u16));
        try!(buffer.write_u16::<LittleEndian>(self.priority));
        try!(buffer.write_u16::<LittleEndian>(self.version));
        try!(buffer.write_u32::<LittleEndian>(self.front_len));
        try!(buffer.write_u32::<LittleEndian>(self.middle_len));
        try!(buffer.write_u32::<LittleEndian>(self.data_len));
        try!(buffer.write_u16::<LittleEndian>(self.data_off));

        try!(buffer.write_u8(self.entity_name.entity_type.clone() as u8));
        try!(buffer.write_u64::<LittleEndian>(self.entity_name.num));

        try!(buffer.write_u16::<LittleEndian>(self.compat_version));
        try!(buffer.write_u16::<LittleEndian>(self.reserved));
        try!(buffer.write_u32::<LittleEndian>(self.crc));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct CephMsgFooter {
    pub front_crc: u32,
    pub middle_crc: u32,
    pub data_crc: u32,
    pub crypto_sig: u64,
    pub flags: u8
}

impl CephPrimitive for CephMsgFooter{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let front_crc = try!(cursor.read_u32::<LittleEndian>());
        let middle_crc = try!(cursor.read_u32::<LittleEndian>());
        let data_crc = try!(cursor.read_u32::<LittleEndian>());

        let crypto_sig = try!(cursor.read_u64::<LittleEndian>());
        let flags = try!(cursor.read_u8());

        return Ok(
            CephMsgFooter{
                front_crc: front_crc,
                middle_crc: middle_crc,
                data_crc: data_crc,
                crypto_sig: crypto_sig,
                flags: flags
            }
        );
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u32::<LittleEndian>(self.front_crc));
        try!(buffer.write_u32::<LittleEndian>(self.middle_crc));
        try!(buffer.write_u32::<LittleEndian>(self.data_crc));
        try!(buffer.write_u64::<LittleEndian>(self.crypto_sig));
        try!(buffer.write_u8(self.flags));

        return Ok(buffer);
    }
}

struct CephMsgTagAck{
    tag: CephMsg, //0x08
    seq: u64 //Sequence number of msg being acknowledged
}
impl CephMsgTagAck{
    fn new(sequence_number: u64) -> CephMsgTagAck{
        return CephMsgTagAck{
            tag: CephMsg::Ack,
            seq: sequence_number,
        };
    }
}

impl CephPrimitive for CephMsgTagAck{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag_bits = try!(cursor.read_u8());
        let msg = match CephMsg::from_u8(tag_bits){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to CephMsg", tag_bits)));
            }
        };
        let seq = try!(cursor.read_u64::<LittleEndian>());

        return Ok(CephMsgTagAck{
            tag: msg,
            seq: seq,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u8(self.tag.clone() as u8));
        try!(buffer.write_u64::<LittleEndian>(self.seq));
        return Ok(buffer);
    }
}

struct CephMsgKeepAlive{
    tag: CephMsg, //0x09
    data: u8, // No data
}

impl CephMsgKeepAlive{
    fn new() -> CephMsgKeepAlive{
        return CephMsgKeepAlive{
            tag: CephMsg::KeepAlive,
            data: 0,
        }
    }
}

impl CephPrimitive for CephMsgKeepAlive{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag = try!(cursor.read_u8());
        let msg = match CephMsg::from_u8(tag){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to CephMsg", tag)));
            }
        };
        let data = try!(cursor.read_u8());

        return Ok(CephMsgKeepAlive{
            tag: msg,
            data: data,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u8(self.tag.clone() as u8));
        try!(buffer.write_u8(self.data));
        return Ok(buffer);
    }
}

struct CephMsgKeepAlive2{
    tag: CephMsg, //0x0E
    timestamp: Utime,
}

impl CephMsgKeepAlive2{
    fn new() -> CephMsgKeepAlive2{
        let now: time::Timespec = time::now().to_timespec();
        let timestamp = Utime{
            tv_sec: now.sec as u32,
            tv_nsec: now.nsec as u32,
        };
        return CephMsgKeepAlive2{
            tag: CephMsg::KeepAlive2,
            timestamp:timestamp,
        }
    }
}

impl CephPrimitive for CephMsgKeepAlive2{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag_bits = try!(cursor.read_u8());
        let msg = match CephMsg::from_u8(tag_bits){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to CephMsg", tag_bits)));
            }
        };
        let time = try!(Utime::read_from_wire(cursor));

        return Ok(CephMsgKeepAlive2{
            tag: msg,
            timestamp: time,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u8(self.tag.clone() as u8));
        try!(buffer.write_u32::<LittleEndian>(self.timestamp.tv_sec));
        try!(buffer.write_u32::<LittleEndian>(self.timestamp.tv_nsec));

        return Ok(buffer);
    }
}

struct CephMsgKeepAlive2Ack{
    tag: CephMsg, //0x0F
    timestamp: Utime,
}

impl CephMsgKeepAlive2Ack {
    fn new() -> CephMsgKeepAlive2Ack{
        let now: time::Timespec = time::now().to_timespec();
        let timestamp = Utime{
            tv_sec: now.sec as u32,
            tv_nsec: now.nsec as u32,
        };

        return CephMsgKeepAlive2Ack{
            tag: CephMsg::KeepAlive2Ack,
            timestamp: timestamp,
        };
    }
}

impl CephPrimitive for CephMsgKeepAlive2Ack{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag_bits = try!(cursor.read_u8());
        let msg = match CephMsg::from_u8(tag_bits){
            Some(t) => t,
            None => {
                return Err(SerialError::new(format!("Unable to convert {:?} to CephMsg", tag_bits)));
            }
        };

        let time = try!(Utime::read_from_wire(cursor));

        return Ok(CephMsgKeepAlive2Ack{
            tag: msg,
            timestamp: time,
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer: Vec<u8> = Vec::new();

        try!(buffer.write_u8(self.tag.clone() as u8));
        try!(buffer.write_u32::<LittleEndian>(self.timestamp.tv_sec));
        try!(buffer.write_u32::<LittleEndian>(self.timestamp.tv_nsec));

        return Ok(buffer);
    }
}

#[derive(Debug)]
pub struct EntityAddr{
    pub port: u16,
    pub nonce: u32,
    pub v4addr: Option<Ipv4Addr>,
    pub v6addr: Option<Ipv6Addr>,
}

impl CephPrimitive for EntityAddr{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        //type
        let _ = try!(cursor.read_u32::<LittleEndian>());
        let nonce = try!(cursor.read_u32::<LittleEndian>());
        //type-str
        let address_family = try!(cursor.read_u16::<BigEndian>());
        match address_family{
            0x0002 => {
                let port = try!(cursor.read_u16::<BigEndian>());
                let a = try!(cursor.read_u8());
                let b = try!(cursor.read_u8());
                let c = try!(cursor.read_u8());
                let d = try!(cursor.read_u8());
                let ip = Ipv4Addr::new(a,b,c,d);
                return Ok(
                    EntityAddr{
                        port: port,
                        nonce: nonce,
                        v4addr: Some(ip),
                        v6addr:None,
                    }
                );
            },
            0x000A =>{
                //TODO: Test this
                println!("IPv6 Addr");
                let port = try!(cursor.read_u16::<BigEndian>());
                println!("Port {}", port);
                let a = try!(cursor.read_u16::<BigEndian>());
                let b = try!(cursor.read_u16::<BigEndian>());
                let c = try!(cursor.read_u16::<BigEndian>());
                let d = try!(cursor.read_u16::<BigEndian>());
                let e = try!(cursor.read_u16::<BigEndian>());
                let f = try!(cursor.read_u16::<BigEndian>());
                let g = try!(cursor.read_u16::<BigEndian>());
                let h = try!(cursor.read_u16::<BigEndian>());
                let ip = Ipv6Addr::new(a,b,c,d,e,f,g,h);
                println!("IPv6 Addr_string: {}", ip);
                return Ok(
                    EntityAddr{
                        port: port,
                        nonce: nonce,
                        v4addr: None,
                        v6addr: Some(ip),
                    }
                );
            },
            _ => {
                return Err(
                    SerialError::new(format!("unknown ip address family: {}", address_family))
                );
            }
        }
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        //socket_type
        let mut buffer:Vec<u8> = Vec::new();

        try!(buffer.write_u32::<LittleEndian>(0)); //Is this right?
        try!(buffer.write_u32::<LittleEndian>(self.nonce));

        if self.v4addr.is_some(){
            //Address Family
            try!(buffer.write_u16::<BigEndian>(0x0002));
            //Port
            try!(buffer.write_u16::<BigEndian>(self.port));
            let tmp = self.v4addr.unwrap();//TODO eliminate this
            for octet in tmp.octets().iter(){
                try!(buffer.write_u8(*octet));
            }
            //Sockaddr_storage seems to be a 128 byte structure and
            //the ceph client is sending 120 bytes of 0's or padding
            for _ in 0..120{
                try!(buffer.write_u8(0));
            }
        }else if self.v6addr.is_some(){
            //Address Family
            try!(buffer.write_u32::<LittleEndian>(0x000A));

            //Port
            try!(buffer.write_u16::<BigEndian>(self.port));

            let tmp = self.v6addr.unwrap();//TODO eliminate this
            for octet in tmp.segments().iter(){
                try!(buffer.write_u16::<BigEndian>(*octet));
            }
        }else{
            //Unknown
            return Err(
                SerialError::new("EntityAddr needs a v4addr or v6addr.  Missing both".to_string())
            );
        }
        return Ok(buffer);
    }
}

//Connect to Ceph Monitor and send a hello banner
fn send_banner(socket: &mut TcpStream)->Result<usize, SerialError>{
    let banner = String::from("ceph v027");
    let written_bytes = try!(socket.write(banner.as_bytes()));
    if written_bytes <= 0{
        return Err(SerialError::new("Unable to send banner".to_string()));
    }else{
        return Ok(written_bytes);
    }
}

fn send_msg(socket: &mut TcpStream, msg: Message)->Result<usize, SerialError>{
    let bytes_to_send = try!(write_message_to_wire(msg));
    let written_bytes = try!(socket.write(&bytes_to_send[..]));
    if written_bytes <= 0{
        return Err(SerialError::new("Unable to send_msg".to_string()));
    }else{
        return Ok(written_bytes);
    }
}

//TODO: What should this do?
fn recv_msg(socket: &mut TcpStream){

}

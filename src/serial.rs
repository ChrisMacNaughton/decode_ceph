extern crate byteorder;

extern crate num;
use self::num::FromPrimitive;

use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::{Cursor, ErrorKind};
use std::io::prelude::*;
use std::net::{Ipv4Addr,Ipv6Addr,TcpStream};
//There will be no padding between the elements and the elements will be sent in the order they appear
//const CEPH_BANNER: str = "ceph v027";
/*
#define CEPH_BANNER "ceph v027"
#define CEPH_BANNER_MAX_LEN 30

typedef u32le epoch_t;
typedef u32le ceph_seq_t;
typedef u64le ceph_tid_t;
typedef u64le version_t;
*/
#[cfg(test)]
mod tests{
    use std::io::Cursor;
    use std::io::prelude::*;
    use std::net::{Ipv4Addr,TcpStream};
    use super::CephPrimitive;
    //Replay captured data and test results
    #[test]
    fn test_connect(){
        let banner = String::from("ceph v027");
        //Connect to monitor port
        let mut stream = TcpStream::connect("10.0.3.216:6789").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        //recv banner
        (&mut stream).take(9).read_to_end(&mut buf).unwrap();
        println!("Banner received: {}", String::from_utf8(buf).unwrap()); //we're on a roll :D
        //send banner
        println!("Writing banner back to Ceph");
        let mut bytes_written = stream.write(&banner.into_bytes()).unwrap();
        println!("Wrote {} bytes back to Ceph", bytes_written);
        //Send sockaddr_storage
        let client_info = super::EntityAddr{
            port: 0,
            nonce: 0,
            v4addr: Some(Ipv4Addr::new(192,168,1,6)),
            v6addr: None,
        };
        //send sock_addr_storage
        let client_sock_addr_bytes = super::encode_entity_addr(client_info).unwrap();
        println!("Writing client info back to Ceph: {:?}", &client_sock_addr_bytes);
        let mut bytes_written = stream.write(&client_sock_addr_bytes).unwrap();

        //Get server sockaddr_storage
        buf = Vec::new();
        (&mut stream).take(136).read_to_end(&mut buf).unwrap();
        let mut sockaddr_cursor = Cursor::new(&mut buf[..]);
        println!("Decoding live Ceph server sockaddr_storage");
        super::decode_entity_addr(&mut sockaddr_cursor);
        //recv this:
        let mut ceph_response_bytes = vec![
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x1a,0x85,0x0a,0x00,0x03,0xd8,0x00, //17
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //34
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //51
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //68
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //85
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, //102
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x88,0x50,0x0a,0x00,0x03,0x90,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
        ];
        let mut cursor = Cursor::new(&mut ceph_response_bytes[..]);
        super::decode_entity_addr(&mut cursor);
        let connect_msg = super::CephMsgConnect::read_from_wire(&mut cursor);
        println!("Connect msg: {:?}", connect_msg);
        println!("Cursor position: {}", cursor.position());
        let msg_header = super::CephMsgHeader::read_from_wire(&mut cursor);
        println!("Msg header: {:?}", msg_header);
        println!("Cursor position: {}", cursor.position());
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
}

impl SerialError{
    fn new(err: String) -> SerialError {
        SerialError::IoError(
            io::Error::new(ErrorKind::Other, err)
        )
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

trait CephPrimitive {
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>;
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>;
}

#[derive(Debug)]
struct CephMsgConnect{
    features: u64,
    host_type: u32,
    global_seq: u32,
    connect_seq: u32,
    protocol_version: u32,
    authorizer_protocol: u32,
    authorizer_len: u32,
    flags: u8,
    authorizer: Vec<u8>,
}

impl CephPrimitive for CephMsgConnect{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let features = try!(cursor.read_u64::<LittleEndian>());
        println!("Features: {:x}", features);
        let host_type = try!(cursor.read_u32::<LittleEndian>());
        let global_seq = try!(cursor.read_u32::<LittleEndian>());
        let connect_seq = try!(cursor.read_u32::<LittleEndian>());
        let protocol_version = try!(cursor.read_u32::<LittleEndian>());
        let authorizer_protocol = try!(cursor.read_u32::<LittleEndian>());
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
        try!(buffer.write_u64::<LittleEndian>(self.features));
        try!(buffer.write_u32::<LittleEndian>(self.host_type));
        try!(buffer.write_u32::<LittleEndian>(self.global_seq));
        try!(buffer.write_u32::<LittleEndian>(self.connect_seq));
        try!(buffer.write_u32::<LittleEndian>(self.protocol_version));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_protocol));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_len));
        try!(buffer.write_u8(self.flags));

        return Ok(buffer);
    }
}

#[derive(Debug)]
struct CephMsgReply{
    tag: CephMsg,
    features: u64,
    global_seq: u32,
    connect_seq: u32,
    protocol_version: u32,
    authorizer_len: u32,
    flags: u8,
    authorizer: Vec<u8>,
}

impl CephPrimitive for CephMsgReply{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag = try!(cursor.read_u8());
        let features = try!(cursor.read_u64::<LittleEndian>());
        let global_seq = try!(cursor.read_u32::<LittleEndian>());
        let connect_seq = try!(cursor.read_u32::<LittleEndian>());
        let protocol_version = try!(cursor.read_u32::<LittleEndian>());
        let authorizer_len = try!(cursor.read_u32::<LittleEndian>());
        let flags = try!(cursor.read_u8());
        let authorizer = Vec::new();

        return Ok(CephMsgReply{
            tag: CephMsg::from_u8(tag).unwrap(),
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
        try!(buffer.write_u64::<LittleEndian>(self.features));
        try!(buffer.write_u32::<LittleEndian>(self.global_seq));
        try!(buffer.write_u32::<LittleEndian>(self.connect_seq));
        try!(buffer.write_u32::<LittleEndian>(self.protocol_version));
        try!(buffer.write_u32::<LittleEndian>(self.authorizer_len));
        try!(buffer.write_u8(self.flags));
        for b in &self.authorizer{
            try!(buffer.write_u8(b.clone()));
        }
        return Ok(buffer);
    }
}

#[derive(Debug)]
struct CephMsgrMsg {
    tag: CephMsg,//    u8 tag = 0x07;
    header: CephMsgHeader,
    footer: CephMsgFooter,
}

enum_from_primitive!{
#[derive(Debug, Clone)]
enum CephEntity{
    Mon=1,
    Mds=2,
    Osd=4,
    Client=8,
    Auth=20,
    Any=255
}
}

enum_from_primitive!{
#[derive(Debug, Clone)]
enum CephPriority{
    Low = 64,
    Default = 127,
    High = 196,
    Highest = 255,
}
}

enum_from_primitive! {
#[derive(Debug, Clone)]
enum CephMsg{
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
}
}

#[derive(Debug)]
struct CephEntityName{
    entity_type: CephEntity,
    num: u64,
}

struct Utime {
    tv_sec: u32,  // Seconds since epoch.
    tv_nsec: u32, // Nanoseconds since the last second.
}

// From src/include/msgr.h
#[derive(Debug)]
struct CephMsgHeader {
    sequence_num: u64,
    transaction_id: u64,
    msg_type: u16,  //CEPH_MSG_* or MSG_*
    priority: CephPriority,
    version: u16,   //version of message encoding
    front_len: u32, // The size of the front section
    middle_len: u32,// The size of the middle section
    data_len: u32,  // The size of the data section
    data_off: u16,  // The way data should be aligned by the reciever
    entity_name: CephEntityName, // Information about the sender
    compat_version: u16, // Oldest compatible encoding version
    reserved: u16, // Unused
    crc: u32,  // CRC of header
}

impl CephPrimitive for CephMsgHeader{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{

        let sequenece_num = try!(cursor.read_u64::<LittleEndian>());
        let transcation_id = try!(cursor.read_u64::<LittleEndian>());
        let msg_type = try!(cursor.read_u16::<LittleEndian>());
        let priority = try!(cursor.read_u16::<LittleEndian>());
        println!("Priority: {}", priority);
        let version = try!(cursor.read_u16::<LittleEndian>());
        let front_len = try!(cursor.read_u32::<LittleEndian>());
        let middle_len = try!(cursor.read_u32::<LittleEndian>());
        let data_len = try!(cursor.read_u32::<LittleEndian>());
        let data_off = try!(cursor.read_u16::<LittleEndian>());

        let entity_type = try!(cursor.read_u8());
        println!("Entity_type: {}", entity_type);
        let entity_id = try!(cursor.read_u64::<LittleEndian>());

        let compat_version = try!(cursor.read_u16::<LittleEndian>());
        let reserved = try!(cursor.read_u16::<LittleEndian>());
        let crc = try!(cursor.read_u32::<LittleEndian>());

        return Ok(
            CephMsgHeader{
            sequence_num: sequenece_num,
            transaction_id: transcation_id,
            msg_type: msg_type,
            priority: CephPriority::from_u16(priority).unwrap(),
            version: version,
            front_len: front_len,
            middle_len: middle_len,
            data_len: data_len,
            data_off: data_off,
            entity_name: CephEntityName{
                entity_type: CephEntity::from_u8(entity_type).unwrap(),
                num: entity_id,
            },
            compat_version: compat_version,
            reserved: reserved,
            crc: crc,
            }
        );
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        let mut buffer:Vec<u8> = Vec::new();
        try!(buffer.write_u64::<LittleEndian>(self.sequence_num));
        try!(buffer.write_u64::<LittleEndian>(self.transaction_id));
        try!(buffer.write_u16::<LittleEndian>(self.msg_type));
        try!(buffer.write_u16::<LittleEndian>(self.priority.clone() as u16));
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
struct CephMsgFooter {
    front_crc: u32,
    middle_crc: u32,
    data_crc: u32,
    crypto_sig: u64,
    flags: u8
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
/*
impl CephPrimitive for CephMsgTagAck{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgTagAck, SerialError>{
        return;
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        return Ok(Vec::new());
    }
}
*/
struct CephMsgKeepAlive{
    tag: CephMsg, //0x09
    data: u8, // No data
}
/*
impl CephPrimitive for CephMsgKeepAlive{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive, SerialError>{
        return;
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        return Ok(Vec::new());
    }
}
*/
struct CephMsgKeepAlive2{
    tag: CephMsg, //0x0E
    timestamp: Utime,
}

impl CephPrimitive for CephMsgKeepAlive2{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag = try!(cursor.read_u8());
        let msg = CephMsg::from_u8(tag).unwrap();
        let tv_sec = try!(cursor.read_u32::<LittleEndian>());
        let tv_nsec = try!(cursor.read_u32::<LittleEndian>());
        let time = Utime {
            tv_sec: tv_sec,
            tv_nsec: tv_nsec,
        };
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

impl CephPrimitive for CephMsgKeepAlive2Ack{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        let tag = try!(cursor.read_u8());
        let msg = CephMsg::from_u8(tag).unwrap();

        let tv_sec = try!(cursor.read_u32::<LittleEndian>());
        let tv_nsec = try!(cursor.read_u32::<LittleEndian>());
        let time = Utime {
            tv_sec: tv_sec,
            tv_nsec: tv_nsec,
        };
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

struct EntityAddr{
    port: u16,
    nonce: u32,
    v4addr: Option<Ipv4Addr>,
    v6addr: Option<Ipv6Addr>,
}

fn encode_entity_addr(addr: EntityAddr) -> Result<Vec<u8>, SerialError>{
    //socket_type
    let mut buffer:Vec<u8> = Vec::new();
    try!(buffer.write_u32::<LittleEndian>(0)); //TODO: Lookup what this should be
    try!(buffer.write_u32::<LittleEndian>(addr.nonce));

    if addr.v4addr.is_some(){
        //Address Family
        try!(buffer.write_u32::<LittleEndian>(0x0002));

        //Port
        try!(buffer.write_u16::<BigEndian>(addr.port));
        let tmp = addr.v4addr.unwrap();
        for octet in tmp.octets().iter(){
            try!(buffer.write_u8(*octet));
        }
    }else if addr.v6addr.is_some(){
        //Address Family
        try!(buffer.write_u32::<LittleEndian>(0x000A));

        //Port
        try!(buffer.write_u16::<BigEndian>(addr.port));

        let tmp = addr.v6addr.unwrap();
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

fn decode_entity_addr<R: Read>(cursor: &mut R)->Result<EntityAddr, SerialError>{
    //type
    let addr_type = try!(cursor.read_u32::<LittleEndian>());
    println!("Type: {}", addr_type);
    let nonce = try!(cursor.read_u32::<LittleEndian>());
    println!("Nonce: {}", nonce);
    //type-str
    let address_family = try!(cursor.read_u16::<BigEndian>());
    println!("Address_family: {}", address_family);
    match address_family{
        0x0002 => {
            println!("IPv4 Addr");
            let port = try!(cursor.read_u16::<BigEndian>());
            println!("Port {}", port);
            let a = try!(cursor.read_u8());
            let b = try!(cursor.read_u8());
            let c = try!(cursor.read_u8());
            let d = try!(cursor.read_u8());
            let ip = Ipv4Addr::new(a,b,c,d);
            println!("IPv4 Addr_string: {}", ip);
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
            println!("Unknown addr type");
            return Err(
                SerialError::new(format!("unknown ip address family: {}", address_family))
            );
        }
    }
}

/*
struct ceph_list<T> {
        u32le length;
        T     elements[length];
}
 */
impl <T>CephPrimitive for Vec<T>{
	fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, SerialError>{
        return Ok(Vec::new())
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
        return Ok(Vec::new());
    }
}

//Connect to Ceph Monitor and send a hello banner
fn send_banner(socket: &mut TcpStream)->Result<(), SerialError>{
    let banner = String::from("ceph v027");
    let written_bytes = try!(socket.write(banner.as_bytes()));
    if written_bytes != 0{
        return Err(SerialError::new("blah".to_string()));
    }else{
        return Ok(());
    }
}

fn send_msg(socket: &mut TcpStream){

}

fn recv_msg(socket: &mut TcpStream){

}

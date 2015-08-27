extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::ErrorKind;
use std::io::Cursor;
use std::io::prelude::*;
use std::net::TcpStream;
//There will be no padding between the elements and the elements will be sent in the order they appear
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
    //Replay captured data and test results
    #[test]
    fn test_connect(){

    }
    #[test]
    fn test_connect_reply(){

    }

}

#[derive(Debug)]
pub enum SerialError {
	IoError(io::Error),
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

impl From<io::Error> for SerialError {
    fn from(err: io::Error) -> SerialError {
        SerialError::IoError(err)
    }
}

pub struct CephReader<'a> {
	reader : Cursor<&'a [u8]>
}

impl<'a> CephReader<'a> {
	pub fn new(x:&'a Vec<u8>) -> CephReader<'a> {
		CephReader{
            reader : Cursor::new(&x)
        }
	}
}

pub struct CephWriter {
	writer : Cursor<Vec<u8>>
}

impl CephWriter {
	pub fn new() -> CephWriter {
		let v : Vec<u8> = Vec::new();
		CephWriter{
            writer: Cursor::new(v)
        }
	}
    pub fn into_buffer(self) -> Vec<u8> {
		self.writer.into_inner()
	}

}

trait CephPrimitive {
	fn read_from_wire(x: &mut CephReader) -> Result<Self, SerialError>;
	fn write_to_wire(x: &mut CephWriter, v: Self) -> Option<SerialError>;
}

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

struct CephMsgrMsg {
    tag: CephMsg,//    u8 tag = 0x07;
    header: CephMsgHeader,
    footer: CephMsgFooter,
}

enum CephEntity{
    Mon=1,
    Mds=2,
    Osd=4,
    Client=8,
    Auth=20,
    Any=255
}

enum CephPriority{
    Low = 64,
    Default = 127,
    High = 196,
    Highest = 255,
}

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

struct CephEntityName{
    entity_type: CephEntity,
    num: u64,
}

struct Utime {
    tv_sec: u32,  // Seconds since epoch.
    tv_nsec: u32, // Nanoseconds since the last second.
}

// From src/include/msgr.h
struct CephMsgHeader {
    sequence: u64,
    transaction_id: u64,
    msg_type: u16, //CEPH_MSG_* or MSG_*
    priority: u16,
    version: u16,
    front_len: u32,
    middle_len: u32,
    data_len: u32,
    data_off: u16,
    entity_name: CephEntityName,
    compat_version: u16,
    reserved: u16,
    crc: u32,
}

impl CephPrimitive for CephMsgHeader{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgHeader, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

struct CephMsgFooter {
    front_crc: u32,
    middle_crc: u32,
    data_crc: u32,
    crypto_sig: u64,
    flags: u8
}

impl CephPrimitive for CephMsgFooter{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgFooter, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

struct CephMsgTagAck{
    tag: CephMsg, //0x08
    seq: u64 //Sequence number of msg being acknowledged
}

impl CephPrimitive for CephMsgTagAck{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgTagAck, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

struct CephMsgKeepAlive{
    tag: CephMsg, //0x09
    data: u8, // No data
}

impl CephPrimitive for CephMsgKeepAlive{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

struct CephMsgKeepAlive2{
    tag: CephMsg, //0x0E
    timestamp: Utime,
}

impl CephPrimitive for CephMsgKeepAlive2{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive2, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

struct CephMsgKeepAlive2Ack{
    tag: CephMsg, //0x0F
    timestamp: Utime,
}

impl CephPrimitive for CephMsgKeepAlive2Ack{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive2Ack, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}

/*
struct ceph_list<T> {
        u32le length;
        T     elements[length];
}
 */
impl <T>CephPrimitive for Vec<T>{
	fn read_from_wire(x: &mut CephReader) -> Result<Self, SerialError>{
        return Ok(Vec::new())
    }
	fn write_to_wire(x: &mut CephWriter, v: Self) -> Option<SerialError>{
        return None
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

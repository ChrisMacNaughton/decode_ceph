extern crate byteorder;
use self::byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::ErrorKind;
use std::io::Cursor;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::TcpStream;
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
    use std::net::TcpStream;
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
        //Get the sockaddr_storage
        /*
        struct sockaddr_storage {
            sa_family_t  ss_family;     // address family //unsigned int

            // all this is padding, implementation specific, ignore it:
            char      __ss_pad1[_SS_PAD1SIZE];
            int64_t   __ss_align;
            char      __ss_pad2[_SS_PAD2SIZE];
         };*/

        //recv this:
        let mut ceph_response_bytes = vec![
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x02,0x1a,0x85,0x0a,0x00,0x03,0xd8,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
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
        super::decode_entity_addr_t(&mut cursor);
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
/*
impl CephPrimitive for CephMsgHeader{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgHeader, SerialError>{
        return Ok(
            CephMsgHeader{
            sequence: 0,
            transaction_id: 0,
            msg_type: 0,
            priority: 0,
            front_len: 0,
            middle_len: 0,
            data_len: 0,
            data_off: 0,
            entity_name: CephEntityName{
                entity_type: CephEntity::Mon,
                num: 0
            },
            compat_version: 0,
            reserved: 0,
            crc: 0,
            }
            );
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}
*/
struct CephMsgFooter {
    front_crc: u32,
    middle_crc: u32,
    data_crc: u32,
    crypto_sig: u64,
    flags: u8
}
/*
impl CephPrimitive for CephMsgFooter{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgFooter, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}
*/
struct CephMsgTagAck{
    tag: CephMsg, //0x08
    seq: u64 //Sequence number of msg being acknowledged
}
/*
impl CephPrimitive for CephMsgTagAck{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgTagAck, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
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
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}
*/
struct CephMsgKeepAlive2{
    tag: CephMsg, //0x0E
    timestamp: Utime,
}
/*
impl CephPrimitive for CephMsgKeepAlive2{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive2, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}
*/
struct CephMsgKeepAlive2Ack{
    tag: CephMsg, //0x0F
    timestamp: Utime,
}
/*
impl CephPrimitive for CephMsgKeepAlive2Ack{
    fn read_from_wire(x: &mut CephReader)->Result<CephMsgKeepAlive2Ack, SerialError>{
        return;
    }
    fn write_to_wire(x: &mut CephWriter, v: Self)->Option<SerialError>{
        return None
    }
}
*/
fn decode_entity_addr_t<R: Read>(cursor: &mut R){
    /*
    C_SIZE_SOCKADDR_STORAGE = 128
    943         V(C_IPv4, 0x0002, "IPv4") \
    944         V(C_IPv6, 0x000A, "IPv6")

    1898         /*
    1899         struct sockaddr_storage {
    1900                 guint16 family;
    1901                 guint8  pad[???]; // Implementation defined.
    1902         };
    1903         struct sockaddr_in {
    1904                 guint16 family;
    1905                 guint16 port;
    1906                 guint32 addr;
    1907                 guint8  pad[8];
    1908         };
    1909         struct sockaddr_in6 {
    1910                 guint16 family;
    1911                 guint16 port;
    1912                 guint32 flow;
    1913                 guint8  addr[16];
    1914                 guint32 scope;
    1915         };
    1916         */
    1917
     */
    //type
    let addr_type = cursor.read_u32::<LittleEndian>().unwrap();
    println!("Type: {}", addr_type);
    //type-str
    let nonce = cursor.read_u32::<LittleEndian>().unwrap();
    println!("Nonce: {}", nonce);
    let address_family = cursor.read_u16::<BigEndian>().unwrap();
    println!("Address_family: {}", address_family);
    match address_family{
        0x0002 => {
            println!("IPv4 Addr");
            let port = cursor.read_u16::<BigEndian>().unwrap();
            println!("Port {}", port);
            let a = cursor.read_u8().unwrap();
            let b = cursor.read_u8().unwrap();
            let c = cursor.read_u8().unwrap();
            let d = cursor.read_u8().unwrap();
            let ip = Ipv4Addr::new(a,b,c,d);
            println!("IPv4 Addr_string: {}", ip);
        },
        0x000A =>{
            //TODO: Test this
            println!("IPv6 Addr");
            let port = cursor.read_u16::<BigEndian>().unwrap();
            println!("Port {}", port);
            let a = cursor.read_u16::<BigEndian>().unwrap();
            let b = cursor.read_u16::<BigEndian>().unwrap();
            let c = cursor.read_u16::<BigEndian>().unwrap();
            let d = cursor.read_u16::<BigEndian>().unwrap();
            let e = cursor.read_u16::<BigEndian>().unwrap();
            let f = cursor.read_u16::<BigEndian>().unwrap();
            let g = cursor.read_u16::<BigEndian>().unwrap();
            let h = cursor.read_u16::<BigEndian>().unwrap();
            let ip = Ipv6Addr::new(a,b,c,d,e,f,g,h);
            println!("IPv6 Addr_string: {}", ip);
        }
        _ => {
            println!("Unknown addr type");
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

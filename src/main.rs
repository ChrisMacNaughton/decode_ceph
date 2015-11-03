#![allow(dead_code)]
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate log;
extern crate byteorder;
extern crate ease;
extern crate num;
extern crate output_args;
extern crate pcap;
extern crate users;
extern crate simple_logger;
extern crate time;
extern crate influent;

mod serial;
use serial::{CephPrimitive};
mod crypto;

use byteorder::{BigEndian, ReadBytesExt};
use log::LogLevel;
use pcap::{Capture, Device};

use std::io::Cursor;
use std::io::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr, TcpStream};
use std::str::FromStr;

use influent::create_client;
use influent::client::Client;
use influent::client::Credentials;
use influent::measurement::{Measurement, Value};
use output_args::*;

#[cfg(test)]
mod tests{
    extern crate output_args;
    extern crate pcap;

    use std::io::Cursor;
    use std::net::Ipv4Addr;
    use std::path::Path;
    use pcap::{Capture, Device};
    use log;
    use output_args::*;
    use super::serial;

    #[test]
    fn test_packet_parsing(){
        let mut v4_packet2: Vec<u8> = vec![
            0x00 ,0x16 ,0x3e ,0x4c ,0x7b ,0xa0 , //Dst MAC
            0x00 ,0x16 ,0x3e ,0x53 ,0x4e ,0xf2 , //Src MAC
            0x08 ,0x00, //IP TYPE 14
            0x45 ,0x00 ,0x01 ,0x24 ,0x52 ,0xf6 ,0x40 ,0x00 ,0x40 ,0x06 ,0xcb ,0xeb , //26
            0x0a ,0x00, 0x03, 0x63 , //SRC IP //30
            0x0a ,0x00 ,0x03 ,0x90 , //DST IP //34
            0xea ,0x6d , //SRC port //36
            0x1a ,0x90 , //DST port //38
            0x38 ,0xdc ,0xc2 ,0x31 , //42
            0x66 ,0xd4 ,0x41 ,0xae , //46
            0x80 ,0x18 ,0x00 ,0xf5 , //50
            0x1c ,0x09 ,0x00 ,0x00 , //54
            0x01 ,0x01 ,0x08 ,0x0a,  //58
            0x00 ,0x05 ,0x24 ,0x07 , //62
            0x00 ,0x05 ,0x14 ,0x27 , //66
            0x07 ,0x02 ,0x00 ,0x00 , //70
            0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x03 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x2a , //84
            0x00 ,0x3f ,0x00, 0x04 ,0x00 ,0x99 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x0c , //98
            0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x08 ,0x9a ,0x11 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 , //112
            0x03 ,0x00 ,0x00 ,0x00, 0x54 ,0x3b ,0x7e ,0x6a ,

            0x00 ,0x00 ,0x00 ,0x00 ,0x5a ,0x00 , //CephOsdOperation 126
            0x00 ,0x00 ,0x24 ,0x00 ,0x00 ,0x00 ,0x14 ,0x7e ,0xf3 ,0x55 ,0xf0 ,0x88 ,0x81 ,0x32 , //140
            0x00 ,

            0x00 ,0x00 ,0x00 ,0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x03, 0x1c, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8f, 0x8d, 0x49, 0x97, 0xff, 0xff,
            0xff, 0xff, 0x02, 0x00, 0x00, 0x00, 0x68, 0x77, 0x01, 0x00, 0x02, 0x22, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
            0xed, 0x70, 0x12, 0xf6, 0x00, 0x00, 0x00, 0x00, 0x81, 0x44, 0x0c, 0xd5, 0x01, 0xe1,
            0xac, 0x17, 0x8e, 0x5a, 0xd0, 0x6c, 0x05];

        let mut v4_packet: Vec<u8> = vec![
            0, 22, 62, 76, 123, 160, //Dst Mac
            0, 22, 62, 83, 78, 242, //Src mac
            8, 0, //Ethertype
            69, 0, //IP header Packet
            0, 235, 160, 122, 64, 0, 64, 6,
            251, 204,//New position
            10, 0, 3, 99, //src ip
            10, 0, 3, 144, //dst ip
            173, 186, 26, 133, 86, 2, 236, 160, 131, 235, 201, 242, 128, 24, 11, 227, 27,
            208, 0, 0, 1, 1, 8, 10, 4, 202, 237, 131, 4, 202, 237, 87, 7, 67, 255, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 6, 196, 0, 1, 0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 232, 89, 107, 135, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 3, 1, 82, 0, 0, 0, 0, 176, 158, 40
        ];
        let mut cursor = Cursor::new(&v4_packet2[..]);
        cursor.set_position(12);
        let packer_header = super::parse_etherframe(&mut cursor).unwrap();

        //Validate the header parsing results
        let valid_header = super::PacketHeader{
            src_port: 60013,
            dst_port: 6800,
            src_v4addr: Some(Ipv4Addr::new(10,0,3,99)),
            dst_v4addr: Some(Ipv4Addr::new(10,0,3,144)),
            src_v6addr: None,
            dst_v6addr: None,
        };
        let valid_ceph_header = serial::CephMsgHeader{
            sequence_num: 2,
            transaction_id: 3,
            msg_type: serial::CephMsgType::MsgOsdOp,
            priority: 63,
            version: 4,
            front_len: 153,
            middle_len: 0,// The size of the middle section
            data_len: 12,
            data_off: 0,  // The way data should be aligned by the reciever
            entity_name: serial::CephSourceName{
                entity_type: serial::CephEntity::Client,
                num: 4506,
            },
            compat_version: 3,
            reserved: 0,
            crc: 1786657620,
        };

        let valid_ceph_osd_op = serial::CephOsdOperation{
            client: 0,
            map_epoch: 90,
            flags: serial::OsdOp::from_bits(0x0004| 0x0020).unwrap(),
            modification_time: serial::Utime { tv_sec: 1442020884, tv_nsec: 847350000 },
            reassert_version: 0, reassert_epoch: 0,
            locator: serial::ObjectLocator { encoding_version: 6, min_compat_version: 3, size: 28, pool: 0, namespace_size: 0, namespace_data: vec![] },
            placement_group: serial::PlacementGroup { group_version: 1, pool: 0, seed: 2538179983, preferred: 4294967295 },
            object_id: serial::ObjectId { size: 2, data: vec![104, 119] },
            operation_count: 1,
            operation: serial::Operation { operation: 8706, flags: 0, offset: 0, size: 12, truncate_size: 0, truncate_seq: 0, payload_size: 12 },
            snapshot_id: 18446744073709551614,
            snapshot_seq: 0,
            snapshot_count: 0,
            retry_attempt: 0,
            payload: vec![],
        };

        let valid_ceph_footer = serial::CephMsgFooter{
            front_crc: 0,
            middle_crc: 0,
            data_crc: 0,
            crypto_sig: 0,
            flags: 0,
        };

        assert_eq!(packer_header, valid_header);

        let result = super::dissect_msgr(&mut cursor).unwrap();
        println!("Result: {:?}", result);
        assert_eq!(result.header, valid_ceph_header);
        assert_eq!(result.msg, serial::Message::OsdOp(valid_ceph_osd_op));
        assert_eq!(result.footer, valid_ceph_footer);
    }

    #[test]
    fn test_pcap_parsing(){
        let args = output_args::Args {
            carbon: None,
            elasticsearch: None,
            stdout: Some("stdout".to_string()),
            influx: None,
            outputs: vec!["elasticsearch".to_string(), "carbon".to_string(), "stdout".to_string()],
            config_path: "".to_string(),
            log_level: log::LogLevel::Info
        };
        //Set the cursor so the parsing doesn't fail
        let mut cap = Capture::from_file(Path::new("ceph.pcap")).unwrap();
        while let Some(packet) = cap.next() {
            //We received a packet
            let data = packet.data;
            let mut cursor = Cursor::new(&data[..]);

            //Try to parse the packet headers, src, dst and ports
            cursor.set_position(12);
            match super::parse_etherframe(&mut cursor){
                //The packet parsing was clean
                Ok(header) => {
                    //Try to parse some Ceph info from the packet
                    if let Ok(dissect_result) = super::dissect_msgr(&mut cursor){
                        let print_result = super::process_packet(header, dissect_result, &args);
                        //println!("Processed packet: {:?}", &print_result);
                    }else{
                        //Failed to parse Ceph packet.  Ignore
                        //println!("Failed to dissect ceph packet from raw packet: {:?}", cursor);
                    }
                }
                //The packet parsing failed
                Err(err) => {
                    //println!("Invalid etherframe: {:?}", err)
                }
            };
        }
    }
}

//TODO expose even more data
#[derive(Debug)]
struct Document<'a>{
    header: &'a PacketHeader,
    flags: serial::OsdOp,
    operation_count: u16,
    //placement_group: serial::PlacementGroup,
    size: u64,
    timestamp: u64, //Milliseconds since epoch
}

// JSON value representation
impl<'a> Document<'a>{
    fn to_carbon_string(&self, root_key: &str)->Result<String, String>{
        let src_addr: String = match self.header.src_v4addr{
            Some(addr) => addr.to_string(),
            None => {
                match self.header.src_v6addr{
                    Some(addr) => addr.to_string(),
                    None => "".to_string(),
                }
            },
        };

        let dst_addr: String = match self.header.dst_v4addr{
            Some(addr) => addr.to_string(),
            None => {
                match self.header.dst_v6addr{
                    Some(addr) => addr.to_string(),
                    None => "".to_string(),
                }
            },
        };
        //NOTE: carbon uses epoch time aka seconds since epoch not milliseconds
        let carbon_string = format!( r#"
{root_key}.src_ip {} {timestamp}
{root_key}.dst_ip {} {timestamp}
{root_key}.flags {:?} {timestamp}
{root_key}.operation_count {} {timestamp}
{root_key}.size {} {timestamp}
  "#, src_addr, dst_addr, self.flags, self.operation_count, self.size, root_key = root_key,
  timestamp = (self.timestamp/1000));

        return Ok(carbon_string);
    }
    fn to_json(&self)->Result<String, String>{

        let src_addr: String = match self.header.src_v4addr{
            Some(addr) => addr.to_string(),
            None => {
                match self.header.src_v6addr{
                    Some(addr) => addr.to_string(),
                    None => "".to_string(),
                }
            },
        };

        let dst_addr: String = match self.header.dst_v4addr{
            Some(addr) => addr.to_string(),
            None => {
                match self.header.dst_v6addr{
                    Some(addr) => addr.to_string(),
                    None => "".to_string(),
                }
            },
        };


        return Ok(format!("{{\"src_ip\": \"{}\",\"dst_ip\": \"{}\", \"operation\":\"{:?}\", \
            \"operation_count\":{}, \"size\":{}, \"postDate\":\"{}\"}}",
            src_addr,
            dst_addr,
            self.flags,
            self.operation_count,
            self.size,
            self.timestamp));
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct PacketHeader{
    pub src_port: u16,
    pub dst_port: u16,

    pub src_v4addr: Option<Ipv4Addr>,
    pub dst_v4addr: Option<Ipv4Addr>,

    pub src_v6addr: Option<Ipv6Addr>,
    pub dst_v6addr: Option<Ipv6Addr>,
}

fn get_arguments() -> output_args::Args {
    output_args::get_args()
}

fn check_user()->Result<(), ()>{
    let current_user = users::get_current_uid();
    if current_user != 0 {
        error!("This program must be run by root to access the network devices");
        return Err(());
    }
    return Ok(());
}

fn read_v4ip<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<Ipv4Addr, serial::SerialError>{
        let a = try!(cursor.read_u8());
        let b = try!(cursor.read_u8());
        let c = try!(cursor.read_u8());
        let d = try!(cursor.read_u8());

        let ip = Ipv4Addr::new(a,b,c,d);
        return Ok(ip);
}

fn read_v6ip<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<Ipv6Addr, serial::SerialError>{
        let a = try!(cursor.read_u16::<BigEndian>());
        let b = try!(cursor.read_u16::<BigEndian>());
        let c = try!(cursor.read_u16::<BigEndian>());
        let d = try!(cursor.read_u16::<BigEndian>());
        let e = try!(cursor.read_u16::<BigEndian>());
        let f = try!(cursor.read_u16::<BigEndian>());
        let g = try!(cursor.read_u16::<BigEndian>());
        let h = try!(cursor.read_u16::<BigEndian>());

        let ip = Ipv6Addr::new(a,b,c,d,e,f,g,h);
        debug!("Ipv6Addr parsed: {:?}", &ip);
        return Ok(ip);
}

//Takes a cursor to a byte array and parses ip info from it
fn parse_etherframe<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<PacketHeader, serial::SerialError>{
    //This is done now in the loop
    //cursor.set_position(12);
    let ethertype = try!(cursor.read_u16::<BigEndian>());

    if ethertype == 0x0800{
        let mut current_pos = cursor.position();
        cursor.set_position(current_pos + 12);

        let src_ip = try!(read_v4ip(cursor));
        let dst_ip = try!(read_v4ip(cursor));

        let src_port = try!(cursor.read_u16::<BigEndian>());
        let dst_port = try!(cursor.read_u16::<BigEndian>());

        //Skip the TCP header bullshit
        current_pos = cursor.position();
        cursor.set_position(current_pos + 28);

        return Ok(
            PacketHeader{
                src_port: src_port,
                dst_port: dst_port,
                src_v4addr: Some(src_ip),
                dst_v4addr: Some(dst_ip),
                src_v6addr: None,
                dst_v6addr: None,
            }
        );
    }else if  ethertype == 0x86DD{
        //let mut current_pos = cursor.position();
        //cursor.set_position(current_pos + 12);

        let src_ip = try!(read_v6ip(cursor));
        let dst_ip = try!(read_v6ip(cursor));

        let src_port = try!(cursor.read_u16::<BigEndian>());
        let dst_port = try!(cursor.read_u16::<BigEndian>());

        //Skip the TCP header bullshit
        let current_pos = cursor.position();
        cursor.set_position(current_pos + 160);

        return Ok(
            PacketHeader{
                src_port: src_port,
                dst_port: dst_port,
                src_v4addr: None,
                dst_v4addr: None,
                src_v6addr: Some(src_ip),
                dst_v6addr: Some(dst_ip),
            }
        );
    }else{
        return Err(serial::SerialError::new(format!("Unknown Packet type: {}", ethertype)));
    }
}

fn get_time()->u64{
    let now = time::now();
    let milliseconds_since_epoch = now.to_timespec().sec * 1000;
    return milliseconds_since_epoch as u64;
}

fn log_to_stdout(){

}

fn log_packet_to_carbon(carbon_url: &str, data: String)->Result<(), String>{
    //Carbon is plaintext
    //echo "local.random.diceroll 4 `date +%s`" | nc -q0 ${SERVER} ${PORT}

    let mut stream = try!(TcpStream::connect(carbon_url).map_err(|e| e.to_string()));
    let bytes_written = try!(stream.write(&data.into_bytes()[..]).map_err(|e| e.to_string()));
    info!("Wrote: {} bytes to graphite", &bytes_written);

    return Ok(());
}

fn log_packet_to_es(url: &str, json: &String)->Result<(), String>{
    debug!("Logging to {}", url);
    let parsed_url = try!(ease::Url::parse(url).map_err(|e| e.to_string()));
    let mut req = ease::Request::new(parsed_url);
    req.body(json.clone());
    match req.post(){
        Ok(_) => {
            info!("Logged to ES");
            return Ok(());},
        Err(_) => {
            error!("ES POST FAILED");
            return Err("Post operation failed".to_string());
        }
    };
}

fn parse_carbon_url(url: &String)->Result<(String, u16), String>{
    let parts: Vec<&str> = url.split(":").collect();
    if parts.len() == 2{
        let carbon_host = parts[0].to_string();
        let carbon_port = try!(u16::from_str(parts[1]).map_err(|e| e.to_string()));
        return Ok((carbon_host, carbon_port));
    }else{
        return Err("Invalid carbon host specification.  See CLI example".to_string());
    }
}

fn log_msg_to_carbon(header: &PacketHeader, msg: &serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    if output_args.carbon.is_some(){
        for ceph_msg in &msg.msg{

            let op = match *ceph_msg {
                serial::Message::OsdOp(ref osd_op) => osd_op,
                serial::Message::OsdSubop(ref sub_op) => sub_op,
                _ => return Err("Bad type".to_string())
            };

            let carbon = output_args.clone().carbon.unwrap();

            let carbon_host = carbon.host.clone();
            let carbon_port = carbon.port.clone();
            let carbon_url = format!("{}:{}", carbon_host, carbon_port);
            let carbon_root_key = carbon.root_key.clone();

            let milliseconds_since_epoch = get_time();
            let doc = Document{
                header: header,
                flags: op.flags,
                operation_count: op.operation_count,
                size: op.operation.size,
                timestamp: milliseconds_since_epoch,
            };
            let carbon_data = format!("{}.{}", carbon_root_key, try!(doc.to_carbon_string(&carbon.root_key)));
            try!(log_packet_to_carbon(&carbon_url, carbon_data));

        }
    }
    Ok(())
}

fn log_msg_to_elasticsearch(header: &PacketHeader, msg: &serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    if output_args.elasticsearch.is_some() && output_args.outputs.contains(&"elasticsearch".to_string()){
        for ceph_msg in &msg.msg{
            let op = match *ceph_msg {
                serial::Message::OsdOp(ref osd_op) => osd_op,
                serial::Message::OsdSubop(ref sub_op) => sub_op,
                _ => return Err("Bad type".to_string())
            };
            let milliseconds_since_epoch = get_time();
            let doc = Document{
                header: header,
                flags: op.flags,
                operation_count: op.operation_count,
                size: op.operation.size,
                timestamp: milliseconds_since_epoch,
            };
            let doc_json = try!(doc.to_json());
            //It's ok to unwrap here because we checked is_some() above
            // try!(log_packet_to_es("http://10.0.3.144:9200/ceph/operations", &doc_json));
            try!(log_packet_to_es(&output_args.elasticsearch.clone().unwrap(), &doc_json));
        }
    }
    Ok(())
}

fn log_msg_to_stdout(header: &PacketHeader, msg: &serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    if output_args.stdout.is_some(){
        for ceph_msg in &msg.msg{
            let op = match *ceph_msg {
                serial::Message::OsdOp(ref osd_op) => osd_op,
                serial::Message::OsdSubop(ref sub_op) => sub_op,
                _ => return Err("Bad type".to_string())
            };
            let now = time::now();
            let time_spec = now.to_timespec();
            //TODO Expand this
            println!("{}", format!("ceph.{}.{:?}.{} {}",
                &header.src_v4addr.unwrap(),
                op.flags,
                op.operation.size,
                time_spec.sec)
            );
        }
    }
    Ok(())
}

fn log_msg_to_influx(header: &PacketHeader, msg: &serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    if output_args.influx.is_some() && output_args.outputs.contains(&"influx".to_string()) {
        for ceph_msg in &msg.msg{
            let op = match *ceph_msg {
                serial::Message::OsdOp(ref osd_op) => osd_op,
                serial::Message::OsdSubop(ref sub_op) => sub_op,
                _ => return Err("Bad type".to_string())
            };

            let influx = &output_args.influx.clone().unwrap();
            let credentials = Credentials {
                username: influx.user.as_ref(),
                password: influx.password.as_ref(),
                database: "ceph"
            };
            let host = format!("http://{}:{}",influx.host, influx.port);
            let hosts = vec![host.as_ref()];
            let client = create_client(credentials, hosts);



            let src_addr: String = match header.src_v4addr{
                Some(addr) => addr.to_string(),
                None => {
                    match header.src_v6addr{
                        Some(addr) => addr.to_string(),
                        None => "".to_string(),
                    }
                },
            };

            let dst_addr: String = match header.dst_v4addr{
                Some(addr) => addr.to_string(),
                None => {
                    match header.dst_v6addr{
                        Some(addr) => addr.to_string(),
                        None => "".to_string(),
                    }
                },
            };
            let size = op.operation.size as f64;
            let count = op.operation_count as i64;
            let flags: String = format!("{:?}", op.flags).clone();
            let mut measurement = Measurement::new("ceph");
            measurement.add_tag("src_address", src_addr.as_ref());
            measurement.add_tag("dst_address", dst_addr.as_ref());
            measurement.add_field("size", Value::Float(size));
            measurement.add_field("operation", Value::String(flags.as_ref()));
            measurement.add_field("count", Value::Integer(count));

            let res = client.write_one(measurement, None);
            debug!("{:?}", res);
        }
    }
    Ok(())
}

fn process_packet(header: PacketHeader, msg: serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    //Process OSD operation packets
    let _ = log_msg_to_carbon(&header, &msg, output_args);
    let _ = log_msg_to_elasticsearch(&header, &msg, output_args);
    let _ = log_msg_to_stdout(&header, &msg, output_args);
    let _ = log_msg_to_influx(&header, &msg, output_args);
    Ok(())
}

//MSGR is Ceph's outer message protocol
fn dissect_msgr<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<serial::CephMsgrMsg, serial::SerialError>{
    let result = try!(serial::CephMsgrMsg::read_from_wire(cursor));
    return Ok(result);
}

fn main() {
    //TODO make configurable via cli or config arg
    simple_logger::init_with_level(LogLevel::Info).unwrap();

    match check_user(){
        Ok(_) => {},
        Err(_) =>{
            return;
        }
    };
    let args = get_arguments();
    for output in &args.outputs {
        info!("Logging to {}", output);
    }

    let dev_list = match Device::list(){
        Ok(l) => l,
        Err(e) => {
            error!("Unable to list network devices.  Error: {}", e);
            return;
        }
    };

    info!("Searching for network devices");
    for dev_device in dev_list{
        if dev_device.name == "any"{
            let device_name = dev_device.name.clone();
            let mut cooked_header = false;

            info!("Found Network device {}", &device_name);
            info!("Setting up capture({})", &device_name);
            let mut cap = Capture::from_device(dev_device).unwrap() //open the device
                          .promisc(true)
                          //.snaplen(500) //Might need this still if we're losing packets
                          .timeout(50)
                          .open() //activate the handle
                          .unwrap(); //assume activation worked
            let link_list = cap.list_datalinks().unwrap();
            //Try to detect "cooked" headers and set the cursor position properly
            for link_type in link_list{
                info!("Device datalink({}): {:?}", &device_name, &link_type.get_name());
                match link_type{
                    //LINUX_SLL
                    pcap::Linktype(113) => {
                        cooked_header = true;
                    },
                    //Anything else
                    _ => {},
                }
            }
            info!("Setting up filter({})", &device_name);
            //Grab both monitor and OSD traffic
            match cap.filter("tcp portrange 6789-7300"){
                Ok(_) => {
                    info!("Filter successful({})", &device_name);
                },
                Err(e) => {
                    error!("Invalid capture filter({}). Error: {:?}", &device_name, e);
                    return;
                }
            }
            info!("Waiting for packets({})", &device_name);
            //Grab some packets :)

            //Infinite loop
            loop {
                match cap.next(){
                    //We received a packet
                    Some(packet) =>{
                        let data = packet.data;
                        let mut cursor = Cursor::new(&data[..]);

                        //Try to parse the packet headers, src, dst and ports
                        //match parse_etherframe(&mut cursor){
                        if cooked_header{
                            cursor.set_position(14);
                        }else{
                            cursor.set_position(12);
                        }
                        match parse_etherframe(&mut cursor){
                            //The packet parsing was clean
                            Ok(header) => {
                                //Try to parse some Ceph info from the packet
                                if let Ok(dissect_result) = dissect_msgr(&mut cursor){
                                    //Try to send the packet off to Elasticsearch, Carbon, stdout, etc
                                    //let args_clone = args.clone();
                                    let print_result = process_packet(header, dissect_result, &args);
                                    debug!("Processed packet({}): {:?}",&device_name, &print_result);
                                }else{
                                    //Failed to parse Ceph packet.  Ignore
                                    //debug!("Failed to dissect ceph packet from raw packet: {:?}", cursor);
                                }
                            }
                            //The packet parsing failed
                            Err(_) => {
                                //error!("Invalid etherframe: {:?}", err)
                            }
                        };
                    },
                    //We missed a packet, ignore
                    None => {},
                }
            }
        }
    }
}

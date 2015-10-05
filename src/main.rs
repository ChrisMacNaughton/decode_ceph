#![allow(dead_code)]
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate clap;
extern crate byteorder;
extern crate ease;
extern crate num;
extern crate pcap;
extern crate users;
extern crate time;

mod serial;
use serial::{CephPrimitive};
mod crypto;

use byteorder::{BigEndian, ReadBytesExt};
use clap::App;
use pcap::{Capture, Device};

use std::io::Cursor;
use std::io::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr, TcpStream};
use std::str::FromStr;

#[cfg(test)]
mod tests{
    use std::io::Cursor;
    use std::net::{Ipv4Addr};

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
        let packer_header = super::parse_etherframe(&mut cursor).unwrap();
        //assert_eq!(packer_header.src_port, 44474);
        //assert_eq!(packer_header.dst_port, 6789);
        //assert_eq!(packer_header.src_v4addr, Ipv4Addr(10,0,3,99));
        //assert_eq!(packer_header.dst_v4addr, Ipv4Addr(10,0,3,144));
        let result = super::dissect_msgr(&mut cursor);
        println!("Result: {:?}", result);
    }
}

#[derive(Debug)]
struct Args {
    carbon: Option<String>,
    elasticsearch: Option<String>,
    stdout: Option<String>,
    outputs: Vec<String>,
}

fn parse_option<'a, 'b>(option: &str, matches: &clap::ArgMatches<'a, 'b>) -> Option<String>{
    match matches.value_of(option){
        Some(opt) => Some(opt.to_string()),
        None => None,
    }
}

fn get_arguments() -> Args{
    let output_types = vec!["elastic_search", "carbon", "stdout"];
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut outputs:Vec<String> = Vec::new();

    if let Some(ref out) = matches.values_of("OUTPUTS") {
        for output in out.iter() {
            if output_types.contains(output) {
                outputs.push(output.to_string());
            } else {
                println!("{} is not a valid output type", output);
            }
        }
    }
    return Args{
        carbon: parse_option("CARBON", &matches),
        elasticsearch: parse_option("ES", &matches),
        stdout: parse_option("STDOUT", &matches),
        outputs: outputs,
    };
}

fn check_user()->Result<(), ()>{
    let current_user = users::get_current_uid();
    if current_user != 0 {
        println!("This program must be run by root to access the network devices");
        return Err(());
    }
    return Ok(());
}

#[derive(Debug)]
struct Document{
    header: PacketHeader,
    flags: serial::OsdOp,
    operation_count: u16,
    size: u64,
    timestamp: u64, //Milliseconds since epoch
}

// JSON value representation
impl Document{
    fn to_json(&self)->Result<String, String>{
        if self.header.src_v4addr.is_some() && self.header.dst_v4addr.is_some(){
            return Ok(format!("{{\"src_ip\": \"{}\",\"dst_ip\": \"{}\", \"operation\":\"{:?}\", \
                    \"operation_count\":{}, \"size\":{}, \"postDate\":\"{}\"}}",
                    self.header.src_v4addr.unwrap(),
                    self.header.dst_v4addr.unwrap(),
                    self.flags,
                    self.operation_count,
                    self.size,
                    self.timestamp));
        }else if self.header.src_v6addr.is_some() && self.header.dst_v6addr.is_some(){
            return Ok(format!("{{\"src_ip\": \"{}\",\"dst_ip\": \"{}\", \"operation\":\"{:?}\", \
                    \"operation_count\":{}, \"size\":{}, \"postDate\":\"{}\"}}",
                    self.header.src_v6addr.unwrap(),
                    self.header.dst_v6addr.unwrap(),
                    self.flags,
                    self.operation_count,
                    self.size,
                    self.timestamp));
        }else{
            return Err("src_v4addr or src_v6addr is missing".to_string());
        }
    }
}

#[derive(Debug)]
pub struct PacketHeader{
    pub src_port: u16,
    pub dst_port: u16,

    pub src_v4addr: Option<Ipv4Addr>,
    pub dst_v4addr: Option<Ipv4Addr>,

    pub src_v6addr: Option<Ipv6Addr>,
    pub dst_v6addr: Option<Ipv6Addr>,
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
        return Ok(ip);
}

//Takes a cursor to a byte array and parses ip info from it
fn parse_etherframe<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<PacketHeader, serial::SerialError>{
    cursor.set_position(12);
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

fn log_packet_to_carbon(server: &str, port: u16, data: String)->Result<(), String>{
    //Carbon is plaintext
    //echo "local.random.diceroll 4 `date +%s`" | nc -q0 ${SERVER} ${PORT}

    let mut stream = try!(TcpStream::connect((server, port)).map_err(|e| e.to_string()));
    let bytes_written = try!(stream.write(&data.into_bytes()[..]).map_err(|e| e.to_string()));
    println!("Wrote: {} bytes to graphite", &bytes_written);

    return Ok(());
}

fn log_packet_to_es(url: &str, json: &String)->Result<(), String>{
    let parsed_url = try!(ease::Url::parse(url).map_err(|e| e.to_string()));
    let mut req = ease::Request::new(parsed_url);
    req.body(json.clone());
    match req.post(){
        Ok(_) => {
            println!("Logged to ES");
            return Ok(());},
        Err(_) => {
            println!("ES POST FAILED");
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

fn process_packet(header: PacketHeader, msg: serial::CephMsgrMsg, output_args: &Args)->Result<(),String>{
    //Process OSD operation packets
    match msg.msg{
        //Client -> OSD operation
        serial::Message::OsdOp(osd_op) => {
            //Grab the current time to send along
            if output_args.carbon.is_some(){
                let now = time::now();
                let time_spec = now.to_timespec();
                let carbon_url = output_args.carbon.clone().unwrap();
                let (carbon_host, carbon_port) = try!(parse_carbon_url(&carbon_url));
                let graphite_data = format!("ceph.{}.{:?}.{} {}",
                    &header.src_v4addr.unwrap(),
                    &osd_op.flags,
                    &osd_op.operation.size,
                    time_spec.sec);
                try!(log_packet_to_carbon(&carbon_host, carbon_port, graphite_data));
            }

            if output_args.elasticsearch.is_some(){
                let milliseconds_since_epoch = get_time();
                let doc = Document{
                    header: header,
                    flags: osd_op.flags,
                    operation_count: osd_op.operation_count,
                    size: osd_op.operation.size,
                    timestamp: milliseconds_since_epoch,
                };
                let doc_json = try!(doc.to_json());
                //It's ok to unwrap here because we checked is_some() above
                try!(log_packet_to_es(&output_args.elasticsearch.clone().unwrap(), &doc_json));
            }
            return Ok(());
        },
        //Osd <-> Osd operation
        serial::Message::OsdSubop(sub_op) => {
            if output_args.carbon.is_some(){
                let now = time::now();
                let time_spec = now.to_timespec();
                let carbon_url = output_args.carbon.clone().unwrap();
                let (carbon_host, carbon_port) = try!(parse_carbon_url(&carbon_url));

                let graphite_data = format!("ceph.{}.{:?}.{} {}",
                    &header.src_v4addr.unwrap(),
                    &sub_op.flags,
                    &sub_op.operation.size,
                    time_spec.sec);
                try!(log_packet_to_carbon(&carbon_host, carbon_port, graphite_data));
            }

            if output_args.elasticsearch.is_some(){
                let milliseconds_since_epoch = get_time();
                let doc = Document{
                    header: header,
                    flags: sub_op.flags,
                    operation_count: sub_op.operation_count,
                    size: sub_op.operation.size,
                    timestamp: milliseconds_since_epoch,
                };
                let doc_json = try!(doc.to_json());
                //It's ok to unwrap here because we checked is_some() above
                try!(log_packet_to_es(&output_args.elasticsearch.clone().unwrap(), &doc_json));
            }
            return Ok(());
        },
        //TODO: Add more operation parsing results here
        _=> {
            return Ok(());
        }
    }
}

//MSGR is Ceph's outer message protocol
fn dissect_msgr<'a>(cursor: &mut Cursor<&'a [u8]>)->Result<serial::CephMsgrMsg, serial::SerialError>{
    let result = try!(serial::CephMsgrMsg::read_from_wire(cursor));
    return Ok(result);
}

fn main() {
    match check_user(){
        Ok(_) => {},
        Err(_) =>{
            return;
        }
    };
    let args = get_arguments();
    for output in &args.outputs {
        println!("Logging to {}", output);
    }

    let dev_list = match Device::list(){
        Ok(l) => l,
        Err(e) => {
            println!("Unable to list network devices.  Error: {}", e);
            return;
        }
    };

    println!("Validating network device");
    for dev_device in dev_list{
        if dev_device.name == "any"{
            println!("Found Network device");
            println!("Setting up capture");
            let mut cap = Capture::from_device(dev_device).unwrap() //open the device
                          .promisc(true)
                          //.snaplen(500)
                          .timeout(50)
                          .open() //activate the handle
                          .unwrap(); //assume activation worked
            println!("Setting up filter");
            //Grab both monitor and OSD traffic
            match cap.filter("tcp portrange 6789-7300"){
                Ok(_) => {
                    println!("Filter successful");
                },
                Err(e) => {
                    println!("Invalid capture filter. Error: {:?}", e);
                    return;
                }
            }
            println!("Waiting for packets");
            //Grab some packets :)
            loop {
                match cap.next(){
                    Some(packet) =>{
                        let data = packet.data;
                        let mut cursor = Cursor::new(&data[..]);
                        let packet_header = parse_etherframe(&mut cursor);
                        let result = dissect_msgr(&mut cursor);
                        if result.is_ok(){
                            if packet_header.is_ok(){
                                let p = packet_header.unwrap();
                                let print_result = process_packet(p, result.unwrap(), &args);
                            }
                        }
                    },
                    None => {},
                }
            }
        }
    }
}

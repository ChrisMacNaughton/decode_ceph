extern crate nom;
extern crate uuid;

use serial;

use self::nom::IResult::Done;
use self::uuid::{ParseError, Uuid};
use self::nom::{le_u8, le_u16, le_u32, le_i64, le_u64};
use serial::*;

#[test]
fn test_ceph_read_entity_name_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = EntityNameT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_EntityNameT() {
    //let bytes = vec![];
    //let result = EntityNameT::write_to_wire();
    //println!("ceph_write_EntityNameT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EntityNameT {
    pub _type: u8,
    pub _num: i64,
}

impl<'a> CephPrimitive<'a> for EntityNameT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
    		_type: le_u8 ~
    		_num: le_i64,
    		||{
    			EntityNameT{
    			_type: _type,
    			_num: _num,
    		}
    	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_ceph_sockaddr_storage() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = CephSockaddrStorage::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_CephSockaddrStorage() {
    //let bytes = vec![];
    //let result = CephSockaddrStorage::write_to_wire();
    //println!("ceph_write_CephSockaddrStorage{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct CephSockaddrStorage {
    pub ss_family: u16,
}

impl<'a> CephPrimitive<'a> for CephSockaddrStorage {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		ss_family: le_u16,
		||{
			CephSockaddrStorage{
			ss_family: ss_family,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_entity_inst_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = EntityInstT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_EntityInstT() {
    //let bytes = vec![];
    //let result = EntityInstT::write_to_wire();
    //println!("ceph_write_EntityInstT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EntityInstT {
    pub name: EntityNameT,
    pub addr: EntityAddr,
}

impl<'a> CephPrimitive<'a> for EntityInstT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		name: call!(EntityNameT::read_from_wire) ~
		addr: call!(EntityAddr::read_from_wire),
		||{
			EntityInstT{
			name: name,
			addr: addr,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_write_EversionT() {
    //let bytes = vec![];
    //let result = EversionT::write_to_wire();
    //println!("ceph_write_EversionT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EversionT {
    pub version: u64,
    pub epoch: u32,
}

impl<'a> CephPrimitive<'a> for EversionT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		version: le_u64 ~
		epoch: le_u32,
		||{
			EversionT{
			version: version,
			epoch: epoch,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

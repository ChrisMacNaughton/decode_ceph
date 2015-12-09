extern crate nom;
extern crate uuid;

use serial;

use self::uuid::{ParseError, Uuid};
use self::nom::{le_i8, le_u8, le_i16, le_u16, le_i32, le_u32, le_i64, le_u64, be_u16};
use serial::*;

#[test]
fn test_ceph_read_entity_name_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = EntityNameT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_EntityNameT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = EntityNameT::write_to_wire();
	println!("ceph_write_EntityNameT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EntityNameT{
	pub _type: u8,
	pub _num: i64,
}

impl<'a> CephPrimitive<'a> for EntityNameT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	let TYPE_MON = 1;
	let TYPE_MDS = 1;
	let TYPE_OSD = 1;
	let TYPE_CLIENT = 1;
	let NEW = 1;
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ceph_sockaddr_storage(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = CephSockaddrStorage::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_CephSockaddrStorage(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = CephSockaddrStorage::write_to_wire();
	println!("ceph_write_CephSockaddrStorage{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct CephSockaddrStorage{
	pub ss_family: u16,
}

impl<'a> CephPrimitive<'a> for CephSockaddrStorage{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		ss_family: le_u16,
		||{
			CephSockaddrStorage{
			ss_family: ss_family,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_entity_inst_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = EntityInstT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_EntityInstT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = EntityInstT::write_to_wire();
	println!("ceph_write_EntityInstT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EntityInstT{
	pub name: name,
	pub addr: addr,
}

impl<'a> CephPrimitive<'a> for EntityInstT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		name: EntityNameT ~
		addr: EntityAddr,
		||{
			EntityInstT{
			name: name,
			addr: addr,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_write_EversionT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = EversionT::write_to_wire();
	println!("ceph_write_EversionT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct EversionT{
	pub version: u64,
	pub epoch: u32,
	pub __pad: u32,
}

impl<'a> CephPrimitive<'a> for EversionT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		version: le_u64 ~
		epoch: le_u32 ~
		__pad: le_u32,
		||{
			EversionT{
			version: version,
			epoch: epoch,
			__pad: __pad,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

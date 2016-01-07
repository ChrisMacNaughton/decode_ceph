extern crate nom;
extern crate num;
extern crate uuid;

use serial;
use self::num::FromPrimitive;
use self::nom::{le_i8, le_u8, le_i16, le_u16, le_i32, le_u32, le_i64, le_u64, be_u16, be_f64};
use self::uuid::{ParseError, Uuid};
use self::nom::IResult::Done;
use serial::*;
use std::u16;
use common_decode::{EntityNameT, EntityInstT, EversionT};

enum_from_primitive!{
#[repr(u8)]
#[derive(Debug, Clone,Eq,PartialEq)]
pub enum OpTypeT{
    OP_SCRUB = 1,         // leader->peon: scrub (a range of) keys
    OP_RESULT = 2,        // peon->leader: result of a scrub
}
}

#[test]
fn test_ceph_read_MLog() {
    ////let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mlog::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    ////assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mlog() {
    //let bytes = vec![];
    //let result = Mlog::write_to_wire();
    //println!("ceph_write_Mlog{:?}", result);
    // assert_eq!(result, expected_bytes);
}

enum_from_primitive!{
#[repr(u16)]
#[derive(Debug, Clone,Eq,PartialEq)]
pub enum CLogType {
  CLOG_DEBUG = 0,
  CLOG_INFO = 1,
  CLOG_SEC = 2,
  CLOG_WARN = 3,
  CLOG_ERROR = 4,
  CLOG_UNKNOWN = u16::MAX,
}
}

#[derive(Debug,Eq,PartialEq)]
pub struct LogEntry<'a> {
    pub who: EntityInstT,
    pub stamp: Utime,
    pub seq: u64,
    pub prio: CLogType,
    pub msg: &'a str,
    pub channel: &'a str,
}
impl<'a> CephPrimitive<'a> for LogEntry<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          who: call!(EntityInstT::read_from_wire)~
          stamp: call!(Utime::read_from_wire) ~
          seq: le_u64 ~
          prio_bits: le_u16 ~
          prio: expr_opt!(CLogType::from_u16(prio_bits)) ~
          msg: parse_str ~
          channel: parse_str,
		||{
			LogEntry{
              who: who,
              stamp: stamp,
              seq: seq,
              prio: prio,
              msg: msg,
              channel: channel,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct Mlog<'a> {
    pub fsid: Uuid,
    pub entries: Vec<LogEntry<'a>>,
}

impl<'a> CephPrimitive<'a> for Mlog<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
        count: le_u32 ~
		entries: count!(LogEntry::read_from_wire, count as usize),
		||{
			Mlog{
			fsid: fsid,
			entries: entries,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonGetVersionReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmongetversionreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmongetversionreply() {
    //let bytes = vec![];
    //let result = Mmongetversionreply::write_to_wire();
    //println!("ceph_write_Mmongetversionreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmongetversionreply {
    pub handle: u64,
    pub version: u64,
    pub oldest_version: u64,
}

impl<'a> CephPrimitive<'a> for Mmongetversionreply {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		handle: le_u64 ~
		version: le_u64 ~
		oldest_version: le_u64,
		||{
			Mmongetversionreply{
			handle: handle,
			version: version,
			oldest_version: oldest_version,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_PaxosServiceMessage() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Paxosservicemessage::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Paxosservicemessage() {
    //let bytes = vec![];
    //let result = Paxosservicemessage::write_to_wire();
    //println!("ceph_write_Paxosservicemessage{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Paxosservicemessage {
    pub version: u64,
    pub deprecated_session_mon: i16,
    pub deprecated_session_mon_tid: u64,
    pub rx_election_epoch: u32,
}

impl<'a> CephPrimitive<'a> for Paxosservicemessage {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		version: le_u64 ~
		deprecated_session_mon: le_i16 ~
		deprecated_session_mon_tid: le_u64 ~
		rx_election_epoch: le_u32,
		||{
			Paxosservicemessage{
			version: version,
			deprecated_session_mon: deprecated_session_mon,
			deprecated_session_mon_tid: deprecated_session_mon_tid,
			rx_election_epoch: rx_election_epoch,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MCommand() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mcommand::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mcommand() {
    //let bytes = vec![];
    //let result = Mcommand::write_to_wire();
    //println!("ceph_write_Mcommand{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mcommand<'a> {
    pub fsid: Uuid,
    pub cmd: Vec<&'a str>,
}

impl<'a> CephPrimitive<'a> for Mcommand<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32~
		cmd: count!(parse_str, count as usize),
		||{
			Mcommand{
			fsid: fsid,
			cmd: cmd,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MForward() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mforward::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mforward() {
    //let bytes = vec![];
    //let result = Mforward::write_to_wire();
    //println!("ceph_write_Mforward{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct StringConstraint<'a> {
    pub value: &'a str,
    pub prefix: &'a str,
}

impl<'a> CephPrimitive<'a> for StringConstraint<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          value: parse_str ~
          prefix: parse_str,
		||{
			StringConstraint{
                value: value,
                prefix: prefix,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
bitflags!{
    flags MonCapFlags: u8 {
        const MON_CAP_R     = (1 << 1),      // read
        const MON_CAP_W     = (1 << 2),      // write
        const MON_CAP_X     = (1 << 3),      // execute
        const MON_CAP_ALL   =
            MON_CAP_R.bits |
            MON_CAP_W.bits |
            MON_CAP_X.bits,
        const MON_CAP_ANY   = 0xff,          // *
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct MonCapGrant<'a> {
    //
    // A grant can come in one of four forms:
    //
    //  - a blanket allow ('allow rw', 'allow *')
    //    - this will match against any service and the read/write/exec flags
    //      in the mon code.  semantics of what X means are somewhat ad hoc.
    //
    //  - a service allow ('allow service mds rw')
    //    - this will match against a specific service and the r/w/x flags.
    //
    //  - a profile ('allow profile osd')
    //    - this will match against specific monitor-enforced semantics of what
    //      this type of user should need to do.  examples include 'osd', 'mds',
    //      'bootstrap-osd'.
    //
    //  - a command ('allow command foo', 'allow command bar with arg1=val1 arg2 prefix val2')
    //      this includes the command name (the prefix string), and a set
    //      of key/value pairs that constrain use of that command.  if no pairs
    //      are specified, any arguments are allowed; if a pair is specified, that
    //      argument must be present and equal or match a prefix.
    //
    pub service: &'a str,
    pub profile: &'a str,
    pub command: &'a str,
    pub command_args: Vec<(&'a str, StringConstraint<'a>)>,
    pub flags: MonCapFlags,
}

impl<'a> CephPrimitive<'a> for MonCapGrant<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          service: parse_str ~
          profile: parse_str ~
          command: parse_str ~
          arg_length: le_u32 ~
          command_args: count!(
              pair!(parse_str, call!(StringConstraint::read_from_wire)),
              arg_length as usize) ~
        flag_bits: le_u8 ~
        flags: expr_opt!(MonCapFlags::from_bits(flag_bits)),
		||{
			MonCapGrant{
                service: service,
                profile: profile,
                command: command,
                command_args: command_args,
                flags: flags,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct MonCap<'a> {
    pub text: &'a str,
    pub grants: Vec<MonCapGrant<'a>>,
}

impl<'a> CephPrimitive<'a> for MonCap<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		text: parse_str ~
        grant_length: le_u32 ~
        grants: count!(call!(MonCapGrant::read_from_wire), grant_length as usize),
		||{
			MonCap{
                text: text,
                grants: grants,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct Mforward<'a> {
    pub tid: u64,
    pub client: EntityInstT,
    pub client_caps: MonCap<'a>,
    pub con_features: u64,
    pub entity_name: EntityNameT,
    pub msg: Paxosservicemessage,
    pub msg_bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Mforward<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		tid: le_u64 ~
		client: call!(EntityInstT::read_from_wire) ~
		client_caps: call!(MonCap::read_from_wire) ~
		con_features: le_u64 ~
		entity_name: call!(EntityNameT::read_from_wire) ~
		msg: call!(Paxosservicemessage::read_from_wire) ~
        msg_size: le_u32 ~
		msg_bl: take!(msg_size),
		||{
			Mforward{
			tid: tid,
			client: client,
			client_caps: client_caps,
			con_features: con_features,
			entity_name: entity_name,
			msg: msg,
			msg_bl: msg_bl,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonGlobalID() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonglobalid::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonglobalid() {
    //let bytes = vec![];
    //let result = Mmonglobalid::write_to_wire();
    //println!("ceph_write_Mmonglobalid{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonglobalid {
    pub old_max_id: u64,
}

impl<'a> CephPrimitive<'a> for Mmonglobalid {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		old_max_id: le_u64,
		||{
			Mmonglobalid{
			old_max_id: old_max_id,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonGetVersion() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmongetversion::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmongetversion() {
    //let bytes = vec![];
    //let result = Mmongetversion::write_to_wire();
    //println!("ceph_write_Mmongetversion{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmongetversion<'a> {
    pub handle: u64,
    pub what: &'a str,
}

impl<'a> CephPrimitive<'a> for Mmongetversion<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		handle: le_u64 ~
		what: parse_str,
		||{
			Mmongetversion{
			handle: handle,
			what: what,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonCommand() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmoncommand::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmoncommand() {
    //let bytes = vec![];
    //let result = Mmoncommand::write_to_wire();
    //println!("ceph_write_Mmoncommand{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmoncommand<'a> {
    pub fsid: Uuid,
    pub cmd: Vec<&'a str>,
}

impl<'a> CephPrimitive<'a> for Mmoncommand<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		cmd: count!(parse_str, count as usize),
		||{
			Mmoncommand{
			fsid: fsid,
			cmd: cmd,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonSubscribeAck() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonsubscribeack::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonsubscribeack() {
    //let bytes = vec![];
    //let result = Mmonsubscribeack::write_to_wire();
    //println!("ceph_write_Mmonsubscribeack{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonsubscribeack {
    pub interval: u32,
    pub fsid: Uuid,
}

impl<'a> CephPrimitive<'a> for Mmonsubscribeack {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		interval: le_u32 ~
		fsid: parse_fsid,
		||{
			Mmonsubscribeack{
			interval: interval,
			fsid: fsid,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonQuorumService() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonquorumservice::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonquorumservice() {
    //let bytes = vec![];
    //let result = Mmonquorumservice::write_to_wire();
    //println!("ceph_write_Mmonquorumservice{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonquorumservice {
    pub epoch: u32,
    pub round: u64,
}

impl<'a> CephPrimitive<'a> for Mmonquorumservice {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		epoch: le_u32 ~
		round: le_u64,
		||{
			Mmonquorumservice{
			epoch: epoch,
			round: round,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonScrub() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonscrub::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonscrub() {
    //let bytes = vec![];
    //let result = Mmonscrub::write_to_wire();
    //println!("ceph_write_Mmonscrub{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ScrubResult<'a> {
    pub prefix_crc: Vec<(&'a str, u32)>, // < prefix -> crc
    pub prefix_keys: Vec<(&'a str, u64)>, // < prefix -> key count
}

impl<'a> CephPrimitive<'a> for ScrubResult<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		num_crc: le_u32 ~
        prefix_crc: count!(pair!(parse_str, le_u32), num_crc as usize) ~
        num_keys: le_u32~
        prefix_keys: count!(pair!(parse_str, le_u64), num_keys as usize),
		||{
			ScrubResult{
                prefix_crc: prefix_crc,
                prefix_keys: prefix_keys,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonscrub<'a> {
    pub op: OpTypeT,
    pub version: u64,
    pub result: ScrubResult<'a>,
    pub num_keys: i32,
    pub key: (&'a str, &'a str),
}

impl<'a> CephPrimitive<'a> for Mmonscrub<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let HEAD_VERSION = 2;
        let COMPAT_VERSION = 1;
        chain!(input,
        op_bits: le_u8~
		op: expr_opt!(OpTypeT::from_u8(op_bits)) ~
		version: le_u64 ~
		result: call!(ScrubResult::read_from_wire) ~
		num_keys: le_i32 ~
		key: pair!(parse_str, parse_str),
		||{
			Mmonscrub{
			op: op,
			version: version,
			result: result,
			num_keys: num_keys,
			key: key,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonCommandAck() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmoncommandack::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmoncommandack() {
    //let bytes = vec![];
    //let result = Mmoncommandack::write_to_wire();
    //println!("ceph_write_Mmoncommandack{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmoncommandack<'a> {
    pub cmd: Vec<&'a str>,
    pub r: i32,
    pub rs: &'a str,
}

impl<'a> CephPrimitive<'a> for Mmoncommandack<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		cmd: count!(parse_str, count as usize)~
		r: le_i32 ~
		rs: parse_str,
		||{
			Mmoncommandack{
			cmd: cmd,
			r: r,
			rs: rs,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MMonMap() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonmap::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonmap() {
    //let bytes = vec![];
    //let result = Mmonmap::write_to_wire();
    //println!("ceph_write_Mmonmap{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonmap<'a> {
    pub monmapbl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Mmonmap<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        monmap_size: le_u32 ~
		monmapbl: take!(monmap_size),
		||{
			Mmonmap{
			monmapbl: monmapbl,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MAuthReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mauthreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mauthreply() {
    //let bytes = vec![];
    //let result = Mauthreply::write_to_wire();
    //println!("ceph_write_Mauthreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mauthreply<'a> {
    pub protocol: u32,
    pub result: i32,
    pub global_id: u64,
    pub result_msg: &'a str,
    pub result_bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Mauthreply<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		protocol: le_u32 ~
		result: le_i32 ~
		global_id: le_u64 ~
		result_msg: parse_str ~
        result_bl_size: le_u32 ~
		result_bl: take!(result_bl_size),
		||{
			Mauthreply{
			protocol: protocol,
			result: result,
			global_id: global_id,
			result_msg: result_msg,
			result_bl: result_bl,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MTimeCheck() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mtimecheck::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mtimecheck() {
    //let bytes = vec![];
    //let result = Mtimecheck::write_to_wire();
    //println!("ceph_write_Mtimecheck{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,PartialEq)]
pub struct Mtimecheck {
    pub op: i32,
    pub epoch: u64,
    pub round: u64,
    pub timestamp: Utime,
    pub skews: Vec<(EntityInstT, f64)>,
    pub latencies: Vec<(EntityInstT, f64)>,
}

impl<'a> CephPrimitive<'a> for Mtimecheck {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        chain!(input,
		op: le_i32 ~
		epoch: le_u64 ~
		round: le_u64 ~
		timestamp: call!(Utime::read_from_wire) ~
		count: le_u32 ~
		skews: count!(
            pair!(call!(EntityInstT::read_from_wire),be_f64), count as usize) ~
		count: le_u32 ~
		latencies: count!(
            pair!(call!(EntityInstT::read_from_wire),be_f64), count as usize),
		||{
			Mtimecheck{
			op: op,
			epoch: epoch,
			round: round,
			timestamp: timestamp,
			skews: skews,
			latencies: latencies,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonElection() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonelection::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonelection() {
    //let bytes = vec![];
    //let result = Mmonelection::write_to_wire();
    //println!("ceph_write_Mmonelection{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonelection<'a> {
    pub fsid: Uuid,
    pub op: i32,
    pub epoch: u32,
    pub monmap_bl: &'a [u8],
    pub quorum: Vec<i32>,
    pub quorum_features: u64,
    pub sharing_bl: &'a [u8],
    pub defunct_one: u64,
    pub defunct_two: u64,
}

impl<'a> CephPrimitive<'a> for Mmonelection<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let OP_PROPOSE = 1;
        let OP_ACK = 1;
        let OP_NAK = 1;
        let OP_VICTORY = 1;
        chain!(input,
		fsid: parse_fsid ~
		op: le_i32 ~
		epoch: le_u32 ~
        monmap_size: le_u32 ~
		monmap_bl: take!(monmap_size) ~
		count: le_u32 ~
		quorum: count!(le_i32,count as usize) ~
		quorum_features: le_u64 ~
        sharing_size: le_u32 ~
		sharing_bl: take!(sharing_size) ~
		defunct_one: le_u64 ~
		defunct_two: le_u64,
		||{
			Mmonelection{
			fsid: fsid,
			op: op,
			epoch: epoch,
			monmap_bl: monmap_bl,
			quorum: quorum,
			quorum_features: quorum_features,
			sharing_bl: sharing_bl,
			defunct_one: defunct_one,
			defunct_two: defunct_two,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MMonProbe() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonprobe::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonprobe() {
    //let bytes = vec![];
    //let result = Mmonprobe::write_to_wire();
    //println!("ceph_write_Mmonprobe{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonprobe<'a> {
    pub fsid: Uuid,
    pub op: i32,
    pub name: &'a str,
    pub quorum: Vec<i32>,
    pub monmap_bl: &'a [u8],
    pub paxos_first_version: u64,
    pub paxos_last_version: u64,
    pub has_ever_joined: u8,
    pub required_features: u64,
}

impl<'a> CephPrimitive<'a> for Mmonprobe<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		fsid: parse_fsid ~
		op: le_i32 ~
		name: parse_str ~
		count: le_u32 ~
		quorum: count!(le_i32,count as usize) ~
        monmap_size: le_u32 ~
		monmap_bl: take!(monmap_size) ~
		paxos_first_version: le_u64 ~
		paxos_last_version: le_u64 ~
		has_ever_joined: le_u8 ~
		required_features: le_u64,
		||{
			Mmonprobe{
			fsid: fsid,
			op: op,
			name: name,
			quorum: quorum,
			monmap_bl: monmap_bl,
			paxos_first_version: paxos_first_version,
			paxos_last_version: paxos_last_version,
			has_ever_joined: has_ever_joined,
			required_features: required_features,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonMetadata() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonmetadata::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonmetadata() {
    //let bytes = vec![];
    //let result = Mmonmetadata::write_to_wire();
    //println!("ceph_write_Mmonmetadata{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonmetadata<'a> {
    pub data: Vec<(&'a str, &'a str)>,
}

impl<'a> CephPrimitive<'a> for Mmonmetadata<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        data_size: le_u32 ~
		data: count!(pair!(parse_str, parse_str), data_size as usize),
		||{
			Mmonmetadata{
			data: data,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MMonJoin() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonjoin::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonjoin() {
    //let bytes = vec![];
    //let result = Mmonjoin::write_to_wire();
    //println!("ceph_write_Mmonjoin{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonjoin<'a> {
    pub fsid: Uuid,
    pub name: &'a str,
    pub addr: EntityAddr,
}

impl<'a> CephPrimitive<'a> for Mmonjoin<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		name: parse_str ~
		addr: call!(EntityAddr::read_from_wire),
		||{
			Mmonjoin{
			fsid: fsid,
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
fn test_ceph_read_MMonHealth() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonhealth::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonhealth() {
    //let bytes = vec![];
    //let result = Mmonhealth::write_to_wire();
    //println!("ceph_write_Mmonhealth{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ceph_data_stats {
    pub byte_total: u64,
    pub byte_used: u64,
    pub byte_avail: u64,
    pub avail_percent: i32,
}

impl<'a> CephPrimitive<'a> for ceph_data_stats {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          byte_total: le_u64 ~
          byte_used: le_u64 ~
          byte_avail: le_u64 ~
          avail_percent: le_i32,
		||{
			ceph_data_stats{
                byte_total: byte_total,
                byte_used: byte_used,
                byte_avail: byte_avail,
                avail_percent: avail_percent,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct DataStats {
    pub fs_stats: ceph_data_stats,
    // data dir
    pub last_update: Utime,
    pub store_stats: LevelDBStoreStats,
}
impl<'a> CephPrimitive<'a> for DataStats {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        fs_stats: call!(ceph_data_stats::read_from_wire) ~
        last_update: call!(Utime::read_from_wire) ~
        store_stats: call!(LevelDBStoreStats::read_from_wire),
		||{
			DataStats{
                fs_stats: fs_stats,
                last_update: last_update,
                store_stats: store_stats,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct LevelDBStoreStats {
    pub bytes_total: u64,
    pub bytes_sst: u64,
    pub bytes_log: u64,
    pub bytes_misc: u64,
    pub last_update: Utime,
}

impl<'a> CephPrimitive<'a> for LevelDBStoreStats {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          bytes_total: le_u64 ~
          bytes_sst: le_u64 ~
          bytes_log: le_u64 ~
          bytes_misc: le_u64 ~
          last_update: call!(Utime::read_from_wire),
		||{
			LevelDBStoreStats{
              bytes_total: bytes_total,
              bytes_sst: bytes_sst,
              bytes_log: bytes_log,
              bytes_misc: bytes_misc,
              last_update: last_update,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}


#[derive(Debug,Eq,PartialEq)]
pub struct Mmonhealth {
    pub service_type: i32,
    pub service_op: i32,
    pub data_stats: DataStats,
}

impl<'a> CephPrimitive<'a> for Mmonhealth {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        chain!(input,
		service_type: le_i32 ~
		service_op: le_i32 ~
		data_stats: call!(DataStats::read_from_wire),
		||{
			Mmonhealth{
			service_type: service_type,
			service_op: service_op,
			data_stats: data_stats,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MMonSync() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonsync::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonsync() {
    //let bytes = vec![];
    //let result = Mmonsync::write_to_wire();
    //println!("ceph_write_Mmonsync{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonsync<'a> {
    pub op: u32,
    pub cookie: u64,
    pub last_committed: u64,
    pub last_key: (&'a str, &'a str),
    pub chunk_bl: &'a [u8],
    pub reply_to: EntityInstT,
}

impl<'a> CephPrimitive<'a> for Mmonsync<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		op: le_u32 ~
		cookie: le_u64 ~
		last_committed: le_u64 ~
		last_key: pair!(parse_str,parse_str) ~
        chunk_size: le_u32 ~
		chunk_bl: take!(chunk_size) ~
		reply_to: call!(EntityInstT::read_from_wire),
		||{
			Mmonsync{
			op: op,
			cookie: cookie,
			last_committed: last_committed,
			last_key: last_key,
			chunk_bl: chunk_bl,
			reply_to: reply_to,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MMonPaxos() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mmonpaxos::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mmonpaxos() {
    //let bytes = vec![];
    //let result = Mmonpaxos::write_to_wire();
    //println!("ceph_write_Mmonpaxos{:?}", result);
    // assert_eq!(result, expected_bytes);
}
enum_from_primitive!{
#[repr(i32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaxosOperation {
     OP_COLLECT =   1, // proposer: propose round
     OP_LAST =      2, // voter:    accept proposed round
     OP_BEGIN =     3, // proposer: value proposed for this round
     OP_ACCEPT =    4, // voter:    accept propsed value
     OP_COMMIT =    5, // proposer: notify learners of agreed value
     OP_LEASE =     6, // leader: extend peon lease
     OP_LEASE_ACK = 7, // peon: lease ack
}
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mmonpaxos<'a> {
    pub epoch: u32,
    pub op: PaxosOperation,
    pub first_committed: u64, // i've committed to
    pub last_committed: u64, // i've committed to
    pub pn_from: u64, // i promise to accept after
    pub pn: u64, // with with proposal
    pub uncommitted_pn: u64, // previous pn, if we are a LAST with an uncommitted value
    pub lease_timestamp: Utime,
    pub sent_timestamp: Utime,
    pub latest_version: u64,
    pub latest_value: &'a [u8],
    pub values: Vec<&'a str>,
}

impl<'a> CephPrimitive<'a> for Mmonpaxos<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let HEAD_VERSION = 3;
        let COMPAT_VERSION = 3;
        chain!(input,
		epoch: le_u32 ~
		op_bits: le_i32 ~
        op: expr_opt!(PaxosOperation::from_i32(op_bits)) ~
		first_committed: le_u64 ~
		last_committed: le_u64 ~
		pn_from: le_u64 ~
		pn: le_u64 ~
		uncommitted_pn: le_u64 ~
		lease_timestamp: call!(Utime::read_from_wire) ~
		sent_timestamp: call!(Utime::read_from_wire) ~
		latest_version: le_u64 ~
        latest_value_size: le_u32 ~
		latest_value: take!(latest_value_size) ~
		count: le_u32 ~
		count!(
            pair!(le_u64,
                take!(10)), count as usize),
		||{
			Mmonpaxos{
			epoch: epoch,
			op: op,
			first_committed: first_committed,
			last_committed: last_committed,
			pn_from: pn_from,
			pn: pn,
			uncommitted_pn: uncommitted_pn,
			lease_timestamp: lease_timestamp,
			sent_timestamp: sent_timestamp,
			latest_version: latest_version,
			latest_value: latest_value,
			values: vec![],
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
//
// #[test]
// fn test_ceph_read_MClientQuota(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Mclientquota::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mclientquota(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = Mclientquota::write_to_wire();
// //println!("ceph_write_Mclientquota{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mclientquota{
// pub ino: ino,
// pub rstat: rstat,
// pub quota: quota,
// }
//
// impl<'a> CephPrimitive<'a> for Mclientquota{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// ino: call!(inodeno_t::read_from_wire) ~
// rstat: call!(nest_info_t::read_from_wire) ~
// quota: call!(quota_info_t::read_from_wire),
// ||{
// Mclientquota{
// ino: ino,
// rstat: rstat,
// quota: quota,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//
#[test]
fn test_ceph_read_MAuth() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mauth::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mauth() {
    //let bytes = vec![];
    //let result = Mauth::write_to_wire();
    //println!("ceph_write_Mauth{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mauth<'a> {
    pub protocol: u32,
    pub auth_payload: &'a [u8],
    pub monmap_epoch: u32,
}

impl<'a> CephPrimitive<'a> for Mauth<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		protocol: le_u32 ~
        auth_size: le_u32 ~
		auth_payload: take!(auth_size) ~
		monmap_epoch: le_u32,
		||{
			Mauth{
			protocol: protocol,
			auth_payload: auth_payload,
			monmap_epoch: monmap_epoch,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MLogAck() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    // let expected_result = Mlogack {
    // };
    //let result = Mlogack::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    ////assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mlogack() {
    //let bytes = vec![];
    //let result = Mlogack::write_to_wire();
    //println!("ceph_write_Mlogack{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mlogack<'a> {
    pub fsid: Uuid,
    pub last: u64,
    pub channel: &'a str,
}

impl<'a> CephPrimitive<'a> for Mlogack<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		last: le_u64 ~
		channel: parse_str,
		||{
			Mlogack{
			fsid: fsid,
			last: last,
			channel: channel,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_ceph_mon_subscribe_item_old() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = CephMonSubscribeItemOld::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_CephMonSubscribeItemOld() {
    //let bytes = vec![];
    //let result = CephMonSubscribeItemOld::write_to_wire();
    //println!("ceph_write_CephMonSubscribeItemOld{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct CephMonSubscribeItemOld {
    pub unused: u64,
    pub have: u64,
    pub onetime: u8,
}

impl<'a> CephPrimitive<'a> for CephMonSubscribeItemOld {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		unused: le_u64 ~
		have: le_u64 ~
		onetime: le_u8,
		||{
			CephMonSubscribeItemOld{
			unused: unused,
			have: have,
			onetime: onetime,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MRoute() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    // let expected_result = Mroute {
    // };
    //let result = Mroute::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    ////assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mroute() {
    //let bytes = vec![];
    //let result = Mroute::write_to_wire();
    //println!("ceph_write_Mroute{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mroute<'a> {
    pub session_mon_tid: u64,
    pub msg: &'a str,
    pub dest: EntityInstT,
}

impl<'a> CephPrimitive<'a> for Mroute<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		session_mon_tid: le_u64 ~
		msg: parse_str ~
		dest: call!(EntityInstT::read_from_wire),
		||{
			Mroute{
			session_mon_tid: session_mon_tid,
			msg: msg,
			dest: dest,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

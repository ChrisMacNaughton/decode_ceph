extern crate nom;
extern crate uuid;

use self::nom::IResult::Done;
use self::uuid::{ParseError, Uuid};
use self::nom::{le_i8, le_u8, le_i16, le_u16, le_i32, le_u32, le_i64, le_u64, be_u16};
use std::collections::HashMap;
use serial::*;
use common_decode::{EntityNameT, EntityInstT, EversionT};

//Return Some or None no matter what.
macro_rules! hard_opt(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      match $submac!($i, $($args)*) {
        nom::IResult::Done(i,o)     => nom::IResult::Done(i, Some(o)),
        nom::IResult::Error(_)      => nom::IResult::Done($i, None),
        nom::IResult::Incomplete(i) => nom::IResult::Done($i, None),
      }
    }
  );
  ($i:expr, $f:expr) => (
    opt!($i, call!($f));
  );
);

#[test]
fn test_ceph_read_OsdReqidT() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = OsdReqidT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_OsdReqidT() {
    //let bytes = vec![];
    //let result = OsdReqidT::write_to_wire();
    //println!("ceph_write_OsdReqidT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct OsdReqidT {
    pub name: EntityNameT,
    pub tid: u64,
    pub inc: i32,
}

impl<'a> CephPrimitive<'a> for OsdReqidT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        take!(12)~ //TODO Why do I need this?
		name: call!(EntityNameT::read_from_wire) ~
		tid: le_u64 ~
		inc: le_i32,
		||{
			OsdReqidT{
			name: name,
			tid: tid,
			inc: inc,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pool_stat_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PoolStatT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PoolStatT() {
    //let bytes = vec![];
    //let result = PoolStatT::write_to_wire();
    //println!("ceph_write_PoolStatT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PoolStatT {
    pub stats: ObjectStatCollectionT,
    pub log_size: i64,
    pub ondisk_log_size: i64,
    pub up: i32,
    pub acting: i32,
}

impl<'a> CephPrimitive<'a> for PoolStatT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		stats: call!(ObjectStatCollectionT::read_from_wire) ~
		log_size: le_i64 ~
		ondisk_log_size: le_i64 ~
		up: le_i32 ~
		acting: le_i32,
		||{
			PoolStatT{
			stats: stats,
			log_size: log_size,
			ondisk_log_size: ondisk_log_size,
			up: up,
			acting: acting,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct pool_snap_info_t<'a> {
    pub snapid_t: u64,
    pub utime_t: Utime,
    pub name: &'a str,
}

impl<'a> CephPrimitive<'a> for pool_snap_info_t<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        snapid_t: le_u64 ~
        utime: call!(Utime::read_from_wire) ~
        name: parse_str,
        ||{
            pool_snap_info_t{
                snapid_t: snapid_t,
                utime_t: utime,
                name: name,
            }
        }
    )
    }

    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

//
// #[test]
// fn test_ceph_read_pg_pool_t(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = PgPoolT::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_PgPoolT(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = PgPoolT::write_to_wire();
// //println!("ceph_write_PgPoolT{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct PgPoolT{
// pub flags: u64,
// pub pool_type: u8,
// pub size: u8,
// pub min_size: u8,
// pub crush_ruleset: u8,
// pub object_hash: u8,
// pub properties: properties,
// pub erasure_code_profile: &'a str,
// pub last_change: u32,
// pub last_force_op_resend: u32,
// pub snap_seq: snap_seq,
// pub snap_epoch: u32,
// pub auid: u64,
// pub crash_replay_interval: u32,
// pub quota_max_bytes: u64,
// pub quota_max_objects: u64,
// pub snaps: snaps,
// pub removed_snaps: removed_snaps,
// pub pg_num_mask: u32,
// pub pgp_num_mask: u32,
// pub tiers: tiers,
// pub tier_of: i64,
// pub read_tier: i64,
// pub write_tier: i64,
// pub cache_mode: cache_mode,
// pub target_max_bytes: u64,
// pub target_max_objects: u64,
// pub cache_target_dirty_ratio_micro: u32,
// pub cache_target_dirty_high_ratio_micro: u32,
// pub cache_target_full_ratio_micro: u32,
// pub cache_min_flush_age: u32,
// pub cache_min_evict_age: u32,
// pub hit_set_params: hit_set_params,
// pub hit_set_period: u32,
// pub hit_set_count: u32,
// pub use_gmt_hitset: u8,
// pub min_read_recency_for_promote: u32,
// pub min_write_recency_for_promote: u32,
// pub hit_set_grade_decay_rate: u32,
// pub hit_set_search_last_n: u32,
// pub stripe_width: u32,
// pub expected_num_objects: u64,
// pub fast_read: u8,
// }
//
// impl<'a> CephPrimitive<'a> for PgPoolT{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// flags: le_u64 ~
// pool_type: le_u8 ~
// size: le_u8 ~
// min_size: le_u8 ~
// crush_ruleset: le_u8 ~
// object_hash: le_u8 ~
// count_1: le_u32 ~
// properties: count!(pair!(parse_str,parse_str), count_1) ~
// erasure_code_profile: parse_str ~
// last_change: le_u32 ~
// last_force_op_resend: le_u32 ~
// snap_seq: le_u64 ~
// snap_epoch: le_u32 ~
// auid: le_u64 ~
// crash_replay_interval: le_u32 ~
// quota_max_bytes: le_u64 ~
// quota_max_objects: le_u64 ~
// count_2: le_u32 ~
// snaps: count!(
// pair!(le_u64,
// call!(pool_snap_info_t::read_from_wire)), count_2) ~
// removed_snaps: le_u64 ~
// pg_num_mask: le_u32 ~
// pgp_num_mask: le_u32 ~
// count_3: le_u32 ~
// tiers: count!(le_u64,count_3)~
// tier_of: le_i64 ~
// read_tier: le_i64 ~
// write_tier: le_i64 ~
// cache_mode: call!(cache_mode_t::read_from_wire) ~
// target_max_bytes: le_u64 ~
// target_max_objects: le_u64 ~
// cache_target_dirty_ratio_micro: le_u32 ~
// cache_target_dirty_high_ratio_micro: le_u32 ~
// cache_target_full_ratio_micro: le_u32 ~
// cache_min_flush_age: le_u32 ~
// cache_min_evict_age: le_u32 ~
// hit_set_params: call!(HitSetParams::read_from_wire) ~
// hit_set_period: le_u32 ~
// hit_set_count: le_u32 ~
// use_gmt_hitset: le_u8 ~
// min_read_recency_for_promote: le_u32 ~
// min_write_recency_for_promote: le_u32 ~
// hit_set_grade_decay_rate: le_u32 ~
// hit_set_search_last_n: le_u32 ~
// stripe_width: le_u32 ~
// expected_num_objects: le_u64 ~
// fast_read: le_u8,
// ||{
// PgPoolT{
// flags: flags,
// pool_type: pool_type,
// size: size,
// min_size: min_size,
// crush_ruleset: crush_ruleset,
// object_hash: object_hash,
// properties: properties,
// erasure_code_profile: erasure_code_profile,
// last_change: last_change,
// last_force_op_resend: last_force_op_resend,
// snap_seq: snap_seq,
// snap_epoch: snap_epoch,
// auid: auid,
// crash_replay_interval: crash_replay_interval,
// quota_max_bytes: quota_max_bytes,
// quota_max_objects: quota_max_objects,
// snaps: snaps,
// removed_snaps: removed_snaps,
// pg_num_mask: pg_num_mask,
// pgp_num_mask: pgp_num_mask,
// tiers: tiers,
// tier_of: tier_of,
// read_tier: read_tier,
// write_tier: write_tier,
// cache_mode: cache_mode,
// target_max_bytes: target_max_bytes,
// target_max_objects: target_max_objects,
// cache_target_dirty_ratio_micro: cache_target_dirty_ratio_micro,
// cache_target_dirty_high_ratio_micro: cache_target_dirty_high_ratio_micro,
// cache_target_full_ratio_micro: cache_target_full_ratio_micro,
// cache_min_flush_age: cache_min_flush_age,
// cache_min_evict_age: cache_min_evict_age,
// hit_set_params: hit_set_params,
// hit_set_period: hit_set_period,
// hit_set_count: hit_set_count,
// use_gmt_hitset: use_gmt_hitset,
// min_read_recency_for_promote: min_read_recency_for_promote,
// min_write_recency_for_promote: min_write_recency_for_promote,
// hit_set_grade_decay_rate: hit_set_grade_decay_rate,
// hit_set_search_last_n: hit_set_search_last_n,
// stripe_width: stripe_width,
// expected_num_objects: expected_num_objects,
// fast_read: fast_read,
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
fn test_ceph_read_pg_shard_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgShardT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgShardT() {
    //let bytes = vec![];
    //let result = PgShardT::write_to_wire();
    //println!("ceph_write_PgShardT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgShardT {
    pub osd: i32,
    pub shard: i8,
}

impl<'a> CephPrimitive<'a> for PgShardT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		osd: le_i32 ~
		shard: le_i8,
		||{
			PgShardT{
			osd: osd,
			shard: shard,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_SnapSetContext() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Snapsetcontext::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Snapsetcontext() {
    //let bytes = vec![];
    //let result = Snapsetcontext::write_to_wire();
    //println!("ceph_write_Snapsetcontext{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Snapsetcontext<'a> {
    pub oid: HObject<'a>,
    pub snap_ref: i32,
    pub registered: u8,
    pub snapset: SnapSet,
    pub exists: u8,
}

impl<'a> CephPrimitive<'a> for Snapsetcontext<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		oid: call!(HObject::read_from_wire) ~
		snap_ref: le_i32 ~
		registered: le_u8 ~
		snapset: call!(SnapSet::read_from_wire) ~
		exists: le_u8,
		||{
			Snapsetcontext{
			oid: oid,
			snap_ref: snap_ref,
			registered: registered,
			snapset: snapset,
			exists: exists,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pg_query_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgQueryT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgQueryT() {
    //let bytes = vec![];
    //let result = PgQueryT::write_to_wire();
    //println!("ceph_write_PgQueryT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgQueryT {
    pub query_type: u32,
    pub since: EversionT,
    pub history: PgHistory,
    pub epoch_sent: u32,
    pub to: i8,
    pub from: i8,
}

impl<'a> CephPrimitive<'a> for PgQueryT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		query_type: le_u32 ~
		since: call!(EversionT::read_from_wire) ~
		history: call!(PgHistory::read_from_wire) ~
		epoch_sent: le_u32 ~
		to: le_i8 ~
		from: le_i8,
		||{
			PgQueryT{
			query_type: query_type,
			since: since,
			history: history,
			epoch_sent: epoch_sent,
			to: to,
			from: from,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_pg_notify_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgNotifyT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgNotifyT() {
    //let bytes = vec![];
    //let result = PgNotifyT::write_to_wire();
    //println!("ceph_write_PgNotifyT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgNotifyT<'a> {
    pub query_epoch: u32,
    pub epoch_sent: u32,
    pub info: PgInfoT<'a>,
    pub to: i8,
    pub from: i8,
}

impl<'a> CephPrimitive<'a> for PgNotifyT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		query_epoch: le_u32 ~
		epoch_sent: le_u32 ~
		info: call!(PgInfoT::read_from_wire) ~
		to: le_i8 ~
		from: le_i8,
		||{
			PgNotifyT{
			query_epoch: query_epoch,
			epoch_sent: epoch_sent,
			info: info,
			to: to,
			from: from,
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
// fn test_ceph_read_OSDSuperblock(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Osdsuperblock::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Osdsuperblock(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = Osdsuperblock::write_to_wire();
// //println!("ceph_write_Osdsuperblock{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Osdsuperblock{
// pub cluster_fsid: Uuid,
// pub osd_fsid: Uuid,
// pub whoami: i32,
// pub current_epoch: u32,
// pub oldest_map: u32,
// pub newest_map: u32,
// pub weight: f64,
// TODO: Decode CompatSet
// pub compat_features: CompatSet,
// pub mounted: u32,
// pub clean_thru: u32,
// }
//
// impl<'a> CephPrimitive<'a> for Osdsuperblock{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// cluster_fsid: parse_fsid ~
// osd_fsid: parse_fsid ~
// whoami: le_i32 ~
// current_epoch: le_u32 ~
// oldest_map: le_u32 ~
// newest_map: le_u32 ~
// weight: le_f64 ~
// compat_features: call!(CompatSet::read_from_wire) ~
// mounted: le_u32 ~
// clean_thru: le_u32,
// ||{
// Osdsuperblock{
// cluster_fsid: cluster_fsid,
// osd_fsid: osd_fsid,
// whoami: whoami,
// current_epoch: current_epoch,
// oldest_map: oldest_map,
// newest_map: newest_map,
// weight: weight,
// compat_features: compat_features,
// mounted: mounted,
// clean_thru: clean_thru,
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
fn test_ceph_read_pg_interval_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgIntervalT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgIntervalT() {
    //let bytes = vec![];
    //let result = PgIntervalT::write_to_wire();
    //println!("ceph_write_PgIntervalT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgIntervalT {
    pub up: Vec<i32>,
    pub acting: Vec<i32>,
    pub first: u32,
    pub last: u32,
    pub maybe_went_rw: u8,
    pub primary: i32,
    pub up_primary: i32,
}

impl<'a> CephPrimitive<'a> for PgIntervalT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		up: count!(le_i32, count as usize)~
		count: le_u32 ~
		acting: count!(le_i32, count as usize)~
		first: le_u32 ~
		last: le_u32 ~
		maybe_went_rw: le_u8 ~
		primary: le_i32 ~
		up_primary: le_i32,
		||{
			PgIntervalT{
			up: up,
			acting: acting,
			first: first,
			last: last,
			maybe_went_rw: maybe_went_rw,
			primary: primary,
			up_primary: up_primary,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_PushReplyOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Pushreplyop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pushreplyop() {
    //let bytes = vec![];
    //let result = Pushreplyop::write_to_wire();
    //println!("ceph_write_Pushreplyop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pushreplyop<'a> {
    pub soid: HObject<'a>,
}

impl<'a> CephPrimitive<'a> for Pushreplyop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		soid: call!(HObject::read_from_wire),
		||{
			Pushreplyop{
			soid: soid,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_ScrubMap() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Scrubmap::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Scrubmap() {
    //let bytes = vec![];
    //let result = Scrubmap::write_to_wire();
    //println!("ceph_write_Scrubmap{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Scrubmap {
    pub valid_through: EversionT,
    pub incr_since: EversionT,
}

impl<'a> CephPrimitive<'a> for Scrubmap {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		valid_through: call!(EversionT::read_from_wire) ~
		incr_since: call!(EversionT::read_from_wire),
		||{
			Scrubmap{
			valid_through: valid_through,
			incr_since: incr_since,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_obj_list_watch_response_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjListWatchResponseT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjListWatchResponseT() {
    //let bytes = vec![];
    //let result = ObjListWatchResponseT::write_to_wire();
    //println!("ceph_write_ObjListWatchResponseT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjListWatchResponseT {
    pub entries: Vec<WatchItemT>,
}

impl<'a> CephPrimitive<'a> for ObjListWatchResponseT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
	count: le_u32 ~
	entries: count!(call!(WatchItemT::read_from_wire),count as usize),
		||{
			ObjListWatchResponseT{
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
fn test_ceph_read_object_stat_sum_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjectStatSumT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectStatSumT() {
    //let bytes = vec![];
    //let result = ObjectStatSumT::write_to_wire();
    //println!("ceph_write_ObjectStatSumT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectStatSumT {
    pub version: u8,
    pub compat: u8,
    pub struct_len: u32,
    pub num_bytes: i64,
    pub num_objects: i64,
    pub num_object_clones: i64,
    pub num_object_copies: i64,
    pub num_objects_missing_on_primary: i64,
    pub num_objects_degraded: i64,
    pub num_objects_misplaced: i64,
    pub num_objects_unfound: i64,
    pub num_rd: i64,
    pub num_rd_kb: i64,
    pub num_wr: i64,
    pub num_wr_kb: i64,
    pub num_scrub_errors: i64,
    pub num_shallow_scrub_errors: i64,
    pub num_deep_scrub_errors: i64,
    pub num_objects_recovered: i64,
    pub num_bytes_recovered: i64,
    pub num_keys_recovered: i64,
    pub num_objects_dirty: i64,
    pub num_whiteouts: i64,
    pub num_objects_omap: i64,
    pub num_objects_hit_set_archive: i64,
    pub num_bytes_hit_set_archive: i64,
    pub num_flush: i64,
    pub num_flush_kb: i64,
    pub num_evict: i64,
    pub num_evict_kb: i64,
    pub num_promote: i64,
    pub num_flush_mode_high: i32,
    pub num_flush_mode_low: i32,
    pub num_evict_mode_some: i32,
    pub num_evict_mode_full: i32,
    pub num_objects_pinned: i64,
}

impl<'a> CephPrimitive<'a> for ObjectStatSumT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        version: le_u8~
        compat: le_u8~
        struct_len: le_u32~
		num_bytes: le_i64 ~
		num_objects: le_i64 ~
		num_object_clones: le_i64 ~
		num_object_copies: le_i64 ~
		num_objects_missing_on_primary: le_i64 ~
		num_objects_degraded: le_i64 ~
		num_objects_misplaced: le_i64 ~
		num_objects_unfound: le_i64 ~
		num_rd: le_i64 ~
		num_rd_kb: le_i64 ~
		num_wr: le_i64 ~
		num_wr_kb: le_i64 ~
		num_scrub_errors: le_i64 ~
		num_shallow_scrub_errors: le_i64 ~
		num_deep_scrub_errors: le_i64 ~
		num_objects_recovered: le_i64 ~
		num_bytes_recovered: le_i64 ~
		num_keys_recovered: le_i64 ~
		num_objects_dirty: le_i64 ~
		num_whiteouts: le_i64 ~
		num_objects_omap: le_i64 ~
		num_objects_hit_set_archive: le_i64 ~
		num_bytes_hit_set_archive: le_i64 ~
		num_flush: le_i64 ~
		num_flush_kb: le_i64 ~
		num_evict: le_i64 ~
		num_evict_kb: le_i64 ~
		num_promote: le_i64 ~
		num_flush_mode_high: le_i32 ~
		num_flush_mode_low: le_i32 ~
		num_evict_mode_some: le_i32 ~
		num_evict_mode_full: le_i32 ~
		num_objects_pinned: le_i64,
		||{
			ObjectStatSumT{
                version: version,
                compat:compat,
                struct_len: struct_len,
    			num_bytes: num_bytes,
    			num_objects: num_objects,
    			num_object_clones: num_object_clones,
    			num_object_copies: num_object_copies,
    			num_objects_missing_on_primary: num_objects_missing_on_primary,
    			num_objects_degraded: num_objects_degraded,
    			num_objects_misplaced: num_objects_misplaced,
    			num_objects_unfound: num_objects_unfound,
    			num_rd: num_rd,
    			num_rd_kb: num_rd_kb,
    			num_wr: num_wr,
    			num_wr_kb: num_wr_kb,
    			num_scrub_errors: num_scrub_errors,
    			num_shallow_scrub_errors: num_shallow_scrub_errors,
    			num_deep_scrub_errors: num_deep_scrub_errors,
    			num_objects_recovered: num_objects_recovered,
    			num_bytes_recovered: num_bytes_recovered,
    			num_keys_recovered: num_keys_recovered,
    			num_objects_dirty: num_objects_dirty,
    			num_whiteouts: num_whiteouts,
    			num_objects_omap: num_objects_omap,
    			num_objects_hit_set_archive: num_objects_hit_set_archive,
    			num_bytes_hit_set_archive: num_bytes_hit_set_archive,
    			num_flush: num_flush,
    			num_flush_kb: num_flush_kb,
    			num_evict: num_evict,
    			num_evict_kb: num_evict_kb,
    			num_promote: num_promote,
    			num_flush_mode_high: num_flush_mode_high,
    			num_flush_mode_low: num_flush_mode_low,
    			num_evict_mode_some: num_evict_mode_some,
    			num_evict_mode_full: num_evict_mode_full,
    			num_objects_pinned: num_objects_pinned,
    		}
    	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_object_locator_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjectLocatorT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectLocatorT() {
    //let bytes = vec![];
    //let result = ObjectLocatorT::write_to_wire();
    //println!("ceph_write_ObjectLocatorT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectLocatorT<'a> {
    pub pool: i64,
    pub key: &'a str,
    pub nspace: &'a str,
    pub hash: i64,
}

impl<'a> CephPrimitive<'a> for ObjectLocatorT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		pool: le_i64 ~
		key: parse_str ~
		nspace: parse_str ~
		hash: le_i64,
		||{
			ObjectLocatorT{
			pool: pool,
			key: key,
			nspace: nspace,
			hash: hash,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pg_log_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgLogT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgLogT() {
    //let bytes = vec![];
    //let result = PgLogT::write_to_wire();
    //println!("ceph_write_PgLogT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgLogT<'a> {
    pub head: EversionT,
    pub tail: EversionT,
    pub can_rollback_to: EversionT,
    pub rollback_info_trimmed_to: EversionT,
    pub log: Vec<pg_log_entry_t<'a>>,
}

impl<'a> CephPrimitive<'a> for PgLogT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		head: call!(EversionT::read_from_wire) ~
		tail: call!(EversionT::read_from_wire) ~
		can_rollback_to: call!(EversionT::read_from_wire) ~
		rollback_info_trimmed_to: call!(EversionT::read_from_wire) ~
		count: le_u32 ~
		log: count!(call!(pg_log_entry_t::read_from_wire),count as usize),
		||{
			PgLogT{
			head: head,
			tail: tail,
			can_rollback_to: can_rollback_to,
			rollback_info_trimmed_to: rollback_info_trimmed_to,
			log: log,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_hit_set_info_t {
    pub begin: Utime, // < time interval
    pub end: Utime, // < time interval
    pub version: EversionT, // < version this HitSet object was written
    pub using_gmt: u8, // < use gmt for creating the hit_set archive object name
}
impl<'a> CephPrimitive<'a> for pg_hit_set_info_t {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          begin: call!(Utime::read_from_wire) ~
          end: call!(Utime::read_from_wire) ~
          version: call!(EversionT::read_from_wire) ~
          using_gmt: le_u8,
		||{
			pg_hit_set_info_t{
                begin: begin,
                end: end,
                version: version,
                using_gmt: using_gmt,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_hit_set_history_t {
    pub current_last_update: EversionT, // < last version inserted into current set
    pub history: Vec<pg_hit_set_info_t>, // < archived sets, sorted oldest -> newest
}

impl<'a> CephPrimitive<'a> for pg_hit_set_history_t {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          current_last_update: call!(EversionT::read_from_wire) ~
          hit_count: le_u32 ~
          history: count!(pg_hit_set_info_t::read_from_wire, hit_count as usize),
		||{
			pg_hit_set_history_t{
              current_last_update: current_last_update,
              history: history,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_pg_info_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgInfoT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgInfoT() {
    //let bytes = vec![];
    //let result = PgInfoT::write_to_wire();
    //println!("ceph_write_PgInfoT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgInfoT<'a> {
    pub pgid: SpgT,
    pub last_update: EversionT,
    pub last_complete: EversionT,
    pub last_epoch_started: u32,
    pub last_user_version: u64,
    pub log_tail: EversionT,
    pub last_backfill: HObject<'a>,
    pub last_backfill_bitwise: u8,
    pub purged_snaps: Vec<u64>,
    pub stats: PgStatT,
    pub history: PgHistory,
    pub hit_set: pg_hit_set_history_t,
}

impl<'a> CephPrimitive<'a> for PgInfoT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		last_update: call!(EversionT::read_from_wire) ~
		last_complete: call!(EversionT::read_from_wire) ~
		last_epoch_started: le_u32 ~
		last_user_version: le_u64 ~
		log_tail: call!(EversionT::read_from_wire) ~
		last_backfill: call!(HObject::read_from_wire) ~
		last_backfill_bitwise: le_u8 ~
        purged_count: le_u32 ~
		purged_snaps: count!(le_u64, purged_count as usize) ~
		stats: call!(PgStatT::read_from_wire) ~
		history: call!(PgHistory::read_from_wire) ~
		hit_set: call!(pg_hit_set_history_t::read_from_wire),
		||{
			PgInfoT{
			pgid: pgid,
			last_update: last_update,
			last_complete: last_complete,
			last_epoch_started: last_epoch_started,
			last_user_version: last_user_version,
			log_tail: log_tail,
			last_backfill: last_backfill,
			last_backfill_bitwise: last_backfill_bitwise,
			purged_snaps: purged_snaps,
			stats: stats,
			history: history,
			hit_set: hit_set,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pg_missing_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgMissingT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgMissingT() {
    //let bytes = vec![];
    //let result = PgMissingT::write_to_wire();
    //println!("ceph_write_PgMissingT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgMissingT<'a> {
    pub rmissing: Vec<(HObject<'a>, PgMissingTItem)>,
}

impl<'a> CephPrimitive<'a> for PgMissingT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		rmissing: count!( //A Vec of pair's.  //TODO This should really be a HashMap
            pair!(
                call!(HObject::read_from_wire),
                call!(PgMissingTItem::read_from_wire)), count as usize),
		||{
			PgMissingT{
			rmissing: rmissing,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pool_snap_info_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PoolSnapInfoT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PoolSnapInfoT() {
    //let bytes = vec![];
    //let result = PoolSnapInfoT::write_to_wire();
    //println!("ceph_write_PoolSnapInfoT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PoolSnapInfoT<'a> {
    pub snapid: u64,
    pub stamp: Utime,
    pub name: &'a str,
}

impl<'a> CephPrimitive<'a> for PoolSnapInfoT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		snapid: le_u64 ~
		stamp: call!(Utime::read_from_wire) ~
		name: parse_str,
		||{
			PoolSnapInfoT{
			snapid: snapid,
			stamp: stamp,
			name: name,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_objectstore_perf_stat_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjectstorePerfStatT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectstorePerfStatT() {
    //let bytes = vec![];
    //let result = ObjectstorePerfStatT::write_to_wire();
    //println!("ceph_write_ObjectstorePerfStatT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectstorePerfStatT {
    pub filestore_commit_latency: u32,
    pub filestore_apply_latency: u32,
}

impl<'a> CephPrimitive<'a> for ObjectstorePerfStatT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		filestore_commit_latency: le_u32 ~
		filestore_apply_latency: le_u32,
		||{
			ObjectstorePerfStatT{
			filestore_commit_latency: filestore_commit_latency,
			filestore_apply_latency: filestore_apply_latency,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_PushOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Pushop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pushop() {
    //let bytes = vec![];
    //let result = Pushop::write_to_wire();
    //println!("ceph_write_Pushop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pushop<'a> {
    pub soid: HObject<'a>,
    pub version: EversionT,
    pub data: &'a [u8],
    pub data_included: Vec<u64>,
    pub omap_header: &'a [u8],
    pub omap_entries: Vec<(&'a str, &'a [u8])>,
    pub attrset: Vec<(&'a str, &'a [u8])>,
    pub recovery_info: Objectrecoveryinfo<'a>,
    pub before_progress: Objectrecoveryprogress<'a>,
    pub after_progress: Objectrecoveryprogress<'a>,
}

impl<'a> CephPrimitive<'a> for Pushop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
        data_size: le_u32 ~
		data: take!(data_size) ~
        data_count: le_u32 ~
		data_included: count!(le_u64, data_count as usize) ~
        omap_header_size: le_u32 ~
		omap_header: take!(omap_header_size) ~
		count: le_u32 ~
		omap_entries: count!(pair!(parse_str,take!(10)), count as usize) ~
		count: le_u32 ~
		attrset: count!(pair!(parse_str,take!(10)), count as usize) ~
		recovery_info: call!(Objectrecoveryinfo::read_from_wire) ~
		before_progress: call!(Objectrecoveryprogress::read_from_wire) ~
		after_progress: call!(Objectrecoveryprogress::read_from_wire),
		||{
			Pushop{
			soid: soid,
			version: version,
			data: data,
			data_included: data_included,
			omap_header: omap_header,
			omap_entries: omap_entries,
			attrset: attrset,
			recovery_info: recovery_info,
			before_progress: before_progress,
			after_progress: after_progress,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pg_stat_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgStatT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgStatT() {
    //let bytes = vec![];
    //let result = PgStatT::write_to_wire();
    //println!("ceph_write_PgStatT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgStatT {
    pub struct_version: u8,
    pub compat: u8,
    pub struct_len: u32,
    pub version: EversionT,
    pub reported_seq: u64,
    pub reported_epoch: u32,
    pub state: u32,
    pub last_fresh: Utime,
    pub last_change: Utime,
    pub last_active: Utime,
    pub last_peered: Utime,
    pub last_clean: Utime,
    pub last_unstale: Utime,
    pub last_undegraded: Utime,
    pub last_fullsized: Utime,
    pub log_start: EversionT,
    pub ondisk_log_start: EversionT,
    pub created: u32,
    pub last_epoch_clean: u32,
    pub parent: pg_t,
    pub parent_split_bits: u32,
    pub last_scrub: EversionT,
    pub last_deep_scrub: EversionT,
    pub last_scrub_stamp: Utime,
    pub last_deep_scrub_stamp: Utime,
    pub last_clean_scrub_stamp: Utime,
    pub stats: ObjectStatCollectionT,
    pub stats_invalid: u8,
    pub log_size: i64,
    pub ondisk_log_size: i64,
    /*
    pub up: Vec<i32>,
    pub acting: Vec<i32>,
    pub mapping_epoch: u32,
    pub blocked_by: Vec<i32>,
    pub last_became_active: Utime,
    pub last_became_peered: Utime,
    pub dirty_stats_invalid: u8,
    pub omap_stats_invalid: u8,
    pub hitset_stats_invalid: u8,
    pub hitset_bytes_stats_invalid: u8,
    pub pin_stats_invalid: u8,
    pub up_primary: i32,
    pub acting_primary: i32,
    */
}

impl<'a> CephPrimitive<'a> for PgStatT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        struct_version: le_u8~
        compat: le_u8~
		struct_len: le_u32 ~
		version: call!(EversionT::read_from_wire) ~
		reported_seq: le_u64 ~
		reported_epoch: le_u32 ~
		state: le_u32 ~
		last_fresh: call!(Utime::read_from_wire) ~
		last_change: call!(Utime::read_from_wire) ~
		last_active: call!(Utime::read_from_wire) ~
		last_peered: call!(Utime::read_from_wire) ~
		last_clean: call!(Utime::read_from_wire) ~
		last_unstale: call!(Utime::read_from_wire) ~
		last_undegraded: call!(Utime::read_from_wire) ~
		last_fullsized: call!(Utime::read_from_wire) ~
		log_start: call!(EversionT::read_from_wire) ~
		ondisk_log_start: call!(EversionT::read_from_wire) ~
		created: le_u32 ~
		last_epoch_clean: le_u32 ~
		parent: call!(pg_t::read_from_wire) ~
		parent_split_bits: le_u32 ~
		last_scrub: call!(EversionT::read_from_wire) ~
		last_deep_scrub: call!(EversionT::read_from_wire) ~
		last_scrub_stamp: call!(Utime::read_from_wire) ~
		last_deep_scrub_stamp: call!(Utime::read_from_wire) ~
		last_clean_scrub_stamp: call!(Utime::read_from_wire) ~
		stats: call!(ObjectStatCollectionT::read_from_wire) ~
		stats_invalid: le_u8 ~
		log_size: le_i64 ~
		ondisk_log_size: le_i64 ,
        /*
		up_count: le_u32 ~
		up: count!(le_i32, up_count as usize)~
		acting_count: le_u32 ~
		acting: count!(le_i32, acting_count as usize)~
		mapping_epoch: le_u32 ~
		blocked_count: le_u32 ~
		blocked_by: count!(le_i32, blocked_count as usize)~
		last_became_active: call!(Utime::read_from_wire) ~
		last_became_peered: call!(Utime::read_from_wire) ~
		dirty_stats_invalid: le_u8 ~
		omap_stats_invalid: le_u8 ~
		hitset_stats_invalid: le_u8 ~
		hitset_bytes_stats_invalid: le_u8 ~
		pin_stats_invalid: le_u8 ~
		up_primary: le_i32 ~
		acting_primary: le_i32 ,
        */
		||{
			PgStatT{
            struct_version: struct_version,
            compat: compat,
            struct_len: struct_len,
			version: version,
			reported_seq: reported_seq,
			reported_epoch: reported_epoch,
			state: state,
			last_fresh: last_fresh,
			last_change: last_change,
			last_active: last_active,
			last_peered: last_peered,
			last_clean: last_clean,
			last_unstale: last_unstale,
			last_undegraded: last_undegraded,
			last_fullsized: last_fullsized,
			log_start: log_start,
			ondisk_log_start: ondisk_log_start,
			created: created,
			last_epoch_clean: last_epoch_clean,
			parent: parent,
			parent_split_bits: parent_split_bits,
			last_scrub: last_scrub,
			last_deep_scrub: last_deep_scrub,
			last_scrub_stamp: last_scrub_stamp,
			last_deep_scrub_stamp: last_deep_scrub_stamp,
			last_clean_scrub_stamp: last_clean_scrub_stamp,
			stats: stats,
			stats_invalid: stats_invalid,
			log_size: log_size,
			ondisk_log_size: ondisk_log_size,
            /*
			up: up,
			acting: acting,
			mapping_epoch: mapping_epoch,
			blocked_by: blocked_by,
			last_became_active: last_became_active,
			last_became_peered: last_became_peered,
			dirty_stats_invalid: dirty_stats_invalid,
			omap_stats_invalid: omap_stats_invalid,
			hitset_stats_invalid: hitset_stats_invalid,
			hitset_bytes_stats_invalid: hitset_bytes_stats_invalid,
			pin_stats_invalid: pin_stats_invalid,
			up_primary: up_primary,
			acting_primary: acting_primary,
            */
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
//
//
// #[test]
// fn test_ceph_read_ObjectContextRWState(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = ObjectcontextRwstate::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_ObjectcontextRwstate(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = ObjectcontextRwstate::write_to_wire();
// //println!("ceph_write_ObjectcontextRwstate{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct ObjectcontextRwstate{
// pub state: State,
// pub count: u64,
// pub waiters: waiters,
// pub recovery_read_marker: u8,
// pub snaptrimmer_write_marker: u8,
// }
//
// impl<'a> CephPrimitive<'a> for ObjectcontextRwstate{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// state: call!(State::read_from_wire) ~
// count: le_u64 ~
// count: le_u32 ~
// waiters: count!(call!(OpRequestRef::read_from_wire),count as usize) ~
// recovery_read_marker: le_u8 ~
// snaptrimmer_write_marker: le_u8,
// ||{
// ObjectcontextRwstate{
// state: state,
// count: count,
// waiters: waiters,
// recovery_read_marker: recovery_read_marker,
// snaptrimmer_write_marker: snaptrimmer_write_marker,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//
// #[test]
// fn test_ceph_read_ScrubMapObject(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = ScrubmapObject::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_ScrubmapObject(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = ScrubmapObject::write_to_wire();
// //println!("ceph_write_ScrubmapObject{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct ScrubmapObject{
// pub size: u64,
// pub negative: u8,
// pub attrs: attrs,
// pub digest: u32,
// pub digest_present: u8,
// pub nlinks: u32,
// pub snapcolls: snapcolls,
// pub omap_digest: u32,
// pub omap_digest_present: u8,
// pub read_error: u8,
// }
//
// impl<'a> CephPrimitive<'a> for ScrubmapObject{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// size: le_u64 ~
// negative: le_u8 ~
// count: le_u32 ~
// attrs: count!(pair!(parse_str,bufferptr), count as usize) ~
// digest: le_u32 ~
// digest_present: le_u8 ~
// nlinks: le_u32 ~
// count: le_u32 ~
// snapcolls: count!(call!(napid_t::read_from_wire),count as usize) ~
// omap_digest: le_u32 ~
// omap_digest_present: le_u8 ~
// read_error: le_u8,
// ||{
// ScrubmapObject{
// size: size,
// negative: negative,
// attrs: attrs,
// digest: digest,
// digest_present: digest_present,
// nlinks: nlinks,
// snapcolls: snapcolls,
// omap_digest: omap_digest,
// omap_digest_present: omap_digest_present,
// read_error: read_error,
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
fn test_ceph_read_old_pg_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = OldPgT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_OldPgT() {
    //let bytes = vec![];
    //let result = OldPgT::write_to_wire();
    //println!("ceph_write_OldPgT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

//
// placement group.
// we encode this into one __le64.
// struct ceph_pg {
// __le16 preferred; /* preferred primary osd */
// __le16 ps;        /* placement seed */
// __le32 pool;      /* object pool */
// } __attribute__ ((packed));
//
//
#[derive(Debug,Eq,PartialEq)]
pub struct OldPgT {
    pub v: u64,
}

impl<'a> CephPrimitive<'a> for OldPgT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		v: le_u64,
		||{
			OldPgT{
			v: v,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_PullOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Pullop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pullop() {
    //let bytes = vec![];
    //let result = Pullop::write_to_wire();
    //println!("ceph_write_Pullop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pullop<'a> {
    pub soid: HObject<'a>,
    pub recovery_info: Objectrecoveryinfo<'a>,
    pub recovery_progress: Objectrecoveryprogress<'a>,
}

impl<'a> CephPrimitive<'a> for Pullop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		soid: call!(HObject::read_from_wire) ~
		recovery_info: call!(Objectrecoveryinfo::read_from_wire) ~
		recovery_progress: call!(Objectrecoveryprogress::read_from_wire),
		||{
			Pullop{
			soid: soid,
			recovery_info: recovery_info,
			recovery_progress: recovery_progress,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_pg_missing_tItem() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgMissingTItem::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgMissingTItem() {
    //let bytes = vec![];
    //let result = PgMissingTItem::write_to_wire();
    //println!("ceph_write_PgMissingTItem{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgMissingTItem {
    pub need: EversionT,
    pub have: EversionT,
}

impl<'a> CephPrimitive<'a> for PgMissingTItem {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		need: call!(EversionT::read_from_wire) ~
		have: call!(EversionT::read_from_wire),
		||{
			PgMissingTItem{
			need: need,
			have: have,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_watch_item_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = WatchItemT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_WatchItemT() {
    //let bytes = vec![];
    //let result = WatchItemT::write_to_wire();
    //println!("ceph_write_WatchItemT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct WatchItemT {
    pub name: EntityNameT,
    pub cookie: u64,
    pub timeout_seconds: u32,
    pub addr: EntityAddr,
}

impl<'a> CephPrimitive<'a> for WatchItemT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		name: call!(EntityNameT::read_from_wire) ~
		cookie: le_u64 ~
		timeout_seconds: le_u32 ~
		addr: call!(EntityAddr::read_from_wire),
		||{
			WatchItemT{
			name: name,
			cookie: cookie,
			timeout_seconds: timeout_seconds,
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
fn test_ceph_read_notify_info_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = NotifyInfoT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_NotifyInfoT() {
    //let bytes = vec![];
    //let result = NotifyInfoT::write_to_wire();
    //println!("ceph_write_NotifyInfoT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct NotifyInfoT<'a> {
    pub cookie: u64,
    pub notify_id: u64,
    pub timeout: u32,
    pub bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for NotifyInfoT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		cookie: le_u64 ~
		notify_id: le_u64 ~
		timeout: le_u32 ~
        bl_size: le_u32 ~
		bl: take!(bl_size),
		||{
			NotifyInfoT{
			cookie: cookie,
			notify_id: notify_id,
			timeout: timeout,
			bl: bl,
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
// fn test_ceph_read_ECSubWrite() {
// //let bytes = vec![];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Ecsubwrite::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Ecsubwrite() {
// //let bytes = vec![];
// //let result = Ecsubwrite::write_to_wire();
// //println!("ceph_write_Ecsubwrite{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Ecsubwrite {
// pub from: PgShardT,
// pub tid: u64,
// pub reqid: reqid,
// pub soid: HObject,
// pub stats: PgStatT,
// pub t: t,
// pub at_version: EversionT,
// pub trim_to: EversionT,
// pub trim_rollback_to: EversionT,
// pub log_entries: Vec<pg_log_entry_t>,
// pub temp_added: Vec<HObject>,
// pub temp_removed: Vec<HObject>,
// pub updated_hit_set_history: pg_hit_set_history_t,
// }
//
// impl<'a> CephPrimitive<'a> for Ecsubwrite {
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
// chain!(input,
// from: call!(PgShardT::read_from_wire) ~
// tid: le_u64 ~
// reqid: call!(OsdReqidT::read_from_wire) ~
// soid: call!(HObject::read_from_wire) ~
// stats: call!(PgStatT::read_from_wire) ~
// t: call!(ObjectStore::read_from_wire) ~
// at_version: call!(EversionT::read_from_wire) ~
// trim_to: call!(EversionT::read_from_wire) ~
// trim_rollback_to: call!(EversionT::read_from_wire) ~
// count: le_u32 ~
// log_entries: count!(call!(pg_log_entry_t::read_from_wire), count as usize) ~
// count: le_u32 ~
// temp_added: count!(
// call!(HObject::read_from_wire),count as usize) ~
// count: le_u32 ~
// temp_removed: count!(call!(HObject::read_from_wire),count as usize) ~
// updated_hit_set_history: opt!(pg_hit_set_history_t::read_from_wire),
// ||{
// Ecsubwrite{
// from: from,
// tid: tid,
// reqid: reqid,
// soid: soid,
// stats: stats,
// t: t,
// at_version: at_version,
// trim_to: trim_to,
// trim_rollback_to: trim_rollback_to,
// log_entries: log_entries,
// temp_added: temp_added,
// temp_removed: temp_removed,
// updated_hit_set_history: updated_hit_set_history,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//

#[test]
fn test_ceph_read_watch_info_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = WatchInfoT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_WatchInfoT() {
    //let bytes = vec![];
    //let result = WatchInfoT::write_to_wire();
    //println!("ceph_write_WatchInfoT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct WatchInfoT {
    pub cookie: u64,
    pub timeout_seconds: u32,
    pub addr: EntityAddr,
}

impl<'a> CephPrimitive<'a> for WatchInfoT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		cookie: le_u64 ~
		timeout_seconds: le_u32 ~
		addr: call!(EntityAddr::read_from_wire),
		||{
			WatchInfoT{
			cookie: cookie,
			timeout_seconds: timeout_seconds,
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
fn test_ceph_read_ObjectRecoveryProgress() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Objectrecoveryprogress::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectrecoveryprogress() {
    //let bytes = vec![];
    //let result = Objectrecoveryprogress::write_to_wire();
    //println!("ceph_write_Objectrecoveryprogress{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectrecoveryprogress<'a> {
    pub first: u8,
    pub data_recovered_to: u64,
    pub data_complete: u8,
    pub omap_recovered_to: &'a str,
    pub omap_complete: u8,
}

impl<'a> CephPrimitive<'a> for Objectrecoveryprogress<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		first: le_u8 ~
		data_recovered_to: le_u64 ~
		data_complete: le_u8 ~
		omap_recovered_to: parse_str ~
		omap_complete: le_u8,
		||{
			Objectrecoveryprogress{
			first: first,
			data_recovered_to: data_recovered_to,
			data_complete: data_complete,
			omap_recovered_to: omap_recovered_to,
			omap_complete: omap_complete,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_object_info_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjectInfoT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectInfoT() {
    //let bytes = vec![];
    //let result = ObjectInfoT::write_to_wire();
    //println!("ceph_write_ObjectInfoT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectInfoT<'a> {
    pub soid: HObject<'a>,
    pub version: EversionT,
    pub prior_version: EversionT,
    pub user_version: u64,
    pub last_reqid: OsdReqidT,
    pub size: u64,
    pub mtime: Utime,
    pub local_mtime: Utime,
    pub flags: i16,
    pub snaps: Vec<u64>,
    pub truncate_seq: u64,
    pub truncate_size: u64,
    pub watchers: Vec<(u64, EntityNameT)>,
    pub data_digest: u32,
    pub omap_digest: u32,
}

impl<'a> CephPrimitive<'a> for ObjectInfoT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		prior_version: call!(EversionT::read_from_wire) ~
		user_version: le_u64 ~
		last_reqid: call!(OsdReqidT::read_from_wire) ~
		size: le_u64 ~
		mtime: call!(Utime::read_from_wire) ~
		local_mtime: call!(Utime::read_from_wire) ~
		flags: le_i16 ~
		count: le_u32 ~
		snaps: count!(le_u64, count as usize)~
		truncate_seq: le_u64 ~
		truncate_size: le_u64 ~
		count: le_u32 ~
		watchers: count!(
            pair!(
                le_u64,
                call!(EntityNameT::read_from_wire)), count as usize) ~
		data_digest: le_u32 ~
		omap_digest: le_u32,
		||{
			ObjectInfoT{
			soid: soid,
			version: version,
			prior_version: prior_version,
			user_version: user_version,
			last_reqid: last_reqid,
			size: size,
			mtime: mtime,
			local_mtime: local_mtime,
			flags: flags,
			snaps: snaps,
			truncate_seq: truncate_seq,
			truncate_size: truncate_size,
			watchers: watchers,
			data_digest: data_digest,
			omap_digest: omap_digest,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_object_stat_collection_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = ObjectStatCollectionT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectStatCollectionT() {
    //let bytes = vec![];
    //let result = ObjectStatCollectionT::write_to_wire();
    //println!("ceph_write_ObjectStatCollectionT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectStatCollectionT {
    pub sum: ObjectStatSumT,
}

impl<'a> CephPrimitive<'a> for ObjectStatCollectionT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		sum: call!(ObjectStatSumT::read_from_wire),
		||{
			ObjectStatCollectionT{
			sum: sum,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_ObjectModDesc() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Objectmoddesc::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectmoddesc() {
    //let bytes = vec![];
    //let result = Objectmoddesc::write_to_wire();
    //println!("ceph_write_Objectmoddesc{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectmoddesc<'a> {
    pub bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Objectmoddesc<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        count: le_u32 ~
		bl: take!(count as usize),
		||{
			Objectmoddesc{
			bl: bl,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_pg_nls_response_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = PgNlsResponseT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgNlsResponseT() {
    //let bytes = vec![];
    //let result = PgNlsResponseT::write_to_wire();
    //println!("ceph_write_PgNlsResponseT{:?}", result);
    // assert_eq!(result, expected_bytes);
}
#[derive(Debug,Eq,PartialEq)]
pub struct ListObjectImpl<'a> {
    nspace: &'a str,
    oid: &'a str,
    locator: &'a str,
}

impl<'a> CephPrimitive<'a> for ListObjectImpl<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        nspace: parse_str ~
        oid: parse_str ~
        locator: parse_str ,
		||{
			ListObjectImpl{
                nspace: nspace,
                oid: oid,
                locator: locator,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgNlsResponseT<'a> {
    pub handle: HObject<'a>,
    pub entries: Vec<ListObjectImpl<'a>>,
}

impl<'a> CephPrimitive<'a> for PgNlsResponseT<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		handle: call!(HObject::read_from_wire) ~
		count: le_u32 ~
		entries: count!(call!(ListObjectImpl::read_from_wire),count as usize),
		||{
			PgNlsResponseT{
			handle: handle,
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
fn test_ceph_read_spg_t() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = SpgT::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_SpgT() {
    //let bytes = vec![];
    //let result = SpgT::write_to_wire();
    //println!("ceph_write_SpgT{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct SpgT {
    pub version: u8,
    pub compat: u8,
    pub struct_len: u32,
    pub pgid: pg_t,
    pub shard: i8,
}

impl<'a> CephPrimitive<'a> for SpgT {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        version: le_u8~
        compat: le_u8~
        struct_len: le_u32~
		pgid: call!(pg_t::read_from_wire) ~
		shard: le_i8,
		||{
			SpgT{
            version: version,
            compat: compat,
            struct_len: struct_len,
			pgid: pgid,
			shard: shard,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_ObjectRecoveryInfo() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Objectrecoveryinfo::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectrecoveryinfo() {
    //let bytes = vec![];
    //let result = Objectrecoveryinfo::write_to_wire();
    //println!("ceph_write_Objectrecoveryinfo{:?}", result);
    // assert_eq!(result, expected_bytes);
}
#[derive(Debug,Eq,PartialEq)]
pub struct SnapContext {
    pub seq: u64, // 'time' stamp
    pub snaps: Vec<u64>, // existent snaps, in descending order
}
impl<'a> CephPrimitive<'a> for SnapContext {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          seq: le_u64~
          snaps_len: le_u32~
          snaps: count!(le_u64, snaps_len as usize),
		||{
			SnapContext{
                seq: seq,
                snaps: snaps,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct SnapSet {
    pub seq: u64,
    pub head_exists: u8,
    pub snaps: Vec<u64>, // descending
    pub clones: Vec<u64>, // ascending
    pub clone_overlap: Vec<(u64, u64)>, // overlap w/ next newest
    pub clone_size: Vec<(u64, u64)>,
}

impl<'a> CephPrimitive<'a> for SnapSet {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
          seq: le_u64~
          head_exists: le_u8~
          snap_len: le_u32 ~
          snaps: count!(le_u64, snap_len as usize)~
          clone_len: le_u32~
          clones: count!(le_u64, clone_len as usize)~
          clone_overlap_len: le_u32~
          clone_overlap: count!(pair!(le_u64, le_u64), clone_overlap_len as usize)~
          clone_size_len: le_u32~
          clone_size: count!(pair!(le_u64, le_u64), clone_size_len as usize),
		||{
			SnapSet{
                seq: seq,
                head_exists: head_exists,
                snaps: snaps,
                clones: clones,
                clone_overlap: clone_overlap,
                clone_size: clone_size,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectrecoveryinfo<'a> {
    pub soid: HObject<'a>,
    pub version: EversionT,
    pub size: u64,
    pub oi: ObjectInfoT<'a>,
    pub ss: SnapSet,
    pub copy_subset: Vec<u64>,
    pub clone_subset: Vec<(HObject<'a>, Vec<u64>)>,
}

impl<'a> CephPrimitive<'a> for Objectrecoveryinfo<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		size: le_u64 ~
		oi: call!(ObjectInfoT::read_from_wire) ~
		ss: call!(SnapSet::read_from_wire) ~
        copy_count: le_u32~
		copy_subset: count!(le_u64, copy_count as usize) ~
		count: le_u32 ~
		clone_subset: count!(pair!(HObject::read_from_wire,count!(le_u64, count as usize)), count as usize) ,
		||{
			Objectrecoveryinfo{
			soid: soid,
			version: version,
			size: size,
			oi: oi,
			ss: ss,
			copy_subset: copy_subset,
			clone_subset: clone_subset,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MDataPing() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mdataping::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mdataping() {
    //let bytes = vec![];
    //let result = Mdataping::write_to_wire();
    //println!("ceph_write_Mdataping{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mdataping<'a> {
    pub tag: &'a str,
    pub counter: u32,
}

impl<'a> CephPrimitive<'a> for Mdataping<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		tag: parse_str ~
		counter: le_u32,
		||{
			Mdataping{
			tag: tag,
			counter: counter,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGCreate() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgcreate::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgcreate() {
    //let bytes = vec![];
    //let result = Mosdpgcreate::write_to_wire();
    //println!("ceph_write_Mosdpgcreate{:?}", result);
    // assert_eq!(result, expected_bytes);
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_t {
    pub version: u8,
    pub m_pool: u64,
    pub m_seed: u32,
    pub m_preferred: i32,
}
impl<'a> CephPrimitive<'a> for pg_t {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        version: le_u8~
		m_pool: le_u64 ~
		m_seed: le_u32 ~
		m_preferred:le_i32,
		||{
			pg_t{
                version: version,
                m_pool: m_pool,
                m_seed: m_seed,
                m_preferred: m_preferred,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct pg_create_t {
    created: u32,
    parent: pg_t, // split from parent (if != pg_t())
    split_bits: i32,
}
impl<'a> CephPrimitive<'a> for pg_create_t {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		created: le_u32 ~
        parent: call!(pg_t::read_from_wire) ~
        split_bits: le_i32,
		||{
			pg_create_t{
			created: created,
            parent: parent,
            split_bits: split_bits,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}


#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgcreate {
    pub epoch: u64,
    pub mkpg: Vec<(pg_t, pg_create_t)>,
    pub ctimes: Vec<(pg_t, Utime)>,
}

impl<'a> CephPrimitive<'a> for Mosdpgcreate {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		epoch: le_u64 ~
		count: le_u32 ~
		mkpg: count!(
            pair!(
                call!(pg_t::read_from_wire),
                call!(pg_create_t::read_from_wire)), count as usize) ~
		count: le_u32 ~
		ctimes: count!(
            pair!(
                call!(pg_t::read_from_wire),
                call!(Utime::read_from_wire)), count as usize) ,
		||{
			Mosdpgcreate{
			epoch: epoch,
			mkpg: mkpg,
			ctimes: ctimes,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MOsdRepOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = MOsdRepOp::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_MOsdRepOp() {
    //let bytes = vec![];
    //let result = MOsdRepOp::write_to_wire();
    //println!("ceph_write_MOsdRepOp{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct MOsdRepOp<'a> {
    pub map_epoch: u32,
    pub reqid: OsdReqidT,
    pub from: PgShardT,
    pub pgid: SpgT,
    pub poid: HObject<'a>,
    pub acks_wanted: u8,
    pub logbl: &'a [u8],
    pub pg_stats: PgStatT,
    pub version: EversionT,
    pub pg_trim_to: EversionT,
    pub pg_trim_rollback_to: EversionT,
    pub new_temp_oid: HObject<'a>,
    pub discard_temp_oid: HObject<'a>,
    pub updated_hit_set_history: Option<pg_hit_set_history_t>,
}

impl<'a> CephPrimitive<'a> for MOsdRepOp<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(OsdReqidT::read_from_wire) ~
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		poid: call!(HObject::read_from_wire) ~
		acks_wanted: le_u8 ~
        log_size: le_u32 ~
		logbl: take!(log_size) ~
		pg_stats: call!(PgStatT::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		pg_trim_to: call!(EversionT::read_from_wire) ~
		pg_trim_rollback_to: call!(EversionT::read_from_wire) ~
		new_temp_oid: call!(HObject::read_from_wire) ~
		discard_temp_oid: call!(HObject::read_from_wire) ~
        hit_set_history_is_present: le_u8~// If this is not zero than grab hit_set_history
		updated_hit_set_history: hard_opt!(pg_hit_set_history_t::read_from_wire),
		||{
			MOsdRepOp{
			map_epoch: map_epoch,
			reqid: reqid,
			from: from,
			pgid: pgid,
			poid: poid,
			acks_wanted: acks_wanted,
			logbl: logbl,
			pg_stats: pg_stats,
			version: version,
			pg_trim_to: pg_trim_to,
			pg_trim_rollback_to: pg_trim_rollback_to,
			new_temp_oid: new_temp_oid,
			discard_temp_oid: discard_temp_oid,
			updated_hit_set_history: updated_hit_set_history,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MBackfillReserve() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mbackfillreserve::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mbackfillreserve() {
    //let bytes = vec![];
    //let result = Mbackfillreserve::write_to_wire();
    //println!("ceph_write_Mbackfillreserve{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mbackfillreserve {
    pub pgid: SpgT,
    pub query_epoch: u32,
    pub backfill_type: u32,
    pub priority: u32,
}

impl<'a> CephPrimitive<'a> for Mbackfillreserve {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		query_epoch: le_u32 ~
		backfill_type: le_u32 ~
		priority: le_u32,
		||{
			Mbackfillreserve{
			pgid: pgid,
			query_epoch: query_epoch,
			backfill_type: backfill_type,
			priority: priority,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MWatchNotify() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mwatchnotify::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mwatchnotify() {
    //let bytes = vec![];
    //let result = Mwatchnotify::write_to_wire();
    //println!("ceph_write_Mwatchnotify{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mwatchnotify<'a> {
    pub cookie: u64,
    pub ver: u64,
    pub notify_id: u64,
    pub opcode: u8,
    pub bl: &'a [u8],
    pub return_code: i32,
    pub notifier_gid: u64,
}

impl<'a> CephPrimitive<'a> for Mwatchnotify<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		cookie: le_u64 ~
		ver: le_u64 ~
		notify_id: le_u64 ~
		opcode: le_u8 ~
        bl_size: le_u32 ~
		bl: take!(bl_size) ~
		return_code: le_i32 ~
		notifier_gid: le_u64,
		||{
			Mwatchnotify{
			cookie: cookie,
			ver: ver,
			notify_id: notify_id,
			opcode: opcode,
			bl: bl,
			return_code: return_code,
			notifier_gid: notifier_gid,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MPGStats() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mpgstats::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpgstats() {
    //let bytes = vec![];
    //let result = Mpgstats::write_to_wire();
    //println!("ceph_write_Mpgstats{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct osd_stat_t {
    pub kb: i64,
    pub kb_used: i64,
    pub kb_avail: i64,
    pub hb_in: Vec<i32>,
    pub hb_out: Vec<i32>,
    pub snap_trim_queue_len: i32,
    pub num_snap_trimming: i32,
    pub op_queue_age_hist: Vec<i32>, // power of 2 histogram
    pub fs_perf_stat: ObjectstorePerfStatT,
}

impl<'a> CephPrimitive<'a> for osd_stat_t {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		kb: le_i64 ~
		kb_used: le_i64 ~
		kb_avail: le_i64 ~
        hb_in_count: le_u32 ~
        hb_in: count!(le_i32, hb_in_count as usize) ~
        hb_out_count: le_u32 ~
        hb_out: count!(le_i32, hb_in_count as usize) ~
        snap_trim_queue_len: le_i32~
        num_snap_trimming: le_i32 ~
        count: le_u32~
        op_queue_age_hist: count!(le_i32, count as usize)~
        fs_perf_stat: call!(ObjectstorePerfStatT::read_from_wire),
		||{
			osd_stat_t{
    		kb: kb,
    		kb_used: kb_used,
    		kb_avail: kb_avail,
            hb_in: hb_in,
            hb_out: hb_out,
            snap_trim_queue_len: snap_trim_queue_len,
            num_snap_trimming: num_snap_trimming,
            op_queue_age_hist: op_queue_age_hist,
            fs_perf_stat: fs_perf_stat,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct Mpgstats {
    pub fsid: Uuid,
    pub pg_stat: Vec<(pg_t, PgStatT)>,
    pub osd_stat: osd_stat_t,
    pub epoch: u32,
    pub had_map_for: Utime,
}

impl<'a> CephPrimitive<'a> for Mpgstats {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pg_stat: count!(
            pair!(
                call!(pg_t::read_from_wire),
                call!(PgStatT::read_from_wire)), count as usize) ~
		osd_stat: call!(osd_stat_t::read_from_wire) ~
		epoch: le_u32 ~
		had_map_for: call!(Utime::read_from_wire) ,
		||{
			Mpgstats{
			fsid: fsid,
			pg_stat: pg_stat,
			osd_stat: osd_stat,
			epoch: epoch,
			had_map_for: had_map_for,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGQuery() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgquery::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgquery() {
    //let bytes = vec![];
    //let result = Mosdpgquery::write_to_wire();
    //println!("ceph_write_Mosdpgquery{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgquery {
    pub pg_list: Vec<(SpgT, PgQueryT)>,
}

impl<'a> CephPrimitive<'a> for Mosdpgquery {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		pg_list: count!(pair!(
            call!(SpgT::read_from_wire),
            call!(PgQueryT::read_from_wire)), count as usize) ,
		||{
			Mosdpgquery{
			pg_list: pg_list,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MOSDPGBackfill() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgbackfill::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgbackfill() {
    //let bytes = vec![];
    //let result = Mosdpgbackfill::write_to_wire();
    //println!("ceph_write_Mosdpgbackfill{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgbackfill<'a> {
    pub op: u32,
    pub map_epoch: u32,
    pub query_epoch: u32,
    pub pgid: SpgT,
    pub last_backfill: HObject<'a>,
    pub compat_stat_sum: u8,
    pub stats: PgStatT,
}

impl<'a> CephPrimitive<'a> for Mosdpgbackfill<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		op: le_u32 ~
		map_epoch: le_u32 ~
		query_epoch: le_u32 ~
		pgid: call!(SpgT::read_from_wire) ~
		last_backfill: call!(HObject::read_from_wire) ~
		compat_stat_sum: le_u8 ~
		stats: call!(PgStatT::read_from_wire),
		||{
			Mosdpgbackfill{
			op: op,
			map_epoch: map_epoch,
			query_epoch: query_epoch,
			pgid: pgid,
			last_backfill: last_backfill,
			compat_stat_sum: compat_stat_sum,
			stats: stats,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDFailure() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdfailure::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdfailure() {
    //let bytes = vec![];
    //let result = Mosdfailure::write_to_wire();
    //println!("ceph_write_Mosdfailure{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdfailure {
    pub fsid: Uuid,
    pub target_osd: EntityInstT,
    pub is_failed: u8,
    pub epoch: u32,
    pub failed_for: i32,
}

impl<'a> CephPrimitive<'a> for Mosdfailure {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		target_osd: call!(EntityInstT::read_from_wire) ~
		is_failed: le_u8 ~
		epoch: le_u32 ~
		failed_for: le_i32,
		||{
			Mosdfailure{
			fsid: fsid,
			target_osd: target_osd,
			is_failed: is_failed,
			epoch: epoch,
			failed_for: failed_for,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGMissing() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgmissing::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgmissing() {
    //let bytes = vec![];
    //let result = Mosdpgmissing::write_to_wire();
    //println!("ceph_write_Mosdpgmissing{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgmissing<'a> {
    pub info: PgInfoT<'a>,
    pub missing: PgMissingT<'a>,
}

impl<'a> CephPrimitive<'a> for Mosdpgmissing<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		info: call!(PgInfoT::read_from_wire) ~
		missing: call!(PgMissingT::read_from_wire),
		||{
			Mosdpgmissing{
			info: info,
			missing: missing,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGTemp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgtemp::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgtemp() {
    //let bytes = vec![];
    //let result = Mosdpgtemp::write_to_wire();
    //println!("ceph_write_Mosdpgtemp{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgtemp {
    pub map_epoch: u32,
    pub pg_temp: Vec<(pg_t, Vec<i32>)>,
}

impl<'a> CephPrimitive<'a> for Mosdpgtemp {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		map_epoch: le_u32 ~
		count: le_u32 ~
		pg_temp: count!(
			pair!(
                call!(pg_t::read_from_wire),
			count!(le_i32, count as usize)), count as usize),
		||{
			Mosdpgtemp{
			map_epoch: map_epoch,
			pg_temp: pg_temp,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MPoolOpReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mpoolopreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpoolopreply() {
    //let bytes = vec![];
    //let result = Mpoolopreply::write_to_wire();
    //println!("ceph_write_Mpoolopreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpoolopreply<'a> {
    pub fsid: Uuid,
    pub replyCode: u32,
    pub epoch: u32,
    pub response_data: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Mpoolopreply<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		replyCode: le_u32 ~
		epoch: le_u32 ~
        response_size: le_u32 ~
		response_data: take!(response_size),
		||{
			Mpoolopreply{
			fsid: fsid,
			replyCode: replyCode,
			epoch: epoch,
			response_data: response_data,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGInfo() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpginfo::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpginfo() {
    //let bytes = vec![];
    //let result = Mosdpginfo::write_to_wire();
    //println!("ceph_write_Mosdpginfo{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpginfo<'a> {
    pub pg_list: Vec<(PgNotifyT<'a>, u32)>,
}

impl<'a> CephPrimitive<'a> for Mosdpginfo<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		pg_list: count!(
            pair!(
                call!(PgNotifyT::read_from_wire),le_u32
                //length_value!(
                //    le_u32,
                //    call!(PgIntervalT::read_from_wire))
            ), count as usize),
		||{
			Mosdpginfo{
			pg_list: pg_list,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MPoolOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mpoolop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpoolop() {
    //let bytes = vec![];
    //let result = Mpoolop::write_to_wire();
    //println!("ceph_write_Mpoolop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpoolop<'a> {
    pub fsid: Uuid,
    pub pool: u32,
    pub name: &'a str,
    pub op: u32,
    pub auid: u64,
    pub snapid: u64,
    pub crush_rule: i16,
}

impl<'a> CephPrimitive<'a> for Mpoolop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		pool: le_u32 ~
		name: parse_str ~
		op: le_u32 ~
		auid: le_u64 ~
		snapid: le_u64 ~
		crush_rule: le_i16,
		||{
			Mpoolop{
			fsid: fsid,
			pool: pool,
			name: name,
			op: op,
			auid: auid,
			snapid: snapid,
			crush_rule: crush_rule,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MOSDSubOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdsubop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdsubop() {
    //let bytes = vec![];
    //let result = Mosdsubop::write_to_wire();
    //println!("ceph_write_Mosdsubop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectModDesc {
    pub can_local_rollback: u8,
    pub rollback_info_completed: u8,
}

impl<'a> CephPrimitive<'a> for ObjectModDesc {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
            can_local_rollback: le_u8~
            rollback_info_completed: le_u8,
    		||{
    			ObjectModDesc{
                    can_local_rollback: can_local_rollback,
                    rollback_info_completed: rollback_info_completed,
            }
        })
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_log_entry_t<'a>{
    pub op: i32,
    pub soid: HObject<'a>,
    pub version: EversionT,
    pub prior_version: EversionT,
    pub reverting_to: EversionT,
    pub user_version: u64, // the user version for this entry
    pub reqid: OsdReqidT,  // caller+tid to uniquely identify request
    pub mtime: Utime,  // this is the _user_ mtime, mind you
    pub snaps: Vec<u64>,   // only for clone entries
    pub invalid_hash: u8, // only when decoding sobject_t based entries
    pub invalid_pool: u8, // only when decoding pool-less hobject based entries
    pub offset: u64,   // [soft state] my offset on disk
    /// describes state for a locally-rollbackable entry
    pub mod_desc: ObjectModDesc,
    pub extra_reqids: Vec<(OsdReqidT, u64)>,
}
impl<'a> CephPrimitive<'a> for pg_log_entry_t<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
            op: le_i32 ~
            soid: call!(HObject::read_from_wire)~
            version: call!(EversionT::read_from_wire)~
            prior_version: call!(EversionT::read_from_wire)~
            reverting_to: call!(EversionT::read_from_wire)~
            user_version: le_u64 ~
            reqid: call!(OsdReqidT::read_from_wire)~
            mtime: call!(Utime::read_from_wire)~
            snap_len: le_u32 ~
            snaps: count!(le_u64, snap_len as usize) ~
            invalid_hash: le_u8~
            invalid_pool: le_u8~
            offset: le_u64~
            mod_desc: call!(ObjectModDesc::read_from_wire)~
            req_id_len: le_u32~
            extra_reqids: count!(
                pair!(
                    call!(OsdReqidT::read_from_wire),
                    le_u64),
                req_id_len as usize),
    		||{
    			pg_log_entry_t{
                    op: op,
                    soid: soid,
                    version: version,
                    prior_version: prior_version,
                    reverting_to: reverting_to,
                    user_version: user_version,
                    reqid: reqid,
                    mtime: mtime,
                    snaps: snaps,
                    invalid_hash: invalid_hash,
                    invalid_pool: invalid_pool,
                    offset: offset,
                    mod_desc: mod_desc,
                    extra_reqids: extra_reqids,
            }
        })
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct ceph_osd_op {
    pub op: u16, // CEPH_OSD_OP_*
    pub flags: u32, // CEPH_OSD_OP_FLAG_*
    pub extent_offset: u64,
    pub extent_length: u64,
    pub extent_truncate_size: u64,
    pub extent_truncate_seq: u32,
    pub xattr_name_len: u32,
    pub xattr_value_len: u32,
    pub xattr_cmp_op: u8, // CEPH_OSD_CMPXATTR_OP_*
    pub xattr_cmp_mode: u8, // CEPH_OSD_CMPXATTR_MODE_*
    pub cls_class_len: u8,
    pub cls_method_len: u8,
    pub cls_argc: u8,
    pub cls_indata_len: u32,
    pub pgls_count: u64,
    pub pgls_start_epoch: u32, // for the pgls sequence
    pub snap_snapid: u64,
    pub watch_cookie: u64,
    pub watch_ver: u64, // no longer used
    pub watch_op: u8, // CEPH_OSD_WATCH_OP_*
    pub watch_gen: u32, // registration generation
    pub notify_cookie: u64,
    pub assert_ver_unused: u64,
    pub assert_ver: u64,
    pub clonerange_offset: u64,
    pub clonerange_length: u64,
    pub clonerange_src_offset: u64,
    pub copy_get_max: u64, // max data in reply
    pub copy_get_flags: u32,
    pub copy_from_snapid: u64,
    pub copy_from_src_version: u64,
    pub copy_from_flags: u8,
    pub copy_from_src_fadvise_flags: u32,
    pub hit_set_get_timestamp: Utime,
    pub tmap2omap_flags: u8,
    pub alloc_hint_expected_object_size: u64,
    pub alloc_hint_expected_write_size: u64,
    pub payload_len: u32,
}

impl<'a> CephPrimitive<'a> for ceph_osd_op {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
            op: le_u16~
            flags: le_u32~
            extent_offset: le_u64~
            extent_length: le_u64~
            extent_truncate_size: le_u64~
            extent_truncate_seq: le_u32~
            xattr_name_len: le_u32~
            xattr_value_len: le_u32~
            xattr_cmp_op: le_u8~
            xattr_cmp_mode: le_u8~
            cls_class_len: le_u8~
            cls_method_len: le_u8~
            cls_argc: le_u8~
            cls_indata_len: le_u32~
            pgls_count: le_u64~
            pgls_start_epoch: le_u32~
            snap_snapid: le_u64~
            watch_cookie: le_u64~
            watch_ver: le_u64~
            watch_op: le_u8~
            watch_gen: le_u32~
            notify_cookie: le_u64~
            assert_ver_unused: le_u64~
            assert_ver: le_u64~
            clonerange_offset: le_u64~
            clonerange_length: le_u64~
            clonerange_src_offset: le_u64~
            copy_get_max: le_u64~
            copy_get_flags: le_u32~
            copy_from_snapid: le_u64~
            copy_from_src_version: le_u64~
            copy_from_flags: le_u8~
            copy_from_src_fadvise_flags: le_u32~
            hit_set_get_timestamp: call!(Utime::read_from_wire)~
            tmap2omap_flags: le_u8~
            alloc_hint_expected_object_size: le_u64~
            alloc_hint_expected_write_size: le_u64~
            payload_len: le_u32,
    		||{
    			ceph_osd_op{
                    op: op,
                    flags: flags,
                    extent_offset: extent_offset,
                    extent_length: extent_length,
                    extent_truncate_size: extent_truncate_size,
                    extent_truncate_seq: extent_truncate_seq,
                    xattr_name_len: xattr_name_len,
                    xattr_value_len: xattr_value_len,
                    xattr_cmp_op: xattr_cmp_op,
                    xattr_cmp_mode: xattr_cmp_mode,
                    cls_class_len: cls_class_len,
                    cls_method_len: cls_method_len,
                    cls_argc: cls_argc,
                    cls_indata_len: cls_indata_len,
                    pgls_count: pgls_count,
                    pgls_start_epoch: pgls_start_epoch,
                    snap_snapid: snap_snapid,
                    watch_cookie: watch_cookie,
                    watch_ver: watch_ver,
                    watch_op: watch_op,
                    watch_gen: watch_gen,
                    notify_cookie: notify_cookie,
                    assert_ver_unused: assert_ver_unused,
                    assert_ver: assert_ver,
                    clonerange_offset: clonerange_offset,
                    clonerange_length: clonerange_length,
                    clonerange_src_offset: clonerange_src_offset,
                    copy_get_max: copy_get_max,
                    copy_get_flags: copy_get_flags,
                    copy_from_snapid: copy_from_snapid,
                    copy_from_src_version: copy_from_src_version,
                    copy_from_flags: copy_from_flags,
                    copy_from_src_fadvise_flags: copy_from_src_fadvise_flags,
                    hit_set_get_timestamp: hit_set_get_timestamp,
                    tmap2omap_flags: tmap2omap_flags,
                    alloc_hint_expected_object_size: alloc_hint_expected_object_size,
                    alloc_hint_expected_write_size: alloc_hint_expected_write_size,
                    payload_len: payload_len,
            }
        })
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct sobject_t<'a> {
    pub oid: &'a str,
    pub snap: u64,
}
impl<'a> CephPrimitive<'a> for sobject_t<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
            oid: parse_str ~
            snap: le_u64,
    		||{
			sobject_t{
                oid: oid,
                snap: snap,
            }
        })
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct COsdOp<'a> {
    pub op: ceph_osd_op,
    pub soid: sobject_t<'a>,
    // bufferlist indata, outdata;
    pub rval: i32,
}
impl<'a> CephPrimitive<'a> for COsdOp<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
            op: call!(ceph_osd_op::read_from_wire) ~
            soid: call!(sobject_t::read_from_wire) ~
            rval: le_i32,
            ||{
                COsdOp{
                    op: op,
                    soid: soid,
                    rval: rval,

                }
            })
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[derive(Debug,Eq,PartialEq)]
pub struct Mosdsubop<'a> {
    pub map_epoch: u32,
    pub reqid: OsdReqidT,
    pub from: PgShardT,
    pub pgid: SpgT,
    pub poid: HObject<'a>,
    pub oloc: ObjectLocator<'a>,
    pub acks_wanted: u8,
    pub ops: Vec<COsdOp<'a>>,
    pub mtime: Utime,
    pub old_exists: u8,
    pub old_size: u64,
    pub old_version: EversionT,
    pub snapset: SnapSet,
    pub snapc: SnapContext,
    pub logbl: &'a [u8],
    pub pg_stats: PgStatT,
    pub version: EversionT,
    pub pg_trim_to: EversionT,
    pub pg_trim_rollback_to: EversionT,
    pub peer_stat: Utime,
    pub attrset: Vec<(&'a str, u64)>,
    pub data_subset: Vec<u64>,
    pub clone_subsets: Vec<(HObject<'a>, Vec<u64>)>,
    pub first: u8,
    pub complete: u8,
    pub data_included: Vec<u64>,
    pub recovery_info: Objectrecoveryinfo<'a>,
    pub recovery_progress: Objectrecoveryprogress<'a>,
    pub current_progress: Objectrecoveryprogress<'a>,
    pub omap_entries: Vec<(&'a str, &'a [u8])>,
    pub omap_header: &'a [u8],
    pub new_temp_oid: HObject<'a>,
    pub discard_temp_oid: HObject<'a>,
    pub updated_hit_set_history: pg_hit_set_history_t,
}

impl<'a> CephPrimitive<'a> for Mosdsubop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(OsdReqidT::read_from_wire) ~
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		poid: call!(HObject::read_from_wire) ~
		oloc: call!(ObjectLocator::read_from_wire) ~
		acks_wanted: le_u8 ~
		count: le_u32 ~
		ops: count!(call!(COsdOp::read_from_wire), count as usize)~
		mtime: call!(Utime::read_from_wire) ~
		old_exists: le_u8 ~
		old_size: le_u64 ~
		old_version: call!(EversionT::read_from_wire) ~
		snapset: call!(SnapSet::read_from_wire) ~
		snapc: call!(SnapContext::read_from_wire) ~
        log_size: le_u32 ~
		logbl: take!(log_size) ~
		pg_stats: call!(PgStatT::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		pg_trim_to: call!(EversionT::read_from_wire) ~
		pg_trim_rollback_to: call!(EversionT::read_from_wire) ~
		peer_stat: call!(Utime::read_from_wire) ~
        attr_count: le_u32 ~
        attrset: count!(
            pair!(
                parse_str,
                le_u64), attr_count as usize) ~
		data_subset: count!(le_u64, count as usize) ~
		count: le_u32 ~
		clone_subsets: count!(pair!(HObject::read_from_wire,count!(le_u64, count as usize)), count as usize) ~
		first: le_u8 ~
		complete: le_u8 ~
		data_included: count!(le_u64, count as usize) ~
		recovery_info: call!(Objectrecoveryinfo::read_from_wire) ~
		recovery_progress: call!(Objectrecoveryprogress::read_from_wire) ~
		current_progress: call!(Objectrecoveryprogress::read_from_wire) ~
		count: le_u32 ~
		omap_entries: count!(pair!(parse_str,take!(10)), count as usize) ~
        omap_header_size: le_u32 ~
		omap_header: take!(omap_header_size) ~
		new_temp_oid: call!(HObject::read_from_wire) ~
		discard_temp_oid: call!(HObject::read_from_wire) ~
		updated_hit_set_history: call!(pg_hit_set_history_t::read_from_wire),
		||{
			Mosdsubop{
			map_epoch: map_epoch,
			reqid: reqid,
			from: from,
			pgid: pgid,
			poid: poid,
			oloc: oloc,
			acks_wanted: acks_wanted,
			ops: ops,
			mtime: mtime,
			old_exists: old_exists,
			old_size: old_size,
			old_version: old_version,
			snapset: snapset,
			snapc: snapc,
			logbl: logbl,
			pg_stats: pg_stats,
			version: version,
			pg_trim_to: pg_trim_to,
			pg_trim_rollback_to: pg_trim_rollback_to,
			peer_stat: peer_stat,
			attrset: attrset,
			data_subset: data_subset,
			clone_subsets: clone_subsets,
			first: first,
			complete: complete,
			data_included: data_included,
			recovery_info: recovery_info,
			recovery_progress: recovery_progress,
			current_progress: current_progress,
			omap_entries: omap_entries,
			omap_header: omap_header,
			new_temp_oid: new_temp_oid,
			discard_temp_oid: discard_temp_oid,
			updated_hit_set_history: updated_hit_set_history,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGTrim() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgtrim::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgtrim() {
    //let bytes = vec![];
    //let result = Mosdpgtrim::write_to_wire();
    //println!("ceph_write_Mosdpgtrim{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgtrim {
    pub epoch: u32,
    pub pgid: SpgT,
    pub trim_to: EversionT,
}

impl<'a> CephPrimitive<'a> for Mosdpgtrim {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		epoch: le_u32 ~
		pgid: call!(SpgT::read_from_wire) ~
		trim_to: call!(EversionT::read_from_wire),
		||{
			Mosdpgtrim{
			epoch: epoch,
			pgid: pgid,
			trim_to: trim_to,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MOSDRepScrub() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdrepscrub::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdrepscrub() {
    //let bytes = vec![];
    //let result = Mosdrepscrub::write_to_wire();
    //println!("ceph_write_Mosdrepscrub{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdrepscrub<'a> {
    pub pgid: SpgT,
    pub scrub_from: EversionT,
    pub scrub_to: EversionT,
    pub map_epoch: u32,
    pub chunky: u8,
    pub start: HObject<'a>,
    pub end: HObject<'a>,
    pub deep: u8,
    pub seed: u32,
}

impl<'a> CephPrimitive<'a> for Mosdrepscrub<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		scrub_from: call!(EversionT::read_from_wire) ~
		scrub_to: call!(EversionT::read_from_wire) ~
		map_epoch: le_u32 ~
		chunky: le_u8 ~
		start: call!(HObject::read_from_wire) ~
		end: call!(HObject::read_from_wire) ~
		deep: le_u8 ~
		seed: le_u32,
		||{
			Mosdrepscrub{
			pgid: pgid,
			scrub_from: scrub_from,
			scrub_to: scrub_to,
			map_epoch: map_epoch,
			chunky: chunky,
			start: start,
			end: end,
			deep: deep,
			seed: seed,
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
// fn test_ceph_read_MOSDECSubOpWriteReply() {
// //let bytes = vec![];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Mosdecsubopwritereply::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mosdecsubopwritereply() {
// //let bytes = vec![];
// //let result = Mosdecsubopwritereply::write_to_wire();
// //println!("ceph_write_Mosdecsubopwritereply{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdecsubopwritereply {
// pub pgid: SpgT,
// pub map_epoch: u32,
// pub op: ECSubWriteReply,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdecsubopwritereply {
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
// chain!(input,
// pgid: call!(SpgT::read_from_wire) ~
// map_epoch: le_u32 ~
// op: call!(ECSubWriteReply::read_from_wire),
// ||{
// Mosdecsubopwritereply{
// pgid: pgid,
// map_epoch: map_epoch,
// op: op,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//
#[test]
fn test_ceph_read_MOSDScrub() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdscrub::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdscrub() {
    //let bytes = vec![];
    //let result = Mosdscrub::write_to_wire();
    //println!("ceph_write_Mosdscrub{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdscrub {
    pub fsid: Uuid,
    pub scrub_pgs: Vec<pg_t>,
    pub repair: u8,
    pub deep: u8,
}

impl<'a> CephPrimitive<'a> for Mosdscrub {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let head_version = 1;
        let compat_version = 1;
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		scrub_pgs: count!(call!(pg_t::read_from_wire), count as usize)~
		repair: le_u8 ~
		deep: le_u8,
		||{
			Mosdscrub{
			fsid: fsid,
			scrub_pgs: scrub_pgs,
			repair: repair,
			deep: deep,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPing() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdping::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdping() {
    //let bytes = vec![];
    //let result = Mosdping::write_to_wire();
    //println!("ceph_write_Mosdping{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdping {
    pub fsid: Uuid,
    pub map_epoch: u32,
    pub peer_as_of_epoch: u32,
    pub op: u8,
    pub peer_stat: Utime,
    pub stamp: Utime,
}

impl<'a> CephPrimitive<'a> for Mosdping {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		map_epoch: le_u32 ~
		peer_as_of_epoch: le_u32 ~
		op: le_u8 ~
		peer_stat: call!(Utime::read_from_wire) ~
		stamp: call!(Utime::read_from_wire),
		||{
			Mosdping{
			fsid: fsid,
			map_epoch: map_epoch,
			peer_as_of_epoch: peer_as_of_epoch,
			op: op,
			peer_stat: peer_stat,
			stamp: stamp,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGPush() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgpush::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpush() {
    //let bytes = vec![];
    //let result = Mosdpgpush::write_to_wire();
    //println!("ceph_write_Mosdpgpush{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpush<'a> {
    pub from: PgShardT,
    pub pgid: SpgT,
    pub map_epoch: u32,
    pub pushes: Vec<Pushop<'a>>,
    pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpush<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		pushes: count!(call!(Pushop::read_from_wire), count as usize) ~
		cost: le_u64,
		||{
			Mosdpgpush{
			from: from,
			pgid: pgid,
			map_epoch: map_epoch,
			pushes: pushes,
			cost: cost,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MRemoveSnaps() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mremovesnaps::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mremovesnaps() {
    //let bytes = vec![];
    //let result = Mremovesnaps::write_to_wire();
    //println!("ceph_write_Mremovesnaps{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mremovesnaps {
    pub snaps: Vec<(i32, Vec<u64>)>,
}

impl<'a> CephPrimitive<'a> for Mremovesnaps {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		snaps: count!(
			pair!(le_i32,
				length_value!(le_u32, le_u64)
			), count as usize) ,
		||{
			Mremovesnaps{
			snaps: snaps,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDAlive() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdalive::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdalive() {
    //let bytes = vec![];
    //let result = Mosdalive::write_to_wire();
    //println!("ceph_write_Mosdalive{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdalive {
    pub want: u32,
}

impl<'a> CephPrimitive<'a> for Mosdalive {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		want: le_u32,
		||{
			Mosdalive{
			want: want,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGPushReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgpushreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpushreply() {
    //let bytes = vec![];
    //let result = Mosdpgpushreply::write_to_wire();
    //println!("ceph_write_Mosdpgpushreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpushreply<'a> {
    pub from: PgShardT,
    pub pgid: SpgT,
    pub map_epoch: u32,
    pub replies: Vec<Pushreplyop<'a>>,
    pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpushreply<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		replies: count!(call!(Pushreplyop::read_from_wire), count as usize) ~
		cost: le_u64,
		||{
			Mosdpgpushreply{
			from: from,
			pgid: pgid,
			map_epoch: map_epoch,
			replies: replies,
			cost: cost,
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
// fn test_ceph_read_MOSDBoot(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Mosdboot::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mosdboot(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = Mosdboot::write_to_wire();
// //println!("ceph_write_Mosdboot{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdboot{
// pub sb: OSDSuperblock,
// pub hb_back_addr: EntityAddr,
// pub hb_front_addr: EntityAddr,
// pub cluster_addr: EntityAddr,
// pub boot_epoch: u32,
// pub metadata: metadata,
// pub osd_features: u64,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdboot{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// sb: call!(OSDSuperblock::read_from_wire) ~
// hb_back_addr: call!(EntityAddr::read_from_wire) ~
// hb_front_addr: call!(EntityAddr::read_from_wire) ~
// cluster_addr: call!(EntityAddr::read_from_wire) ~
// boot_epoch: le_u32 ~
// count: le_u32 ~
// metadata: count!(pair!(parse_str,parse_str), count as usize) ~
// osd_features: le_u64,
// ||{
// Mosdboot{
// sb: sb,
// hb_back_addr: hb_back_addr,
// hb_front_addr: hb_front_addr,
// cluster_addr: cluster_addr,
// boot_epoch: boot_epoch,
// metadata: metadata,
// osd_features: osd_features,
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
fn test_ceph_read_MOSDPGScan() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgscan::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgscan() {
    //let bytes = vec![];
    //let result = Mosdpgscan::write_to_wire();
    //println!("ceph_write_Mosdpgscan{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgscan<'a> {
    pub op: u32,
    pub map_epoch: u32,
    pub query_epoch: u32,
    pub from: PgShardT,
    pub pgid: SpgT,
    pub begin: HObject<'a>,
    pub end: HObject<'a>,
}

impl<'a> CephPrimitive<'a> for Mosdpgscan<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		op: le_u32 ~
		map_epoch: le_u32 ~
		query_epoch: le_u32 ~
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		begin: call!(HObject::read_from_wire) ~
		end: call!(HObject::read_from_wire),
		||{
			Mosdpgscan{
			op: op,
			map_epoch: map_epoch,
			query_epoch: query_epoch,
			from: from,
			pgid: pgid,
			begin: begin,
			end: end,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGPull() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgpull::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpull() {
    //let bytes = vec![];
    //let result = Mosdpgpull::write_to_wire();
    //println!("ceph_write_Mosdpgpull{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpull<'a> {
    pub from: PgShardT,
    pub pgid: SpgT,
    pub map_epoch: u32,
    pub pulls: Vec<Pullop<'a>>,
    pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpull<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		pulls: count!(call!(Pullop::read_from_wire), count as usize) ~
		cost: le_u64,
		||{
			Mosdpgpull{
			from: from,
			pgid: pgid,
			map_epoch: map_epoch,
			pulls: pulls,
			cost: cost,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MOSDMap() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdmap::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdmap() {
    //let bytes = vec![];
    //let result = Mosdmap::write_to_wire();
    //println!("ceph_write_Mosdmap{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdmap<'a> {
    pub fsid: Uuid,
    pub maps: Vec<(u32, &'a [u8])>,
    pub incremental_maps: Vec<(u32, &'a [u8])>,
    pub oldest_map: u32,
    pub newest_map: u32,
}

impl<'a> CephPrimitive<'a> for Mosdmap<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
        count: le_u32 ~
	    maps: count!(pair!(le_u32,take!(10)), count as usize) ~
		count: le_u32 ~
		incremental_maps: count!(pair!(le_u32,take!(10)), count as usize) ~
		oldest_map: le_u32 ~
		newest_map: le_u32,
		||{
			Mosdmap{
			fsid: fsid,
			maps: maps,
			incremental_maps: incremental_maps,
			oldest_map: oldest_map,
			newest_map: newest_map,
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
// fn test_ceph_read_MOSDECSubOpReadReply() {
// //let bytes = vec![];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Mosdecsubopreadreply::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mosdecsubopreadreply() {
// //let bytes = vec![];
// //let result = Mosdecsubopreadreply::write_to_wire();
// //println!("ceph_write_Mosdecsubopreadreply{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdecsubopreadreply {
// pub pgid: SpgT,
// pub map_epoch: u32,
// pub op: ECSubReadReply,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdecsubopreadreply {
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
// chain!(input,
// pgid: call!(SpgT::read_from_wire) ~
// map_epoch: le_u32 ~
// op: call!(ECSubReadReply::read_from_wire),
// ||{
// Mosdecsubopreadreply{
// pgid: pgid,
// map_epoch: map_epoch,
// op: op,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//

#[test]
fn test_ceph_read_MPGStatsAck() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mpgstatsack::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpgstatsack() {
    //let bytes = vec![];
    //let result = Mpgstatsack::write_to_wire();
    //println!("ceph_write_Mpgstatsack{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpgstatsack {
    pub pg_stat: Vec<(PgStatT, (u64, u32))>,
}

impl<'a> CephPrimitive<'a> for Mpgstatsack {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		pg_stat: count!(
            pair!(
                call!(PgStatT::read_from_wire),
                pair!(le_u64, le_u32)), count as usize),
		||{
			Mpgstatsack{
			pg_stat: pg_stat,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDOp() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdop::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdop() {
    //let bytes = vec![];
    //let result = Mosdop::write_to_wire();
    //println!("ceph_write_Mosdop{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdop<'a> {
    pub ops: Vec<COsdOp<'a>>,
}

impl<'a> CephPrimitive<'a> for Mosdop<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		ops: count!(call!(COsdOp::read_from_wire), count as usize),
		||{
			Mosdop{
			ops: ops,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

#[test]
fn test_ceph_read_MGetPoolStats() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mgetpoolstats::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mgetpoolstats() {
    //let bytes = vec![];
    //let result = Mgetpoolstats::write_to_wire();
    //println!("ceph_write_Mgetpoolstats{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mgetpoolstats<'a> {
    pub fsid: Uuid,
    pub pools: Vec<&'a str>,
}

impl<'a> CephPrimitive<'a> for Mgetpoolstats<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pools: count!(parse_str, count as usize),
		||{
			Mgetpoolstats{
			fsid: fsid,
			pools: pools,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDSubOpReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdsubopreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdsubopreply() {
    //let bytes = vec![];
    //let result = Mosdsubopreply::write_to_wire();
    //println!("ceph_write_Mosdsubopreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdsubopreply<'a> {
    pub map_epoch: u32,
    pub reqid: OsdReqidT,
    pub from: PgShardT,
    pub pgid: SpgT,
    pub poid: HObject<'a>,
    pub ops: Vec<COsdOp<'a>>,
    pub ack_type: u8,
    pub result: i32,
    pub last_complete_ondisk: EversionT,
    pub peer_stat: Utime,
    pub attrset: Vec<(&'a str, u64)>,
}

impl<'a> CephPrimitive<'a> for Mosdsubopreply<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        let HEAD_VERSION = 2;
        let COMPAT_VERSION = 1;
        chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(OsdReqidT::read_from_wire) ~
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		poid: call!(HObject::read_from_wire) ~
		count: le_u32 ~
		ops: count!(call!(COsdOp::read_from_wire), count as usize) ~
		ack_type: le_u8 ~
		result: le_i32 ~
		last_complete_ondisk: call!(EversionT::read_from_wire) ~
		peer_stat: call!(Utime::read_from_wire) ~
        attr_count: le_u32 ~
		attrset: count!(
            pair!(
                parse_str,
                le_u64), attr_count as usize) ,
		||{
			Mosdsubopreply{
			map_epoch: map_epoch,
			reqid: reqid,
			from: from,
			pgid: pgid,
			poid: poid,
			ops: ops,
			ack_type: ack_type,
			result: result,
			last_complete_ondisk: last_complete_ondisk,
			peer_stat: peer_stat,
			attrset: attrset,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDMarkMeDown() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdmarkmedown::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdmarkmedown() {
    //let bytes = vec![];
    //let result = Mosdmarkmedown::write_to_wire();
    //println!("ceph_write_Mosdmarkmedown{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdmarkmedown {
    pub fsid: Uuid,
    pub target_osd: EntityInstT,
    pub epoch: u32,
    pub request_ack: u8,
}

impl<'a> CephPrimitive<'a> for Mosdmarkmedown {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		target_osd: call!(EntityInstT::read_from_wire) ~
		epoch: le_u32 ~
		request_ack: le_u8,
		||{
			Mosdmarkmedown{
			fsid: fsid,
			target_osd: target_osd,
			epoch: epoch,
			request_ack: request_ack,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOSDPGRemove() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdpgremove::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgremove() {
    //let bytes = vec![];
    //let result = Mosdpgremove::write_to_wire();
    //println!("ceph_write_Mosdpgremove{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgremove {
    pub pg_list: Vec<SpgT>,
}

impl<'a> CephPrimitive<'a> for Mosdpgremove {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		count: le_u32 ~
		pg_list: count!(call!(SpgT::read_from_wire), count as usize),
		||{
			Mosdpgremove{
			pg_list: pg_list,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}
#[test]
fn test_ceph_read_MOsdRepOpReply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = MOsdRepOpreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_MOsdRepOpreply() {
    //let bytes = vec![];
    //let result = MOsdRepOpreply::write_to_wire();
    //println!("ceph_write_MOsdRepOpreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct MOsdRepOpreply {
    pub map_epoch: u32,
    pub reqid: OsdReqidT,
    pub from: PgShardT,
    pub pgid: SpgT,
    pub ack_type: u8,
    pub result: i32,
    pub last_complete_ondisk: EversionT,
}

impl<'a> CephPrimitive<'a> for MOsdRepOpreply {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(OsdReqidT::read_from_wire) ~
        from: call!(PgShardT::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		ack_type: le_u8 ~
		result: le_i32 ~
		last_complete_ondisk: call!(EversionT::read_from_wire),
		||{
			MOsdRepOpreply{
			map_epoch: map_epoch,
			reqid: reqid,
			from: from,
			pgid: pgid,
			ack_type: ack_type,
			result: result,
			last_complete_ondisk: last_complete_ondisk,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

/*
#[test]
fn test_ceph_read_MOSDECSubOpRead() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mosdecsubopread::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    ////assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdecsubopread() {
    //let bytes = vec![];
    //let result = Mosdecsubopread::write_to_wire();
    //println!("ceph_write_Mosdecsubopread{:?}", result);
    // assert_eq!(result, expected_bytes);
}
*/

//
// #[derive(Debug,Eq,PartialEq)]
// struct ECSubRead {
// pg_shard_t from,
// ceph_tid_t tid,
// map<hobject_t, list<boost::tuple<uint64_t, uint64_t, uint32_t> >, hobject_t::BitwiseComparator> to_read,
// set<hobject_t, hobject_t::BitwiseComparator> attrs_to_read;
// }
//
// #[derive(Debug,Eq,PartialEq)]
// struct ECSubReadReply {
// pg_shard_t from;
// ceph_tid_t tid;
// map<hobject_t, list<pair<uint64_t, bufferlist> >, hobject_t::BitwiseComparator> buffers_read;
// map<hobject_t, map<string, bufferlist>, hobject_t::BitwiseComparator> attrs_read;
// map<hobject_t, int, hobject_t::BitwiseComparator> errors;
// }
//
// #[derive(Debug,Eq,PartialEq)]
// struct ECSubWriteReply {
// pg_shard_t from;
// ceph_tid_t tid;
// eversion_t last_complete;
// bool committed;
// bool applied;
// }
//

//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdecsubopread {
// pub pgid: SpgT,
// pub map_epoch: u32,
// pub op: op,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdecsubopread {
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
// chain!(input,
// pgid: call!(SpgT::read_from_wire) ~
// map_epoch: le_u32 ~
// op: call!(ECSubRead::read_from_wire),
// ||{
// Mosdecsubopread{
// pgid: pgid,
// map_epoch: map_epoch,
// op: op,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//
//
// #[test]
// fn test_ceph_read_MOSDPGLog(){
// //let bytes = vec![
// TODO: fill in test data here
// ];
// let x: &[u8] = &[];
// let expected_result = "";
// //let result = Mosdpglog::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mosdpglog(){
// //let bytes = vec![
// TODO: fill in result data here
// ];
// //let result = Mosdpglog::write_to_wire();
// //println!("ceph_write_Mosdpglog{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdpglog{
// pub to: i8,
// pub from: i8,
// pub info: PgInfoT,
// pub log: PgLogT,
// pub missing: PgMissingTItem,
// pub past_intervals: pg_interval_map_t,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdpglog{
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
// chain!(input,
// to: le_i8 ~
// from: le_i8 ~
// info: call!(PgInfoT::read_from_wire) ~
// log: call!(PgLogT::read_from_wire) ~
// missing: call!(pg_missing_t::read_from_wire) ~
// past_intervals: call!(pg_interval_map_t::read_from_wire),
// ||{
// Mosdpglog{
// to: to,
// from: from,
// info: info,
// log: log,
// missing: missing,
// past_intervals: past_intervals,
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
fn test_ceph_read_mgetpoolstatsreply() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    let expected_result = "";
    //let result = Mgetpoolstatsreply::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    //assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_mgetpoolstatsreply() {
    //let bytes = vec![];
    //let result = Mgetpoolstatsreply::write_to_wire();
    //println!("ceph_write_Mgetpoolstatsreply{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mgetpoolstatsreply<'a> {
    pub fsid: Uuid,
    pub pool_stats: Vec<(&'a str, PoolStatT)>,
}

impl<'a> CephPrimitive<'a> for Mgetpoolstatsreply<'a> {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pool_stats: count!(
            pair!(parse_str, call!(PoolStatT::read_from_wire)),
            count as usize) ,
		||{
			Mgetpoolstatsreply{
			fsid: fsid,
			pool_stats: pool_stats,
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
// fn test_ceph_read_MOSDECSubOpWrite() {
// //let bytes = vec![];
// let x: &[u8] = &[];
// let expected_result = Mosdecsubopwrite {
// };
// //let result = Mosdecsubopwrite::read_from_wire(&bytes);
// //println!("ceph_connect_reply: {:?}", result);
// //assert_eq!(Done(x, expected_result), result);
// }
//
// #[test]
// fn test_ceph_write_Mosdecsubopwrite() {
// //let bytes = vec![];
// //let result = Mosdecsubopwrite::write_to_wire();
// //println!("ceph_write_Mosdecsubopwrite{:?}", result);
// assert_eq!(result, expected_bytes);
// }
//
// #[derive(Debug,Eq,PartialEq)]
// pub struct Mosdecsubopwrite {
// pub pgid: SpgT,
// pub map_epoch: u32,
// pub op: Ecsubwrite,
// }
//
// impl<'a> CephPrimitive<'a> for Mosdecsubopwrite {
// fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
// chain!(input,
// pgid: call!(SpgT::read_from_wire) ~
// map_epoch: le_u32 ~
// op: call!(Ecsubwrite::read_from_wire),
// ||{
// Mosdecsubopwrite{
// pgid: pgid,
// map_epoch: map_epoch,
// op: op,
// }
// })
// }
// fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
// let mut buffer: Vec<u8> = Vec::new();
// return Ok(buffer);
// }
// }
//
#[test]
fn test_ceph_read_mrecoveryreserve() {
    //let bytes = vec![];
    let x: &[u8] = &[];
    // let expected_result = Mrecoveryreserve {
    // };
    //let result = Mrecoveryreserve::read_from_wire(&bytes);
    //println!("ceph_connect_reply: {:?}", result);
    ////assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_mrecoveryreserve() {
    //let bytes = vec![];
    //let result = Mrecoveryreserve::write_to_wire();
    //println!("ceph_write_Mrecoveryreserve{:?}", result);
    // assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mrecoveryreserve {
    pub pgid: SpgT,
    pub query_epoch: u32,
    pub recovery_type: i32,
}

impl<'a> CephPrimitive<'a> for Mrecoveryreserve {
    fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self> {
        chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		query_epoch: le_u32 ~
		recovery_type: le_i32,
		||{
			Mrecoveryreserve{
			pgid: pgid,
			query_epoch: query_epoch,
			recovery_type: recovery_type,
		}
	})
    }
    fn write_to_wire(&self) -> Result<Vec<u8>, SerialError> {
        let mut buffer: Vec<u8> = Vec::new();
        return Ok(buffer);
    }
}

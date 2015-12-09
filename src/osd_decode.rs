#![recursion_limit="9000"]
extern crate nom;
extern crate uuid;

use self::uuid::{ParseError, Uuid};
use self::nom::{le_i8, le_u8, le_i16, le_u16, le_i32, le_u32, le_i64, le_u64, be_u16};
use serial::*;
use common_decode::{EntityNameT, EversionT};

#[test]
fn test_ceph_read_osd_reqid_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = OsdReqidT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_OsdReqidT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = OsdReqidT::write_to_wire();
	println!("ceph_write_OsdReqidT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct OsdReqidT{
	pub name: EntityNameT,
	pub tid: u64,
	pub inc: i32,
}

impl<'a> CephPrimitive<'a> for OsdReqidT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		name: call!(EntityNameT::read_form_wire) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pool_stat_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PoolStatT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PoolStatT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PoolStatT::write_to_wire();
	println!("ceph_write_PoolStatT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PoolStatT{
	pub stats: ObjectStatCollectionT,
	pub log_size: i64,
	pub ondisk_log_size: i64,
	pub up: i32,
	pub acting: i32,
}

impl<'a> CephPrimitive<'a> for PoolStatT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[derive(Debug,Eq,PartialEq)]
pub struct pool_snap_info_t {
  pub snapid_t: u64,
  pub utime_t: Utime,
  pub name: & 'a str,
}

impl<'a> CephPrimitive<'a> for pool_snap_info_t{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
    )}

	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

/*
#[test]
fn test_ceph_read_pg_pool_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgPoolT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgPoolT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgPoolT::write_to_wire();
	println!("ceph_write_PgPoolT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgPoolT{
	pub flags: u64,
	pub pool_type: u8,
	pub size: u8,
	pub min_size: u8,
	pub crush_ruleset: u8,
	pub object_hash: u8,
	pub properties: properties,
	pub erasure_code_profile: &'a str,
	pub last_change: u32,
	pub last_force_op_resend: u32,
	pub snap_seq: snap_seq,
	pub snap_epoch: u32,
	pub auid: u64,
	pub crash_replay_interval: u32,
	pub quota_max_bytes: u64,
	pub quota_max_objects: u64,
	pub snaps: snaps,
	pub removed_snaps: removed_snaps,
	pub pg_num_mask: u32,
	pub pgp_num_mask: u32,
	pub tiers: tiers,
	pub tier_of: i64,
	pub read_tier: i64,
	pub write_tier: i64,
	pub cache_mode: cache_mode,
	pub target_max_bytes: u64,
	pub target_max_objects: u64,
	pub cache_target_dirty_ratio_micro: u32,
	pub cache_target_dirty_high_ratio_micro: u32,
	pub cache_target_full_ratio_micro: u32,
	pub cache_min_flush_age: u32,
	pub cache_min_evict_age: u32,
	pub hit_set_params: hit_set_params,
	pub hit_set_period: u32,
	pub hit_set_count: u32,
	pub use_gmt_hitset: u8,
	pub min_read_recency_for_promote: u32,
	pub min_write_recency_for_promote: u32,
	pub hit_set_grade_decay_rate: u32,
	pub hit_set_search_last_n: u32,
	pub stripe_width: u32,
	pub expected_num_objects: u64,
	pub fast_read: u8,
}

impl<'a> CephPrimitive<'a> for PgPoolT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		flags: le_u64 ~
		pool_type: le_u8 ~
		size: le_u8 ~
		min_size: le_u8 ~
		crush_ruleset: le_u8 ~
		object_hash: le_u8 ~
		count_1: le_u32 ~
		properties: count!(pair!(parse_str,parse_str), count_1) ~
		erasure_code_profile: parse_str ~
		last_change: le_u32 ~
		last_force_op_resend: le_u32 ~
		snap_seq: le_u64 ~
		snap_epoch: le_u32 ~
		auid: le_u64 ~
		crash_replay_interval: le_u32 ~
		quota_max_bytes: le_u64 ~
		quota_max_objects: le_u64 ~
		count_2: le_u32 ~
		snaps: count!(
            pair!(le_u64,
                call!(pool_snap_info_t::read_from_wire)), count_2) ~
		removed_snaps: le_u64 ~
		pg_num_mask: le_u32 ~
		pgp_num_mask: le_u32 ~
		count_3: le_u32 ~
		tiers: count!(le_u64,count_3)~
		tier_of: le_i64 ~
		read_tier: le_i64 ~
		write_tier: le_i64 ~
		cache_mode: call!(cache_mode_t::read_from_wire) ~
		target_max_bytes: le_u64 ~
		target_max_objects: le_u64 ~
		cache_target_dirty_ratio_micro: le_u32 ~
		cache_target_dirty_high_ratio_micro: le_u32 ~
		cache_target_full_ratio_micro: le_u32 ~
		cache_min_flush_age: le_u32 ~
		cache_min_evict_age: le_u32 ~
		hit_set_params: call!(HitSetParams::read_from_wire) ~
		hit_set_period: le_u32 ~
		hit_set_count: le_u32 ~
		use_gmt_hitset: le_u8 ~
		min_read_recency_for_promote: le_u32 ~
		min_write_recency_for_promote: le_u32 ~
		hit_set_grade_decay_rate: le_u32 ~
		hit_set_search_last_n: le_u32 ~
		stripe_width: le_u32 ~
		expected_num_objects: le_u64 ~
		fast_read: le_u8,
		||{
			PgPoolT{
			flags: flags,
			pool_type: pool_type,
			size: size,
			min_size: min_size,
			crush_ruleset: crush_ruleset,
			object_hash: object_hash,
			properties: properties,
			erasure_code_profile: erasure_code_profile,
			last_change: last_change,
			last_force_op_resend: last_force_op_resend,
			snap_seq: snap_seq,
			snap_epoch: snap_epoch,
			auid: auid,
			crash_replay_interval: crash_replay_interval,
			quota_max_bytes: quota_max_bytes,
			quota_max_objects: quota_max_objects,
			snaps: snaps,
			removed_snaps: removed_snaps,
			pg_num_mask: pg_num_mask,
			pgp_num_mask: pgp_num_mask,
			tiers: tiers,
			tier_of: tier_of,
			read_tier: read_tier,
			write_tier: write_tier,
			cache_mode: cache_mode,
			target_max_bytes: target_max_bytes,
			target_max_objects: target_max_objects,
			cache_target_dirty_ratio_micro: cache_target_dirty_ratio_micro,
			cache_target_dirty_high_ratio_micro: cache_target_dirty_high_ratio_micro,
			cache_target_full_ratio_micro: cache_target_full_ratio_micro,
			cache_min_flush_age: cache_min_flush_age,
			cache_min_evict_age: cache_min_evict_age,
			hit_set_params: hit_set_params,
			hit_set_period: hit_set_period,
			hit_set_count: hit_set_count,
			use_gmt_hitset: use_gmt_hitset,
			min_read_recency_for_promote: min_read_recency_for_promote,
			min_write_recency_for_promote: min_write_recency_for_promote,
			hit_set_grade_decay_rate: hit_set_grade_decay_rate,
			hit_set_search_last_n: hit_set_search_last_n,
			stripe_width: stripe_width,
			expected_num_objects: expected_num_objects,
			fast_read: fast_read,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/
#[test]
fn test_ceph_read_pg_shard_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgShardT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgShardT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgShardT::write_to_wire();
	println!("ceph_write_PgShardT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgShardT{
	pub osd: i32,
	pub shard: i8,
}

impl<'a> CephPrimitive<'a> for PgShardT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_SnapSetContext(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Snapsetcontext::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Snapsetcontext(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Snapsetcontext::write_to_wire();
	println!("ceph_write_Snapsetcontext{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Snapsetcontext{
	pub oid: HObject,
	pub snap_ref: i32,
	pub registered: u8,
	pub snapset: SnapSet,
	pub exists: u8,
}

impl<'a> CephPrimitive<'a> for Snapsetcontext{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pg_query_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgQueryT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgQueryT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgQueryT::write_to_wire();
	println!("ceph_write_PgQueryT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgQueryT{
	pub query_type: u32,
	pub since: EversionT,
	pub history: PgHistory,
	pub epoch_sent: u32,
	pub to: i8,
	pub from: i8,
}

impl<'a> CephPrimitive<'a> for PgQueryT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_pg_notify_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgNotifyT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgNotifyT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgNotifyT::write_to_wire();
	println!("ceph_write_PgNotifyT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgNotifyT{
	pub query_epoch: u32,
	pub epoch_sent: u32,
	pub info: PgInfoT,
	pub to: i8,
	pub from: i8,
}

impl<'a> CephPrimitive<'a> for PgNotifyT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

/*
#[test]
fn test_ceph_read_OSDSuperblock(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Osdsuperblock::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Osdsuperblock(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Osdsuperblock::write_to_wire();
	println!("ceph_write_Osdsuperblock{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Osdsuperblock{
	pub cluster_fsid: Uuid,
	pub osd_fsid: Uuid,
	pub whoami: i32,
	pub current_epoch: u32,
	pub oldest_map: u32,
	pub newest_map: u32,
	pub weight: f64,
    //TODO: Decode CompatSet
	pub compat_features: CompatSet,
	pub mounted: u32,
	pub clean_thru: u32,
}

impl<'a> CephPrimitive<'a> for Osdsuperblock{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		cluster_fsid: parse_fsid ~
		osd_fsid: parse_fsid ~
		whoami: le_i32 ~
		current_epoch: le_u32 ~
		oldest_map: le_u32 ~
		newest_map: le_u32 ~
		weight: le_f64 ~
		compat_features: call!(CompatSet::read_from_wire) ~
		mounted: le_u32 ~
		clean_thru: le_u32,
		||{
			Osdsuperblock{
			cluster_fsid: cluster_fsid,
			osd_fsid: osd_fsid,
			whoami: whoami,
			current_epoch: current_epoch,
			oldest_map: oldest_map,
			newest_map: newest_map,
			weight: weight,
			compat_features: compat_features,
			mounted: mounted,
			clean_thru: clean_thru,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/

#[test]
fn test_ceph_read_pg_interval_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgIntervalT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgIntervalT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgIntervalT::write_to_wire();
	println!("ceph_write_PgIntervalT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgIntervalT{
	pub up: Vec<i32>,
	pub acting: Vec<i32>,
	pub first: u32,
	pub last: u32,
	pub maybe_went_rw: u8,
	pub primary: i32,
	pub up_primary: i32,
}

impl<'a> CephPrimitive<'a> for PgIntervalT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		up: count!(le_i32, count)~
		count: le_u32 ~
		acting: count!(le_i32, count)~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_PushReplyOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Pushreplyop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pushreplyop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Pushreplyop::write_to_wire();
	println!("ceph_write_Pushreplyop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pushreplyop{
	pub soid: HObject,
}

impl<'a> CephPrimitive<'a> for Pushreplyop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		soid: call!(HObject::read_from_wire),
		||{
			Pushreplyop{
			soid: soid,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ScrubMap(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Scrubmap::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Scrubmap(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Scrubmap::write_to_wire();
	println!("ceph_write_Scrubmap{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Scrubmap{
	pub valid_through: EversionT,
	pub incr_since: EversionT,
}

impl<'a> CephPrimitive<'a> for Scrubmap{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_obj_list_watch_response_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjListWatchResponseT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjListWatchResponseT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjListWatchResponseT::write_to_wire();
	println!("ceph_write_ObjListWatchResponseT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjListWatchResponseT{
	pub entries: Vec<watch_item_t>,
}

impl<'a> CephPrimitive<'a> for ObjListWatchResponseT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
	count: le_u32 ~
	entries: count!(call!(watch_item_t::read_from_wire),count),
		||{
			ObjListWatchResponseT{
			entries: entries,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

/*
#[test]
fn test_ceph_read_object_stat_sum_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectStatSumT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectStatSumT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectStatSumT::write_to_wire();
	println!("ceph_write_ObjectStatSumT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectStatSumT{
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

impl<'a> CephPrimitive<'a> for ObjectStatSumT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/

#[test]
fn test_ceph_read_object_locator_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectLocatorT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectLocatorT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectLocatorT::write_to_wire();
	println!("ceph_write_ObjectLocatorT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectLocatorT{
	pub pool: i64,
	pub key: &'a str,
	pub nspace: &'a str,
	pub hash: i64,
}

impl<'a> CephPrimitive<'a> for ObjectLocatorT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pg_log_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgLogT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgLogT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgLogT::write_to_wire();
	println!("ceph_write_PgLogT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgLogT{
	pub head: EversionT,
	pub tail: EversionT,
	pub can_rollback_to: EversionT,
	pub rollback_info_trimmed_to: EversionT,
	pub log: Vec<pg_log_entry_t>,
}

impl<'a> CephPrimitive<'a> for PgLogT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		head: call!(EversionT::read_from_wire) ~
		tail: call!(EversionT::read_from_wire) ~
		can_rollback_to: call!(EversionT::read_from_wire) ~
		rollback_info_trimmed_to: call!(EversionT::read_from_wire) ~
		count: le_u32 ~
		log: count!(call!(pg_log_entry_t::read_from_wire),count),
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_hit_set_info_t {
  pub begin: Utime, //< time interval
  pub end: Utime,   //< time interval
  pub version: EversionT,  //< version this HitSet object was written
  pub using_gmt: u8,       //< use gmt for creating the hit_set archive object name
}
impl<'a> CephPrimitive<'a> for pg_hit_set_info_t{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[derive(Debug,Eq,PartialEq)]
pub struct pg_hit_set_history_t {
  pub current_last_update: EversionT,  //< last version inserted into current set
  pub history: Vec<pg_hit_set_info_t>  //< archived sets, sorted oldest -> newest
}

impl<'a> CephPrimitive<'a> for pg_hit_set_history_t{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
          current_last_update: call!(EversionT::read_from_wire) ~
          hit_count: le_u32 ~
          history: count!(pg_hit_set_info_t::read_from_wire, hit_count),
		||{
			pg_hit_set_history_t{
              current_last_update: current_last_update,
              history: history,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_pg_info_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgInfoT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgInfoT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgInfoT::write_to_wire();
	println!("ceph_write_PgInfoT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgInfoT{
	pub pgid: SpgT,
	pub last_update: EversionT,
	pub last_complete: EversionT,
	pub last_epoch_started: u32,
	pub last_user_version: u64,
	pub log_tail: EversionT,
	pub last_backfill: HObject,
	pub last_backfill_bitwise: u8,
	pub purged_snaps: Vec<u64>,
	pub stats: PgStatT,
	pub history: PgHistory,
	pub hit_set: pg_hit_set_history_t,
}

impl<'a> CephPrimitive<'a> for PgInfoT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
		purged_snaps: count!(le_u64, purged_count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pg_missing_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgMissingT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgMissingT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgMissingT::write_to_wire();
	println!("ceph_write_PgMissingT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgMissingT{
	pub rmissing: Vec<(HObject,PgMissingTItem)>,
}

impl<'a> CephPrimitive<'a> for PgMissingT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		rmissing: count!( //A Vec of pair's.  //TODO This should really be a HashMap
            pair!(
                call!(HObject::read_from_wire),
                call!(PgMissingTItem::read_from_wire)), count),
		||{
			PgMissingT{
			rmissing: rmissing,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pool_snap_info_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PoolSnapInfoT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PoolSnapInfoT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PoolSnapInfoT::write_to_wire();
	println!("ceph_write_PoolSnapInfoT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PoolSnapInfoT{
	pub snapid: u64,
	pub stamp: Utime,
	pub name: &'a str,
}

impl<'a> CephPrimitive<'a> for PoolSnapInfoT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_objectstore_perf_stat_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectstorePerfStatT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectstorePerfStatT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectstorePerfStatT::write_to_wire();
	println!("ceph_write_ObjectstorePerfStatT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectstorePerfStatT{
	pub filestore_commit_latency: u32,
	pub filestore_apply_latency: u32,
}

impl<'a> CephPrimitive<'a> for ObjectstorePerfStatT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_PushOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Pushop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pushop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Pushop::write_to_wire();
	println!("ceph_write_Pushop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pushop{
	pub soid: HObject,
	pub version: EversionT,
	pub data: &'a [u8],
	pub data_included: Vec<u64>,
	pub omap_header: &'a [u8],
	pub omap_entries: Vec<(&'a str, &'a [u8])>,
	pub attrset: Vec<(&'a str, &'a [u8])>,
	pub recovery_info: Objectrecoveryinfo,
	pub before_progress: Objectrecoveryprogress,
	pub after_progress: Objectrecoveryprogress,
}

impl<'a> CephPrimitive<'a> for Pushop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
        data_size: le_u32 ~
		data: take!(data_size) ~
        data_count: le_u32 ~
		data_included: count!(le_u64, data_count) ~
        omap_header_size: le_u32 ~
		omap_header: take!(omap_header_size) ~
		count: le_u32 ~
		omap_entries: count!(pair!(parse_str,bufferlist), count) ~
		count: le_u32 ~
		attrset: count!(pair!(parse_str,bufferlist), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

/*
#[test]
fn test_ceph_read_pg_stat_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgStatT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgStatT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgStatT::write_to_wire();
	println!("ceph_write_PgStatT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

//TODO: might need to decode this by hand
#[derive(Debug,Eq,PartialEq)]
pub struct PgStatT{
	pub version: version,
	pub reported_seq: u64,
	pub reported_epoch: u32,
	pub state: u32,
	pub last_fresh: last_fresh,
	pub last_change: last_change,
	pub last_active: last_active,
	pub last_peered: last_peered,
	pub last_clean: last_clean,
	pub last_unstale: last_unstale,
	pub last_undegraded: last_undegraded,
	pub last_fullsized: last_fullsized,
	pub log_start: log_start,
	pub ondisk_log_start: ondisk_log_start,
	pub created: u32,
	pub last_epoch_clean: u32,
	pub parent: parent,
	pub parent_split_bits: u32,
	pub last_scrub: last_scrub,
	pub last_deep_scrub: last_deep_scrub,
	pub last_scrub_stamp: last_scrub_stamp,
	pub last_deep_scrub_stamp: last_deep_scrub_stamp,
	pub last_clean_scrub_stamp: last_clean_scrub_stamp,
	pub stats: stats,
	pub stats_invalid: u8,
	pub log_size: i64,
	pub ondisk_log_size: i64,
	pub up: up,
	pub acting: acting,
	pub mapping_epoch: u32,
	pub blocked_by: blocked_by,
	pub last_became_active: last_became_active,
	pub last_became_peered: last_became_peered,
	pub dirty_stats_invalid: u8,
	pub omap_stats_invalid: u8,
	pub hitset_stats_invalid: u8,
	pub hitset_bytes_stats_invalid: u8,
	pub pin_stats_invalid: u8,
	pub up_primary: i32,
	pub acting_primary: i32,
}

impl<'a> CephPrimitive<'a> for PgStatT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
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
		stats: ObjectStatCollectionT ~
		stats_invalid: le_u8 ~
		log_size: le_i64 ~
		ondisk_log_size: le_i64 ~
		count: le_u32 ~
		up: count!(le_i32, count)~
		count: le_u32 ~
		acting: count!(le_i32, count)~
		mapping_epoch: le_u32 ~
		count: le_u32 ~
		blocked_by: count!(le_i32, count)~
		last_became_active: call!(Utime::read_from_wire) ~
		last_became_peered: call!(Utime::read_from_wire) ~
		dirty_stats_invalid: le_u8 ~
		omap_stats_invalid: le_u8 ~
		hitset_stats_invalid: le_u8 ~
		hitset_bytes_stats_invalid: le_u8 ~
		pin_stats_invalid: le_u8 ~
		up_primary: le_i32 ~
		acting_primary: le_i32 ,
		||{
			PgStatT{
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
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ObjectContextRWState(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectcontextRwstate::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectcontextRwstate(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectcontextRwstate::write_to_wire();
	println!("ceph_write_ObjectcontextRwstate{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectcontextRwstate{
	pub state: State,
	pub count: u64,
	pub waiters: waiters,
	pub recovery_read_marker: u8,
	pub snaptrimmer_write_marker: u8,
}

impl<'a> CephPrimitive<'a> for ObjectcontextRwstate{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		state: call!(State::read_from_wire) ~
		count: le_u64 ~
		count: le_u32 ~
		waiters: count!(call!(OpRequestRef::read_from_wire),count) ~
		recovery_read_marker: le_u8 ~
		snaptrimmer_write_marker: le_u8,
		||{
			ObjectcontextRwstate{
			state: state,
			count: count,
			waiters: waiters,
			recovery_read_marker: recovery_read_marker,
			snaptrimmer_write_marker: snaptrimmer_write_marker,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ScrubMapObject(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ScrubmapObject::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ScrubmapObject(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ScrubmapObject::write_to_wire();
	println!("ceph_write_ScrubmapObject{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ScrubmapObject{
	pub size: u64,
	pub negative: u8,
	pub attrs: attrs,
	pub digest: u32,
	pub digest_present: u8,
	pub nlinks: u32,
	pub snapcolls: snapcolls,
	pub omap_digest: u32,
	pub omap_digest_present: u8,
	pub read_error: u8,
}

impl<'a> CephPrimitive<'a> for ScrubmapObject{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		size: le_u64 ~
		negative: le_u8 ~
		count: le_u32 ~
		attrs: count!(pair!(parse_str,bufferptr), count) ~
		digest: le_u32 ~
		digest_present: le_u8 ~
		nlinks: le_u32 ~
		count: le_u32 ~
		snapcolls: count!(call!(napid_t::read_from_wire),count) ~
		omap_digest: le_u32 ~
		omap_digest_present: le_u8 ~
		read_error: le_u8,
		||{
			ScrubmapObject{
			size: size,
			negative: negative,
			attrs: attrs,
			digest: digest,
			digest_present: digest_present,
			nlinks: nlinks,
			snapcolls: snapcolls,
			omap_digest: omap_digest,
			omap_digest_present: omap_digest_present,
			read_error: read_error,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/

#[test]
fn test_ceph_read_old_pg_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = OldPgT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_OldPgT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = OldPgT::write_to_wire();
	println!("ceph_write_OldPgT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

/*
 * placement group.
 * we encode this into one __le64.
struct ceph_pg {
    __le16 preferred; /* preferred primary osd */
    __le16 ps;        /* placement seed */
    __le32 pool;      /* object pool */
} __attribute__ ((packed));

 */
#[derive(Debug,Eq,PartialEq)]
pub struct OldPgT{
	pub v: u64,
}

impl<'a> CephPrimitive<'a> for OldPgT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		v: le_u64,
		||{
			OldPgT{
			v: v,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_PullOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Pullop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Pullop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Pullop::write_to_wire();
	println!("ceph_write_Pullop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Pullop{
	pub soid: HObject,
	pub recovery_info: Objectrecoveryinfo,
	pub recovery_progress: Objectrecoveryprogress,
}

impl<'a> CephPrimitive<'a> for Pullop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_pg_missing_tItem(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgMissingTItem::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgMissingTItem(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgMissingTItem::write_to_wire();
	println!("ceph_write_PgMissingTItem{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgMissingTItem{
	pub need: Eversion,
	pub have: Eversion,
}

impl<'a> CephPrimitive<'a> for PgMissingTItem{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_watch_item_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = WatchItemT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_WatchItemT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = WatchItemT::write_to_wire();
	println!("ceph_write_WatchItemT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct WatchItemT{
	pub name: EntityNameT,
	pub cookie: u64,
	pub timeout_seconds: u32,
	pub addr: addr,
}

impl<'a> CephPrimitive<'a> for WatchItemT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		name: call!(EntityNameT::read_from_wire) ~
		cookie: le_u64 ~
		timeout_seconds: le_u32 ~
		addr: EntityAddr,
		||{
			WatchItemT{
			name: name,
			cookie: cookie,
			timeout_seconds: timeout_seconds,
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
fn test_ceph_read_notify_info_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = NotifyInfoT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_NotifyInfoT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = NotifyInfoT::write_to_wire();
	println!("ceph_write_NotifyInfoT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct NotifyInfoT{
	pub cookie: u64,
	pub notify_id: u64,
	pub timeout: u32,
	pub bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for NotifyInfoT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_ECSubWrite(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Ecsubwrite::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Ecsubwrite(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Ecsubwrite::write_to_wire();
	println!("ceph_write_Ecsubwrite{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Ecsubwrite{
	pub from: from,
	pub tid: u64,
	pub reqid: reqid,
	pub soid: soid,
	pub stats: stats,
	pub t: t,
	pub at_version: at_version,
	pub trim_to: trim_to,
	pub trim_rollback_to: trim_rollback_to,
	pub log_entries: log_entries,
	pub temp_added: temp_added,
	pub temp_removed: temp_removed,
	pub updated_hit_set_history: updated_hit_set_history,
}

impl<'a> CephPrimitive<'a> for Ecsubwrite{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		tid: le_u64 ~
		reqid: call!(osd_reqid_t::read_from_wire) ~
		soid: call!(HObject::read_from_wire) ~
		stats: call!(PgStatT::read_from_wire) ~
		t: call!(ObjectStore::Transaction::read_from_wire) ~
		at_version: call!(EversionT::read_from_wire) ~
		trim_to: call!(EversionT::read_from_wire) ~
		trim_rollback_to: call!(EversionT::read_from_wire) ~
		count: le_u32 ~
		log_entries: count!(call!(pg_log_entry_t::read_from_wire), count) ~
		count: le_u32 ~
		temp_added: count!(call!(hobject_t, hobject_t::BitwiseComparator::read_from_wire),count) ~
		count: le_u32 ~
		temp_removed: count!(call!(hobject_t, hobject_t::BitwiseComparator::read_from_wire),count) ~
		updated_hit_set_history: opt!(pg_hit_set_history_t::read_from_wire),
		||{
			Ecsubwrite{
			from: from,
			tid: tid,
			reqid: reqid,
			soid: soid,
			stats: stats,
			t: t,
			at_version: at_version,
			trim_to: trim_to,
			trim_rollback_to: trim_rollback_to,
			log_entries: log_entries,
			temp_added: temp_added,
			temp_removed: temp_removed,
			updated_hit_set_history: updated_hit_set_history,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_watch_info_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = WatchInfoT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_WatchInfoT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = WatchInfoT::write_to_wire();
	println!("ceph_write_WatchInfoT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct WatchInfoT{
	pub cookie: u64,
	pub timeout_seconds: u32,
	pub addr: addr,
}

impl<'a> CephPrimitive<'a> for WatchInfoT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		cookie: le_u64 ~
		timeout_seconds: le_u32 ~
		addr: EntityAddr,
		||{
			WatchInfoT{
			cookie: cookie,
			timeout_seconds: timeout_seconds,
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
fn test_ceph_read_ObjectRecoveryProgress(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Objectrecoveryprogress::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectrecoveryprogress(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Objectrecoveryprogress::write_to_wire();
	println!("ceph_write_Objectrecoveryprogress{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectrecoveryprogress{
	pub first: u8,
	pub data_recovered_to: u64,
	pub data_complete: u8,
	pub omap_recovered_to: &'a str,
	pub omap_complete: u8,
}

impl<'a> CephPrimitive<'a> for Objectrecoveryprogress{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_object_info_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectInfoT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectInfoT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectInfoT::write_to_wire();
	println!("ceph_write_ObjectInfoT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectInfoT{
	pub soid: soid,
	pub version: version,
	pub prior_version: prior_version,
	pub user_version: u64,
	pub last_reqid: last_reqid,
	pub size: u64,
	pub mtime: mtime,
	pub local_mtime: local_mtime,
	pub flags: flags,
	pub snaps: Vec<u64>,
	pub truncate_seq: u64,
	pub truncate_size: u64,
	pub watchers: watchers,
	pub data_digest: u32,
	pub omap_digest: u32,
}

impl<'a> CephPrimitive<'a> for ObjectInfoT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		prior_version: call!(EversionT::read_from_wire) ~
		user_version: le_u64 ~
		last_reqid: call!(osd_reqid_t::read_from_wire) ~
		size: le_u64 ~
		mtime: call!(Utime::read_from_wire) ~
		local_mtime: call!(Utime::read_from_wire) ~
		flags: call!(flag_t::read_from_wire) ~
		count: le_u32 ~
		snaps: count!(le_u64, count)~
		truncate_seq: le_u64 ~
		truncate_size: le_u64 ~
		count: le_u32 ~
		watchers: count!(pair!(le_u64,entity_name_t), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_object_stat_collection_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = ObjectStatCollectionT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_ObjectStatCollectionT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = ObjectStatCollectionT::write_to_wire();
	println!("ceph_write_ObjectStatCollectionT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct ObjectStatCollectionT{
	pub sum: sum,
}

impl<'a> CephPrimitive<'a> for ObjectStatCollectionT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		sum: call!(object_stat_sum_t::read_from_wire),
		||{
			ObjectStatCollectionT{
			sum: sum,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ObjectModDesc(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Objectmoddesc::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectmoddesc(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Objectmoddesc::write_to_wire();
	println!("ceph_write_Objectmoddesc{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectmoddesc{
	pub bl: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Objectmoddesc{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
        count: le_u32 ~
		bl: take!(count),
		||{
			Objectmoddesc{
			bl: bl,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_pg_nls_response_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = PgNlsResponseT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_PgNlsResponseT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = PgNlsResponseT::write_to_wire();
	println!("ceph_write_PgNlsResponseT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct PgNlsResponseT{
	pub handle: handle,
	pub entries: entries,
}

impl<'a> CephPrimitive<'a> for PgNlsResponseT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		handle: call!(collection_list_handle_t::read_from_wire) ~
		count: le_u32 ~
		entries: count!(call!(brados::ListObjectImpl::read_from_wire),count),
		||{
			PgNlsResponseT{
			handle: handle,
			entries: entries,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_spg_t(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = SpgT::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_SpgT(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = SpgT::write_to_wire();
	println!("ceph_write_SpgT{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct SpgT{
	pub pgid: pgid,
	pub shard: i8,
}

impl<'a> CephPrimitive<'a> for SpgT{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(pg_t::read_from_wire) ~
		shard: le_i8,
		||{
			SpgT{
			pgid: pgid,
			shard: shard,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_ObjectRecoveryInfo(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Objectrecoveryinfo::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Objectrecoveryinfo(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Objectrecoveryinfo::write_to_wire();
	println!("ceph_write_Objectrecoveryinfo{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Objectrecoveryinfo{
	pub soid: soid,
	pub version: version,
	pub size: u64,
	pub oi: oi,
	pub ss: ss,
	pub copy_subset: copy_subset,
	pub clone_subset: clone_subset,
}

impl<'a> CephPrimitive<'a> for Objectrecoveryinfo{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		soid: call!(HObject::read_from_wire) ~
		version: call!(EversionT::read_from_wire) ~
		size: le_u64 ~
		oi: call!(ObjectInfoT::read_from_wire) ~
		ss: call!(SnapSet::read_from_wire) ~
		copy_subset: count!(le_u64, count) ~
		count: le_u32 ~
		clone_subset: count!(pair!(hobject_t,count!(le_u64, count)), count) ,
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MDataPing(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mdataping::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mdataping(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mdataping::write_to_wire();
	println!("ceph_write_Mdataping{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mdataping{
	pub tag: &'a str,
	pub counter: u32,
	pub mdata_hook: mdata_hook,
	pub mp: mp,
	pub free_data: u8,
}

impl<'a> CephPrimitive<'a> for Mdataping{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	let head_version = 1;
	let compat_version = 1;
	chain!(input,
		tag: parse_str ~
		counter: le_u32 ~
		mdata_hook: call!(mdata_hook_func::read_from_wire) ~
		mp: call!(xio_reg_mem::read_from_wire) ~
		free_data: le_u8,
		||{
			Mdataping{
			tag: tag,
			counter: counter,
			mdata_hook: mdata_hook,
			mp: mp,
			free_data: free_data,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGCreate(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgcreate::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgcreate(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgcreate::write_to_wire();
	println!("ceph_write_Mosdpgcreate{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgcreate{
	pub epoch: u64,
	pub mkpg: mkpg,
	pub ctimes: ctimes,
}

impl<'a> CephPrimitive<'a> for Mosdpgcreate{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	let head_version = 1;
	let compat_version = 1;
	chain!(input,
		epoch: le_u64 ~
		count: le_u32 ~
		mkpg: count!(pair!(g_t,pg_create_t), count) ~
		count: le_u32 ~
		ctimes: count!(pair!(g_t,utime_t), count) ,
		||{
			Mosdpgcreate{
			epoch: epoch,
			mkpg: mkpg,
			ctimes: ctimes,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOsdRepOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = MOsdRepOp::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_MOsdRepOp(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = MOsdRepOp::write_to_wire();
	println!("ceph_write_MOsdRepOp{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct MOsdRepOp{
	pub map_epoch: u32,
	pub reqid: reqid,
	pub from: from,
	pub pgid: pgid,
	pub poid: poid,
	pub acks_wanted: u8,
	pub logbl: &'a [u8],
	pub pg_stats: PgStatTs,
	pub version: version,
	pub pg_trim_to: pg_trim_to,
	pub pg_trim_rollback_to: pg_trim_rollback_to,
	pub new_temp_oid: new_temp_oid,
	pub discard_temp_oid: discard_temp_oid,
	pub updated_hit_set_history: updated_hit_set_history,
}

impl<'a> CephPrimitive<'a> for MOsdRepOp{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(osd_reqid_t::read_from_wire) ~
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
		updated_hit_set_history: opt!(pg_hit_set_history_t::read_from_wire),
		||{
			MOsdRepOp{
			map_epoch: map_epoch,
			reqid: reqid,
			from: from,
			pgid: pgid,
			poid: poid,
			acks_wanted: acks_wanted,
			logbl: logbl,
			pg_stats: PgStatTs,
			version: version,
			pg_trim_to: pg_trim_to,
			pg_trim_rollback_to: pg_trim_rollback_to,
			new_temp_oid: new_temp_oid,
			discard_temp_oid: discard_temp_oid,
			updated_hit_set_history: updated_hit_set_history,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MBackfillReserve(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mbackfillreserve::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mbackfillreserve(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mbackfillreserve::write_to_wire();
	println!("ceph_write_Mbackfillreserve{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mbackfillreserve{
	pub pgid: pgid,
	pub query_epoch: u32,
	pub backfill_type: u32,
	pub priority: u32,
}

impl<'a> CephPrimitive<'a> for Mbackfillreserve{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MWatchNotify(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mwatchnotify::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mwatchnotify(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mwatchnotify::write_to_wire();
	println!("ceph_write_Mwatchnotify{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mwatchnotify{
	pub cookie: u64,
	pub ver: u64,
	pub notify_id: u64,
	pub opcode: u8,
	pub bl: &'a [u8],
	pub return_code: i32,
	pub notifier_gid: u64,
}

impl<'a> CephPrimitive<'a> for Mwatchnotify{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MPGStats(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mpgstats::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpgstats(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mpgstats::write_to_wire();
	println!("ceph_write_Mpgstats{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpgstats{
	pub fsid: Uuid,
	pub pg_stat: PgStatT,
	pub osd_stat: osd_stat,
	pub epoch: u32,
	pub had_map_for: had_map_for,
}

impl<'a> CephPrimitive<'a> for Mpgstats{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pg_stat: count!(pair!(g_t,pg_stat_t), count) ~
		osd_stat: call!(osd_stat_t::read_from_wire) ~
		epoch: le_u32 ~
		had_map_for: Utime ,
		||{
			Mpgstats{
			fsid: fsid,
			pg_stat: PgStatT,
			osd_stat: osd_stat,
			epoch: epoch,
			had_map_for: had_map_for,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGQuery(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgquery::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgquery(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgquery::write_to_wire();
	println!("ceph_write_Mosdpgquery{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgquery{
	pub pg_list: pg_list,
}

impl<'a> CephPrimitive<'a> for Mosdpgquery{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		pg_list: count!(pair!(spg_t,pg_query_t), count) ,
		||{
			Mosdpgquery{
			pg_list: pg_list,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDPGBackfill(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgbackfill::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgbackfill(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgbackfill::write_to_wire();
	println!("ceph_write_Mosdpgbackfill{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgbackfill{
	pub op: u32,
	pub map_epoch: u32,
	pub query_epoch: u32,
	pub pgid: pgid,
	pub last_backfill: last_backfill,
	pub compat_stat_sum: u8,
	pub stats: stats,
}

impl<'a> CephPrimitive<'a> for Mosdpgbackfill{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		op: le_u32 ~
		map_epoch: le_u32 ~
		query_epoch: le_u32 ~
		pgid: call!(SpgT::readom_from_wire) ~
		last_backfill: call!(HObject::readom_from_wire) ~
		compat_stat_sum: le_u8 ~
		stats: call!(PgStatT::readom_from_wire),
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDFailure(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdfailure::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdfailure(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdfailure::write_to_wire();
	println!("ceph_write_Mosdfailure{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdfailure{
	pub fsid: Uuid,
	pub target_osd: target_osd,
	pub is_failed: u8,
	pub epoch: u32,
	pub failed_for: i32,
}

impl<'a> CephPrimitive<'a> for Mosdfailure{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		fsid: parse_fsid ~
		target_osd: call!(entity_inst_t::read_from_wire) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGMissing(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgmissing::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgmissing(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgmissing::write_to_wire();
	println!("ceph_write_Mosdpgmissing{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgmissing{
	pub info: info,
	pub missing: missing,
}

impl<'a> CephPrimitive<'a> for Mosdpgmissing{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		info: PgInfoT ~
		missing: call!(pg_missing_t::read_from_wire),
		||{
			Mosdpgmissing{
			info: info,
			missing: missing,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGTemp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgtemp::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgtemp(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgtemp::write_to_wire();
	println!("ceph_write_Mosdpgtemp{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgtemp{
	pub map_epoch: u32,
	pub pg_temp: pg_temp,
}

impl<'a> CephPrimitive<'a> for Mosdpgtemp{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		map_epoch: le_u32 ~
		count: le_u32 ~
		pg_temp: count!(
			pair!(g_t,
			count!(le_i32, count)), count),
		||{
			Mosdpgtemp{
			map_epoch: map_epoch,
			pg_temp: pg_temp,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MPoolOpReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mpoolopreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpoolopreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mpoolopreply::write_to_wire();
	println!("ceph_write_Mpoolopreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpoolopreply{
	pub fsid: Uuid,
	pub replyCode: u32,
	pub epoch: u32,
	pub response_data: &'a [u8],
}

impl<'a> CephPrimitive<'a> for Mpoolopreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGInfo(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpginfo::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpginfo(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpginfo::write_to_wire();
	println!("ceph_write_Mosdpginfo{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpginfo{
	pub pg_list: pg_list,
}

impl<'a> CephPrimitive<'a> for Mosdpginfo{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		pg_list: count!(call!(pair<pg_notify_t, pg_interval_map_t>::read_from_wire), count),
		||{
			Mosdpginfo{
			pg_list: pg_list,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MPoolOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mpoolop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpoolop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mpoolop::write_to_wire();
	println!("ceph_write_Mpoolop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpoolop{
	pub fsid: Uuid,
	pub pool: u32,
	pub name: &'a str,
	pub op: u32,
	pub auid: u64,
	pub snapid: u64,
	pub crush_rule: i16,
}

impl<'a> CephPrimitive<'a> for Mpoolop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDSubOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdsubop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdsubop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdsubop::write_to_wire();
	println!("ceph_write_Mosdsubop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdsubop{
	pub map_epoch: u32,
	pub reqid: reqid,
	pub from: from,
	pub pgid: pgid,
	pub poid: poid,
	pub oloc: oloc,
	pub acks_wanted: u8,
	pub ops: ops,
	pub mtime: mtime,
	pub old_exists: u8,
	pub old_size: u64,
	pub old_version: old_version,
	pub snapset: snapset,
	pub snapc: snapc,
	pub logbl: &'a [u8],
	pub pg_stats: PgStatTs,
	pub version: version,
	pub pg_trim_to: pg_trim_to,
	pub pg_trim_rollback_to: pg_trim_rollback_to,
	pub peer_stat: Utime,
	pub attrset: &'a [u8],
	pub data_subset: data_subset,
	pub clone_subsets: clone_subsets,
	pub first: u8,
	pub complete: u8,
	pub data_included: data_included,
	pub recovery_info: recovery_info,
	pub recovery_progress: recovery_progress,
	pub current_progress: current_progress,
	pub omap_entries: &'a [u8],
	pub omap_header: &'a [u8],
	pub new_temp_oid: new_temp_oid,
	pub discard_temp_oid: discard_temp_oid,
	pub updated_hit_set_history: updated_hit_set_history,
}

impl<'a> CephPrimitive<'a> for Mosdsubop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(osd_reqid_t::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		poid: call!(HObject::read_from_wire) ~
		oloc: call!(object_locator_t::read_from_wire) ~
		acks_wanted: le_u8 ~
		count: le_u32 ~
		ops: count!(call!(OSDOp::read_from_wire), count)~
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
        //TODO: How does this work?
		count: le_u32 ~
		attrset: count!(
            pair!(parse_str,bufferlist),
            count) ~
		data_subset: count!(le_u64, count) ~
		count: le_u32 ~
		clone_subsets: count!(pair!(hobject_t,count!(le_u64, count)), count) ~
		first: le_u8 ~
		complete: le_u8 ~
		data_included: count!(le_u64, count) ~
		recovery_info: call!(ObjectRecoveryInfo::read_from_wire) ~
		recovery_progress: call!(ObjectRecoveryProgress::read_from_wire) ~
		current_progress: call!(ObjectRecoveryProgress::read_from_wire) ~
		count: le_u32 ~
		omap_entries: count!(pair!(parse_str,bufferlist), count) ~
        omap_header_size: le_u32 ~
		omap_header: take!(omap_header_size) ~
		new_temp_oid: call!(HObject::read_from_wire) ~
		discard_temp_oid: call!(HObject::read_from_wire) ~
		updated_hit_set_history: opt!(pg_hit_set_history_t::read_from_wire),
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
			pg_stats: PgStatTs,
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGTrim(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgtrim::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgtrim(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgtrim::write_to_wire();
	println!("ceph_write_Mosdpgtrim{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgtrim{
	pub epoch: u32,
	pub pgid: SpgT,
	pub trim_to: EversionT,
}

impl<'a> CephPrimitive<'a> for Mosdpgtrim{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDRepScrub(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdrepscrub::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdrepscrub(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdrepscrub::write_to_wire();
	println!("ceph_write_Mosdrepscrub{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdrepscrub{
	pub pgid: SpgT,
	pub scrub_from: EversionT,
	pub scrub_to: EversionT,
	pub map_epoch: u32,
	pub chunky: u8,
	pub start: HObject,
	pub end: HObject,
	pub deep: u8,
	pub seed: u32,
}

impl<'a> CephPrimitive<'a> for Mosdrepscrub{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDECSubOpWriteReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdecsubopwritereply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdecsubopwritereply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdecsubopwritereply::write_to_wire();
	println!("ceph_write_Mosdecsubopwritereply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdecsubopwritereply{
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub op: ECSubWriteReply,
}

impl<'a> CephPrimitive<'a> for Mosdecsubopwritereply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		op: call!(ECSubWriteReply::read_from_wire),
		||{
			Mosdecsubopwritereply{
			pgid: pgid,
			map_epoch: map_epoch,
			op: op,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDScrub(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdscrub::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdscrub(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdscrub::write_to_wire();
	println!("ceph_write_Mosdscrub{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdscrub{
	pub fsid: Uuid,
	pub scrub_pgs: Vec<scrub_pgs>,
	pub repair: u8,
	pub deep: u8,
}

impl<'a> CephPrimitive<'a> for Mosdscrub{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	let head_version = 1;
	let compat_version = 1;
	chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		scrub_pgs: count!(call!(pg_t::read_from_wire), count)~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPing(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdping::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdping(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdping::write_to_wire();
	println!("ceph_write_Mosdping{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdping{
	pub fsid: Uuid,
	pub map_epoch: u32,
	pub peer_as_of_epoch: u32,
	pub op: u8,
	pub peer_stat: Utime,
	pub stamp: Utime,
}

impl<'a> CephPrimitive<'a> for Mosdping{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGPush(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgpush::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpush(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgpush::write_to_wire();
	println!("ceph_write_Mosdpgpush{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpush{
	pub from: from,
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub pushes: Vec<Pushop>,
	pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpush{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		pushes: count!(call!(PushOp::read_from_wire), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MRemoveSnaps(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mremovesnaps::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mremovesnaps(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mremovesnaps::write_to_wire();
	println!("ceph_write_Mremovesnaps{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mremovesnaps{
	pub snaps: HashMap<i32, Vec<u64>>,
}

impl<'a> CephPrimitive<'a> for Mremovesnaps{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		snaps: count!(
			pair!(le_i32,
				length_value!(le_u32, le_u64)
			), count) ,
		||{
			Mremovesnaps{
			snaps: snaps,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDAlive(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdalive::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdalive(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdalive::write_to_wire();
	println!("ceph_write_Mosdalive{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdalive{
	pub want: u32,
}

impl<'a> CephPrimitive<'a> for Mosdalive{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		want: le_u32,
		||{
			Mosdalive{
			want: want,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGPushReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgpushreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpushreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgpushreply::write_to_wire();
	println!("ceph_write_Mosdpgpushreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpushreply{
	pub from: from,
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub replies: Vec<PushReplyOp>,
	pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpushreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		replies: count!(call!(PushReplyOp::read_from_wire), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
/*
#[test]
fn test_ceph_read_MOSDBoot(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdboot::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdboot(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdboot::write_to_wire();
	println!("ceph_write_Mosdboot{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdboot{
	pub sb: OSDSuperblock,
	pub hb_back_addr: EntityAddr,
	pub hb_front_addr: EntityAddr,
	pub cluster_addr: EntityAddr,
	pub boot_epoch: u32,
	pub metadata: metadata,
	pub osd_features: u64,
}

impl<'a> CephPrimitive<'a> for Mosdboot{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		sb: call!(OSDSuperblock::read_from_wire) ~
		hb_back_addr: call!(EntityAddr::read_from_wire) ~
		hb_front_addr: call!(EntityAddr::read_from_wire) ~
		cluster_addr: call!(EntityAddr::read_from_wire) ~
		boot_epoch: le_u32 ~
		count: le_u32 ~
		metadata: count!(pair!(parse_str,parse_str), count) ~
		osd_features: le_u64,
		||{
			Mosdboot{
			sb: sb,
			hb_back_addr: hb_back_addr,
			hb_front_addr: hb_front_addr,
			cluster_addr: cluster_addr,
			boot_epoch: boot_epoch,
			metadata: metadata,
			osd_features: osd_features,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/

#[test]
fn test_ceph_read_MOSDPGScan(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgscan::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgscan(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgscan::write_to_wire();
	println!("ceph_write_Mosdpgscan{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgscan{
	pub op: u32,
	pub map_epoch: u32,
	pub query_epoch: u32,
	pub from: from,
	pub pgid: SpgT,
	pub begin: HObject,
	pub end: HObject,
}

impl<'a> CephPrimitive<'a> for Mosdpgscan{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		op: le_u32 ~
		map_epoch: le_u32 ~
		query_epoch: le_u32 ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGPull(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgpull::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgpull(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgpull::write_to_wire();
	println!("ceph_write_Mosdpgpull{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgpull{
	pub from: from,
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub pulls: Vec<Pullop>,
	pub cost: u64,
}

impl<'a> CephPrimitive<'a> for Mosdpgpull{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		count: le_u32 ~
		pulls: count!(call!(PullOp::read_from_wire), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDMap(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdmap::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdmap(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdmap::write_to_wire();
	println!("ceph_write_Mosdmap{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdmap{
	pub fsid: Uuid,
	pub maps: Vec<(u32, &'a [u8])>,
	pub incremental_maps: Vec<(u32, &'a [u8])>,
	pub oldest_map: u32,
	pub newest_map: u32,
}

impl<'a> CephPrimitive<'a> for Mosdmap{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		fsid: parse_fsid ~
        count: le_u32 ~
	    maps: count!(pair!(le_u32,bufferlist), count) ~
		count: le_u32 ~
		incremental_maps: count!(pair!(le_u32,bufferlist), count) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDECSubOpReadReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdecsubopreadreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdecsubopreadreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdecsubopreadreply::write_to_wire();
	println!("ceph_write_Mosdecsubopreadreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdecsubopreadreply{
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub op: ECSubReadReply,
}

impl<'a> CephPrimitive<'a> for Mosdecsubopreadreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		op: call!(ECSubReadReply::read_from_wire),
		||{
			Mosdecsubopreadreply{
			pgid: pgid,
			map_epoch: map_epoch,
			op: op,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MPGStatsAck(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mpgstatsack::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mpgstatsack(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mpgstatsack::write_to_wire();
	println!("ceph_write_Mpgstatsack{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mpgstatsack{
	pub pg_stat: Vec<(PgStatT, (u64, u32))>,
}

impl<'a> CephPrimitive<'a> for Mpgstatsack{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		pg_stat: count!(
            pair!(
                call!(PgStatT::read_from_wire),
                pair!(le_u64, le_u32)), count),
		||{
			Mpgstatsack{
			pg_stat: PgStatT,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDOp(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdop::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdop(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdop::write_to_wire();
	println!("ceph_write_Mosdop{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdop{
	pub ops: ops,
}

impl<'a> CephPrimitive<'a> for Mosdop{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		ops: count!(call!(OSDOp::read_from_wire), count),
		||{
			Mosdop{
			ops: ops,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MGetPoolStats(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mgetpoolstats::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mgetpoolstats(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mgetpoolstats::write_to_wire();
	println!("ceph_write_Mgetpoolstats{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mgetpoolstats{
	pub fsid: Uuid,
	pub pools: Vec<& 'a str>,
}

impl<'a> CephPrimitive<'a> for Mgetpoolstats{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pools: count!(parse_str, count),
		||{
			Mgetpoolstats{
			fsid: fsid,
			pools: pools,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDSubOpReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdsubopreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdsubopreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdsubopreply::write_to_wire();
	println!("ceph_write_Mosdsubopreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdsubopreply{
	pub map_epoch: u32,
	pub reqid: reqid,
	pub from: from,
	pub pgid: SpgT,
	pub poid: HObject,
	pub ops: Vec<OsdOp>,
	pub ack_type: u8,
	pub result: i32,
	pub last_complete_ondisk: EversionT,
	pub peer_stat: Utime,
	pub attrset: attrset,
}

impl<'a> CephPrimitive<'a> for Mosdsubopreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(osd_reqid_t::read_from_wire) ~
		pgid: call!(SpgT::read_from_wire) ~
		poid: call!(HObject::read_from_wire) ~
		count: le_u32 ~
		ops: count!(call!(OSDOp::read_from_wire), count) ~
		ack_type: le_u8 ~
		result: le_i32 ~
		last_complete_ondisk: call!(EversionT::read_from_wire) ~
		peer_stat: call!(Utime::read_from_wire) ~
		attrset: call!(peer_stat::read_form_wire) ~
		count: le_u32 ~
		count!(pair!(parse_str,bufferptr), count),
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDMarkMeDown(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdmarkmedown::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdmarkmedown(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdmarkmedown::write_to_wire();
	println!("ceph_write_Mosdmarkmedown{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdmarkmedown{
	pub fsid: Uuid,
	pub target_osd: EntityInstT,
	pub epoch: u32,
	pub request_ack: u8,
}

impl<'a> CephPrimitive<'a> for Mosdmarkmedown{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDPGRemove(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpgremove::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpgremove(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpgremove::write_to_wire();
	println!("ceph_write_Mosdpgremove{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpgremove{
	pub pg_list: Vec<SpgT>,
}

impl<'a> CephPrimitive<'a> for Mosdpgremove{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		count: le_u32 ~
		pg_list: count!(call!(SpgT::read_from_wire), count),
		||{
			Mosdpgremove{
			pg_list: pg_list,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOsdRepOpReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = MOsdRepOpreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_MOsdRepOpreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = MOsdRepOpreply::write_to_wire();
	println!("ceph_write_MOsdRepOpreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct MOsdRepOpreply{
	pub map_epoch: u32,
	pub reqid: osd_reqid_t,
	pub from: from,
	pub pgid: SpgT,
	pub ack_type: u8,
	pub result: i32,
	pub last_complete_ondisk: EversionT,
}

impl<'a> CephPrimitive<'a> for MOsdRepOpreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		map_epoch: le_u32 ~
		reqid: call!(osd_reqid_t::read_from_wire) ~
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

#[test]
fn test_ceph_read_MOSDECSubOpRead(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdecsubopread::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdecsubopread(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdecsubopread::write_to_wire();
	println!("ceph_write_Mosdecsubopread{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdecsubopread{
	pub pgid: pgid,
	pub map_epoch: u32,
	pub op: op,
}

impl<'a> CephPrimitive<'a> for Mosdecsubopread{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		op: call!(ECSubRead::read_from_wire),
		||{
			Mosdecsubopread{
			pgid: pgid,
			map_epoch: map_epoch,
			op: op,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
/*
#[test]
fn test_ceph_read_MOSDPGLog(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mosdpglog::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdpglog(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdpglog::write_to_wire();
	println!("ceph_write_Mosdpglog{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdpglog{
	pub to: i8,
	pub from: i8,
	pub info: PgInfoT,
	pub log: PgLogT,
	pub missing: PgMissingTItem,
	pub past_intervals: pg_interval_map_t,
}

impl<'a> CephPrimitive<'a> for Mosdpglog{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		to: le_i8 ~
		from: le_i8 ~
		info: call!(PgInfoT::read_from_wire) ~
		log: call!(PgLogT::read_from_wire) ~
		missing: call!(pg_missing_t::read_from_wire) ~
		past_intervals: call!(pg_interval_map_t::read_from_wire),
		||{
			Mosdpglog{
			to: to,
			from: from,
			info: info,
			log: log,
			missing: missing,
			past_intervals: past_intervals,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
*/
#[test]
fn test_ceph_read_MGetPoolStatsReply(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	let expected_result = "";
	let result = Mgetpoolstatsreply::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mgetpoolstatsreply(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mgetpoolstatsreply::write_to_wire();
	println!("ceph_write_Mgetpoolstatsreply{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mgetpoolstatsreply{
	pub fsid: Uuid,
	pub pool_stats: Vec<(&'a str, PoolStatT)>,
}

impl<'a> CephPrimitive<'a> for Mgetpoolstatsreply{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		fsid: parse_fsid ~
		count: le_u32 ~
		pool_stats: count!(
            pair!(parse_str, call!(PoolStatT::read_from_wire)),
            count) ,
		||{
			Mgetpoolstatsreply{
			fsid: fsid,
			pool_stats: pool_stats,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MOSDECSubOpWrite(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	//let expected_result = Mosdecsubopwrite {
	//};
	let result = Mosdecsubopwrite::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mosdecsubopwrite(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mosdecsubopwrite::write_to_wire();
	println!("ceph_write_Mosdecsubopwrite{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mosdecsubopwrite{
	pub pgid: SpgT,
	pub map_epoch: u32,
	pub op: Ecsubwrite,
}

impl<'a> CephPrimitive<'a> for Mosdecsubopwrite{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
	chain!(input,
		pgid: call!(SpgT::read_from_wire) ~
		map_epoch: le_u32 ~
		op: call!(Ecsubwrite::read_from_wire),
		||{
			Mosdecsubopwrite{
			pgid: pgid,
			map_epoch: map_epoch,
			op: op,
		}
	})
}
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}
#[test]
fn test_ceph_read_MRecoveryReserve(){
	let bytes = vec![
		//TODO: fill in test data here
	];
	let x: &[u8] = &[];
	//let expected_result = Mrecoveryreserve {
	//};
	let result = Mrecoveryreserve::read_from_wire(&bytes);
	println!("ceph_connect_reply: {:?}", result);
	assert_eq!(Done(x, expected_result), result);
}

#[test]
fn test_ceph_write_Mrecoveryreserve(){
	let expected_bytes = vec![
		//TODO: fill in result data here
	];
	let result = Mrecoveryreserve::write_to_wire();
	println!("ceph_write_Mrecoveryreserve{:?}", result);
	//assert_eq!(result, expected_bytes);
}

#[derive(Debug,Eq,PartialEq)]
pub struct Mrecoveryreserve{
	pub pgid: SpgT,
	pub query_epoch: u32,
	pub recovery_type: i32,
}

impl<'a> CephPrimitive<'a> for Mrecoveryreserve{
	fn read_from_wire(input: &'a [u8]) -> nom::IResult<&[u8], Self>{
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
	fn write_to_wire(&self) -> Result<Vec<u8>, SerialError>{
		let mut buffer: Vec<u8> = Vec::new();
		return Ok(buffer);
	}
}

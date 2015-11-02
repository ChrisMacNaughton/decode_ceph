extern crate byteorder;
extern crate crypto as rust_crypto;
extern crate num;
extern crate rand;
extern crate time;
use serial;

use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use self::num::FromPrimitive;
use self::rust_crypto::{ symmetriccipher, buffer, aes, blockmodes };
use self::rust_crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use self::rand::os::OsRng;

use self::rand::Rng;

use std::io::prelude::*;
use std::ops::Add;

static CEPH_AES_IV: &'static str = "cephsageyudagreg";
static AUTH_ENC_MAGIC: &'static str = "0xff009cad8826aa55ull";
/*
  #define CEPH_AES_IV "cephsageyudagreg"
  #define CEPHX_GET_AUTH_SESSION_KEY      0x0100
  #define CEPHX_GET_PRINCIPAL_SESSION_KEY 0x0200
  #define CEPHX_GET_ROTATING_KEY          0x0400

  #define CEPHX_REQUEST_TYPE_MASK            0x0F00
  #define CEPHX_CRYPT_ERR                 1

  Ceph X protocol

  First, the principal has to authenticate with the authenticator. A
  shared-secret mechanism is being used, and the negotitaion goes like this:

  A = Authenticator
  P = Principle
  S = Service

  1. Obtaining principal/auth session key

  (Authenticate Request)
  p->a : principal, principal_addr.  authenticate me!

 ...authenticator does lookup in database...

  a->p : A= {principal/auth session key, validity}^principal_secret (*)
         B= {principal ticket, validity, principal/auth session key}^authsecret


  [principal/auth session key, validity] = service ticket
  [principal ticket, validity, principal/auth session key] = service ticket info

  (*) annotation: ^ signifies 'encrypted by'
  At this point, if is genuine, the principal should have the principal/auth
  session key at hand. The next step would be to request an authorization to
  use some other service:

  2. Obtaining principal/service session key

  p->a : B, {principal_addr, timestamp}^principal/auth session key.  authorize
         me!
  a->p : E= {service ticket}^svcsecret
         F= {principal/service session key, validity}^principal/auth session key

  principal_addr, timestamp = authenticator

  service ticket = principal name, client network address, validity, principal/service session key

  Note that steps 1 and 2 are pretty much the same thing; contacting the
  authenticator and requesting for a key.

  Following this the principal should have a principal/service session key that
  could be used later on for creating a session:

  3. Opening a session to a service

  p->s : E + {principal_addr, timestamp}^principal/service session key
  s->p : {timestamp+1}^principal/service/session key

  timestamp+1 = reply authenticator

  Now, the principal is fully authenticated with the service. So, logically we
  have 2 main actions here. The first one would be to obtain a session key to
  the service (steps 1 and 2), and the second one would be to authenticate with
  the service, using that ticket.
*/
enum MonitorClientState {
    None,
    Negotiating,
    Authenticating,
    HaveSession,
}

pub struct CephXChallengeBlob{
    server_challenge: u64,
    client_challenge: u64,
}

impl CephXChallengeBlob{

    //Start a new challenge
    fn new(self)->Self{
        let mut rand_source = OsRng::new().unwrap();

        return CephXChallengeBlob{
            server_challenge: 0,
            client_challenge: rand_source.next_u64(),
        }
    }
    fn encrypt(secret: Vec<u8>){
        //secret = key

    }
}

struct CephXRequestHeader{
  request_type: u16,
}

struct CephXResponseHeader{
  request_type: u16,
  status: i32,
}

pub struct AuthCapsInfo {
    pub allow_all: bool,
    pub caps: String,
}

impl serial::CephPrimitive for AuthCapsInfo{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, serial::SerialError>{
        //Struct Version
        // let struct_version = try!(cursor.read_u8());
        let _ = try!(cursor.read_u8());

        let allow_all = try!(cursor.read_u8());
        let caps_len = try!(cursor.read_u32::<LittleEndian>());
        let mut caps_buffer:Vec<u8> = Vec::with_capacity(caps_len as usize);
        for _ in 0..caps_len{
            caps_buffer.push(
                try!(cursor.read_u8())
            );
        }

        return Ok(AuthCapsInfo{
            allow_all: allow_all != 0,
            caps: String::from_utf8(caps_buffer).unwrap(),
        });
    }
	fn write_to_wire(&self) -> Result<Vec<u8>, serial::SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        //Struct Version
        try!(buffer.write_u8(1));
        try!(buffer.write_u8(self.allow_all as u8));

        //How do I encode caps?
        try!(buffer.write_u32::<LittleEndian>(self.caps.len() as u32));
        for b in self.caps.as_bytes(){
            try!(buffer.write_u8(b.clone()));
        }

        return Ok(buffer);
    }
}

pub struct AuthTicket {
    pub name: serial::CephEntity,
    pub global_id: u64, /* global instance id */
    pub auid: u64,
    pub created: serial::Utime,
    pub renew_after: serial::Utime,
    pub expires: serial::Utime,
    pub caps: AuthCapsInfo,
    pub flags: u32,
}

impl AuthTicket{
    pub fn new(time_to_live: f32)->AuthTicket{
        let now: time::Timespec = time::now().to_timespec();

        let d = time::Duration::seconds(time_to_live.trunc() as i64);
        let d2 = time::Duration::seconds((time_to_live / 2.0) as i64);

        //now + ttl
        let expire_time_secs = now.add(d);
        let expire_time_nsecs: u32 = time_to_live.fract() as u32 * 1000000000;

        // now + (ttl / 2.0)
        let renew_time_secs =  time::now().to_timespec().add(d2);
        let renew_time_nsecs = (time_to_live.fract() / 2.0) * 1000000000.0;

        return AuthTicket{
            name: serial::CephEntity::Client,
            global_id: 0,
            auid: u64::max_value(),
            //Now
            created: serial::Utime{
                tv_sec: now.sec as u32,
                tv_nsec: now.nsec as u32,
            },
            //renew_after=now +ttl
            renew_after: serial::Utime{
                tv_sec: renew_time_secs.sec as u32,
                tv_nsec: renew_time_nsecs as u32,
            },
            //expires=now += ttl
            expires: serial::Utime{
                tv_sec: expire_time_secs.sec as u32,
                tv_nsec: expire_time_nsecs,
            },
            caps: AuthCapsInfo{
                allow_all: true,
                caps: "".to_string(),
            },
            flags: 0,
        };
    }
}

impl serial::CephPrimitive for AuthTicket{
    fn read_from_wire<R: Read>(cursor: &mut R) -> Result<Self, serial::SerialError>{
        //Struct Version
        // let struct_version = try!(cursor.read_u8());
        let _ = try!(cursor.read_u8());
        let name = serial::CephEntity::from_u8(try!(cursor.read_u8()));

        let global_id = try!(cursor.read_u64::<LittleEndian>());
        let auid = try!(cursor.read_u64::<LittleEndian>());

        let created = try!(serial::Utime::read_from_wire(cursor));
        let renew_after = try!(serial::Utime::read_from_wire(cursor));
        let expires = try!(serial::Utime::read_from_wire(cursor));

        let caps = AuthCapsInfo::read_from_wire(cursor).unwrap();

        return Ok(
            AuthTicket{
                name: name.unwrap(),
                global_id: global_id,
                auid: auid,
                created: created,
                renew_after: renew_after,
                expires: expires,
                caps: caps,
                flags: 0,
            }
        );
    }

	fn write_to_wire(&self) -> Result<Vec<u8>, serial::SerialError>{
        let mut buffer: Vec<u8> = Vec::new();
        //Struct Version
        try!(buffer.write_u8(2));
        try!(buffer.write_u8(self.name.clone() as u8));
        try!(buffer.write_u64::<LittleEndian>(self.global_id));
        try!(buffer.write_u64::<LittleEndian>(self.auid));

        //Send the times
        buffer.extend(try!(self.created.write_to_wire()));
        buffer.extend(try!(self.renew_after.write_to_wire()));
        buffer.extend(try!(self.expires.write_to_wire()));

        return Ok(buffer);
    }
}

pub struct CryptoAesKeyHandler{
    pub encryption_key: String,
    pub description_key: String,
}

pub enum CryptoKeyHandler{
    Aes(CryptoAesKeyHandler),
    None,
}

impl CryptoKeyHandler{
    fn create(self){
        match self{
            CryptoKeyHandler::Aes(ref aes_handler) =>{

            },
            CryptoKeyHandler::None =>{

            }
        }
    }
    fn validate_secret(self){
        match self{
            CryptoKeyHandler::Aes(ref aes_handler) =>{

            },
            CryptoKeyHandler::None =>{

            }
        }
    }
    fn encrypt(self){
        match self{
            CryptoKeyHandler::Aes(ref aes_handler) =>{

            },
            CryptoKeyHandler::None =>{

            }
        }
    }
    fn decrypt(self){
        match self{
            CryptoKeyHandler::Aes(ref aes_handler) =>{

            },
            CryptoKeyHandler::None =>{

            }
        }
    }
}

pub struct AuthHandler{
    state: MonitorClientState,
}

impl AuthHandler{
    fn calculate_client_server_challenge(){
        //client challenge is random bytes
        //encrypt
        let challenge = CephXChallengeBlob::new();

    }
    pub fn authenticate(self){
        match self.state{
            MonitorClientState::Negotiating => {
                /*
                //TODO: Translate me to Rust
                if (!auth || (int)m->protocol != auth->get_protocol()) {
                    delete auth;
                    auth = get_auth_client_handler(cct, m->protocol, rotating_secrets);
                    if (!auth) {
                        ldout(cct, 10) << "no handler for protocol " << m->protocol << dendl;
                        if (m->result == -ENOTSUP) {
                            ldout(cct, 10) << "none of our auth protocols are supported by the server" << dendl;
                            authenticate_err = m->result;
                            auth_cond.SignalAll();
                        }
                        m->put();
                        return;
                    }
                    auth->set_want_keys(want_keys);
                    auth->init(entity_name);
                    auth->set_global_id(global_id);
                    } else {
                        auth->reset();
                    }
                    */

                //TODO: How do I mutate the state machine?
                //self.state = MonitorClientState::Authenticating;
            },
            MonitorClientState::HaveSession => {
                //Already authenticated

            },
            _ => {
                //What should the default case be?
            }
        }
    }
}

pub struct CryptoKey{
    pub key_type: u16,
    pub created: serial::Utime,

    //secret is the keyring secret
    pub secret: Vec<u8>, //what should bufferptr be?
    pub key_handler: CryptoKeyHandler,
}

impl CryptoKey{
    pub fn encode(&self){
        //let data = try!(encrypt(data: &[u8], ceph_key: &[u8]));
    }
    pub fn decode(&self){
        //let data = try!(decrypt(encrypted_data: &[u8], ceph_key: &[u8]));
    }
    pub fn set_secret(&self){ //int type, const bufferptr& s, utime_t createdint type, const bufferptr& s, utime_t created
        /*
        _set_secret {
        404   if (s.length() == 0) {
        405     secret = s;
        406     ckh.reset();
        407     return 0;
        408   }
        409
        410   CryptoHandler *ch = CryptoHandler::create(t);
        411   if (ch) {
        412     int ret = ch->validate_secret(s);
        413     if (ret < 0) {
        414       delete ch;
        415       return ret;
        416     }
        417     string error;
        418     ckh.reset(ch->get_key_handler(s, error));
        419     delete ch;
        420     if (error.length()) {
        421       return -EIO;
        422     }
        423   }
        424   type = t;
        425   secret = s;
        426   return 0;
        }

        395   int r = _set_secret(type, s);
        396   if (r < 0)
        397     return r;
        398   this->created = c;
        399   return 0;
         */
    }
}

//Example taken from rust-crypto/blob/master/examples/symmetriccipher.rs

// Encrypt a buffer with the given key and iv using
// AES-128/CBC/Pkcs encryption.
fn encrypt(data: &[u8], ceph_key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize128,
            ceph_key,
            CEPH_AES_IV.as_bytes(),
            blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
fn decrypt(encrypted_data: &[u8], ceph_key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            ceph_key,
            CEPH_AES_IV.as_bytes(),
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

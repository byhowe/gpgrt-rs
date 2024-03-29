#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage>
{
  storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage>
{
  #[inline]
  pub const fn new(storage: Storage) -> Self
  {
    Self { storage }
  }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
  Storage: AsRef<[u8]> + AsMut<[u8]>,
{
  #[inline]
  pub fn get_bit(&self, index: usize) -> bool
  {
    debug_assert!(index / 8 < self.storage.as_ref().len());
    let byte_index = index / 8;
    let byte = self.storage.as_ref()[byte_index];
    let bit_index = if cfg!(target_endian = "big") {
      7 - (index % 8)
    } else {
      index % 8
    };
    let mask = 1 << bit_index;
    byte & mask == mask
  }

  #[inline]
  pub fn set_bit(&mut self, index: usize, val: bool)
  {
    debug_assert!(index / 8 < self.storage.as_ref().len());
    let byte_index = index / 8;
    let byte = &mut self.storage.as_mut()[byte_index];
    let bit_index = if cfg!(target_endian = "big") {
      7 - (index % 8)
    } else {
      index % 8
    };
    let mask = 1 << bit_index;
    if val {
      *byte |= mask;
    } else {
      *byte &= !mask;
    }
  }

  #[inline]
  pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64
  {
    debug_assert!(bit_width <= 64);
    debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
    debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
    let mut val = 0;
    for i in 0 .. (bit_width as usize) {
      if self.get_bit(i + bit_offset) {
        let index = if cfg!(target_endian = "big") {
          bit_width as usize - 1 - i
        } else {
          i
        };
        val |= 1 << index;
      }
    }
    val
  }

  #[inline]
  pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64)
  {
    debug_assert!(bit_width <= 64);
    debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
    debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
    for i in 0 .. (bit_width as usize) {
      let mask = 1 << i;
      let val_bit_is_set = val & mask == mask;
      let index = if cfg!(target_endian = "big") {
        bit_width as usize - 1 - i
      } else {
        i
      };
      self.set_bit(index + bit_offset, val_bit_is_set);
    }
  }
}
pub const GPG_ERROR_H: i32 = 1;
pub const GPGRT_H: i32 = 1;
pub const GPG_ERROR_VERSION: &'static [u8; 13usize] = b"1.42-unknown\0";
pub const GPGRT_VERSION: &'static [u8; 13usize] = b"1.42-unknown\0";
pub const GPG_ERROR_VERSION_NUMBER: i32 = 76288;
pub const GPGRT_VERSION_NUMBER: i32 = 76288;
pub const GPG_ERR_SYSTEM_ERROR: i32 = 32768;
pub const GPG_ERR_SOURCE_SHIFT: i32 = 24;
pub const GPGRT_HAVE_MACRO_FUNCTION: i32 = 1;
pub const GPG_ERR_INITIALIZED: i32 = 1;
pub const GPGRT_LOG_WITH_PREFIX: i32 = 1;
pub const GPGRT_LOG_WITH_TIME: i32 = 2;
pub const GPGRT_LOG_WITH_PID: i32 = 4;
pub const GPGRT_LOG_RUN_DETACHED: i32 = 256;
pub const GPGRT_LOG_NO_REGISTRY: i32 = 512;
pub const GPGRT_SPAWN_NONBLOCK: i32 = 16;
pub const GPGRT_SPAWN_RUN_ASFW: i32 = 64;
pub const GPGRT_SPAWN_DETACHED: i32 = 128;
pub const GPGRT_CONFDIR_USER: i32 = 1;
pub const GPGRT_CONFDIR_SYS: i32 = 2;
pub type va_list = __builtin_va_list;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker
{
  _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt
{
  _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data
{
  _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE
{
  pub _flags: ::std::os::raw::c_int,
  pub _IO_read_ptr: *mut ::std::os::raw::c_char,
  pub _IO_read_end: *mut ::std::os::raw::c_char,
  pub _IO_read_base: *mut ::std::os::raw::c_char,
  pub _IO_write_base: *mut ::std::os::raw::c_char,
  pub _IO_write_ptr: *mut ::std::os::raw::c_char,
  pub _IO_write_end: *mut ::std::os::raw::c_char,
  pub _IO_buf_base: *mut ::std::os::raw::c_char,
  pub _IO_buf_end: *mut ::std::os::raw::c_char,
  pub _IO_save_base: *mut ::std::os::raw::c_char,
  pub _IO_backup_base: *mut ::std::os::raw::c_char,
  pub _IO_save_end: *mut ::std::os::raw::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: ::std::os::raw::c_int,
  pub _flags2: ::std::os::raw::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: ::std::os::raw::c_ushort,
  pub _vtable_offset: ::std::os::raw::c_schar,
  pub _shortbuf: [::std::os::raw::c_char; 1usize],
  pub _lock: *mut _IO_lock_t,
  pub _offset: __off64_t,
  pub _codecvt: *mut _IO_codecvt,
  pub _wide_data: *mut _IO_wide_data,
  pub _freeres_list: *mut _IO_FILE,
  pub _freeres_buf: *mut ::std::os::raw::c_void,
  pub __pad5: usize,
  pub _mode: ::std::os::raw::c_int,
  pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE()
{
  assert_eq!(
    ::std::mem::size_of::<_IO_FILE>(),
    216usize,
    concat!("Size of: ", stringify!(_IO_FILE))
  );
  assert_eq!(
    ::std::mem::align_of::<_IO_FILE>(),
    8usize,
    concat!("Alignment of ", stringify!(_IO_FILE))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
    0usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_flags))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_read_ptr)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_read_end)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_read_base)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_write_base)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
    40usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_write_ptr)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
    48usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_write_end)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
    56usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_buf_base)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
    64usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_IO_buf_end))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
    72usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_save_base)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
    80usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_backup_base)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
    88usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_IO_save_end)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
    96usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_markers))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
    104usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_chain))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
    112usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_fileno))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
    116usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_flags2))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
    120usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_old_offset))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
    128usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_cur_column))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
    130usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_vtable_offset)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
    131usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_shortbuf))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
    136usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_lock))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
    144usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_offset))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._codecvt as *const _ as usize },
    152usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_codecvt))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._wide_data as *const _ as usize },
    160usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_wide_data))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._freeres_list as *const _ as usize },
    168usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_freeres_list)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._freeres_buf as *const _ as usize },
    176usize,
    concat!(
      "Offset of field: ",
      stringify!(_IO_FILE),
      "::",
      stringify!(_freeres_buf)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
    184usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(__pad5))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
    192usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_mode))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
    196usize,
    concat!("Offset of field: ", stringify!(_IO_FILE), "::", stringify!(_unused2))
  );
}
pub const GPG_ERR_SOURCE_UNKNOWN: gpg_err_source_t = 0;
pub const GPG_ERR_SOURCE_GCRYPT: gpg_err_source_t = 1;
pub const GPG_ERR_SOURCE_GPG: gpg_err_source_t = 2;
pub const GPG_ERR_SOURCE_GPGSM: gpg_err_source_t = 3;
pub const GPG_ERR_SOURCE_GPGAGENT: gpg_err_source_t = 4;
pub const GPG_ERR_SOURCE_PINENTRY: gpg_err_source_t = 5;
pub const GPG_ERR_SOURCE_SCD: gpg_err_source_t = 6;
pub const GPG_ERR_SOURCE_GPGME: gpg_err_source_t = 7;
pub const GPG_ERR_SOURCE_KEYBOX: gpg_err_source_t = 8;
pub const GPG_ERR_SOURCE_KSBA: gpg_err_source_t = 9;
pub const GPG_ERR_SOURCE_DIRMNGR: gpg_err_source_t = 10;
pub const GPG_ERR_SOURCE_GSTI: gpg_err_source_t = 11;
pub const GPG_ERR_SOURCE_GPA: gpg_err_source_t = 12;
pub const GPG_ERR_SOURCE_KLEO: gpg_err_source_t = 13;
pub const GPG_ERR_SOURCE_G13: gpg_err_source_t = 14;
pub const GPG_ERR_SOURCE_ASSUAN: gpg_err_source_t = 15;
pub const GPG_ERR_SOURCE_TPM2D: gpg_err_source_t = 16;
pub const GPG_ERR_SOURCE_TLS: gpg_err_source_t = 17;
pub const GPG_ERR_SOURCE_ANY: gpg_err_source_t = 31;
pub const GPG_ERR_SOURCE_USER_1: gpg_err_source_t = 32;
pub const GPG_ERR_SOURCE_USER_2: gpg_err_source_t = 33;
pub const GPG_ERR_SOURCE_USER_3: gpg_err_source_t = 34;
pub const GPG_ERR_SOURCE_USER_4: gpg_err_source_t = 35;
pub const GPG_ERR_SOURCE_DIM: gpg_err_source_t = 128;
pub type gpg_err_source_t = ::std::os::raw::c_uint;
pub const GPG_ERR_NO_ERROR: gpg_err_code_t = 0;
pub const GPG_ERR_GENERAL: gpg_err_code_t = 1;
pub const GPG_ERR_UNKNOWN_PACKET: gpg_err_code_t = 2;
pub const GPG_ERR_UNKNOWN_VERSION: gpg_err_code_t = 3;
pub const GPG_ERR_PUBKEY_ALGO: gpg_err_code_t = 4;
pub const GPG_ERR_DIGEST_ALGO: gpg_err_code_t = 5;
pub const GPG_ERR_BAD_PUBKEY: gpg_err_code_t = 6;
pub const GPG_ERR_BAD_SECKEY: gpg_err_code_t = 7;
pub const GPG_ERR_BAD_SIGNATURE: gpg_err_code_t = 8;
pub const GPG_ERR_NO_PUBKEY: gpg_err_code_t = 9;
pub const GPG_ERR_CHECKSUM: gpg_err_code_t = 10;
pub const GPG_ERR_BAD_PASSPHRASE: gpg_err_code_t = 11;
pub const GPG_ERR_CIPHER_ALGO: gpg_err_code_t = 12;
pub const GPG_ERR_KEYRING_OPEN: gpg_err_code_t = 13;
pub const GPG_ERR_INV_PACKET: gpg_err_code_t = 14;
pub const GPG_ERR_INV_ARMOR: gpg_err_code_t = 15;
pub const GPG_ERR_NO_USER_ID: gpg_err_code_t = 16;
pub const GPG_ERR_NO_SECKEY: gpg_err_code_t = 17;
pub const GPG_ERR_WRONG_SECKEY: gpg_err_code_t = 18;
pub const GPG_ERR_BAD_KEY: gpg_err_code_t = 19;
pub const GPG_ERR_COMPR_ALGO: gpg_err_code_t = 20;
pub const GPG_ERR_NO_PRIME: gpg_err_code_t = 21;
pub const GPG_ERR_NO_ENCODING_METHOD: gpg_err_code_t = 22;
pub const GPG_ERR_NO_ENCRYPTION_SCHEME: gpg_err_code_t = 23;
pub const GPG_ERR_NO_SIGNATURE_SCHEME: gpg_err_code_t = 24;
pub const GPG_ERR_INV_ATTR: gpg_err_code_t = 25;
pub const GPG_ERR_NO_VALUE: gpg_err_code_t = 26;
pub const GPG_ERR_NOT_FOUND: gpg_err_code_t = 27;
pub const GPG_ERR_VALUE_NOT_FOUND: gpg_err_code_t = 28;
pub const GPG_ERR_SYNTAX: gpg_err_code_t = 29;
pub const GPG_ERR_BAD_MPI: gpg_err_code_t = 30;
pub const GPG_ERR_INV_PASSPHRASE: gpg_err_code_t = 31;
pub const GPG_ERR_SIG_CLASS: gpg_err_code_t = 32;
pub const GPG_ERR_RESOURCE_LIMIT: gpg_err_code_t = 33;
pub const GPG_ERR_INV_KEYRING: gpg_err_code_t = 34;
pub const GPG_ERR_TRUSTDB: gpg_err_code_t = 35;
pub const GPG_ERR_BAD_CERT: gpg_err_code_t = 36;
pub const GPG_ERR_INV_USER_ID: gpg_err_code_t = 37;
pub const GPG_ERR_UNEXPECTED: gpg_err_code_t = 38;
pub const GPG_ERR_TIME_CONFLICT: gpg_err_code_t = 39;
pub const GPG_ERR_KEYSERVER: gpg_err_code_t = 40;
pub const GPG_ERR_WRONG_PUBKEY_ALGO: gpg_err_code_t = 41;
pub const GPG_ERR_TRIBUTE_TO_D_A: gpg_err_code_t = 42;
pub const GPG_ERR_WEAK_KEY: gpg_err_code_t = 43;
pub const GPG_ERR_INV_KEYLEN: gpg_err_code_t = 44;
pub const GPG_ERR_INV_ARG: gpg_err_code_t = 45;
pub const GPG_ERR_BAD_URI: gpg_err_code_t = 46;
pub const GPG_ERR_INV_URI: gpg_err_code_t = 47;
pub const GPG_ERR_NETWORK: gpg_err_code_t = 48;
pub const GPG_ERR_UNKNOWN_HOST: gpg_err_code_t = 49;
pub const GPG_ERR_SELFTEST_FAILED: gpg_err_code_t = 50;
pub const GPG_ERR_NOT_ENCRYPTED: gpg_err_code_t = 51;
pub const GPG_ERR_NOT_PROCESSED: gpg_err_code_t = 52;
pub const GPG_ERR_UNUSABLE_PUBKEY: gpg_err_code_t = 53;
pub const GPG_ERR_UNUSABLE_SECKEY: gpg_err_code_t = 54;
pub const GPG_ERR_INV_VALUE: gpg_err_code_t = 55;
pub const GPG_ERR_BAD_CERT_CHAIN: gpg_err_code_t = 56;
pub const GPG_ERR_MISSING_CERT: gpg_err_code_t = 57;
pub const GPG_ERR_NO_DATA: gpg_err_code_t = 58;
pub const GPG_ERR_BUG: gpg_err_code_t = 59;
pub const GPG_ERR_NOT_SUPPORTED: gpg_err_code_t = 60;
pub const GPG_ERR_INV_OP: gpg_err_code_t = 61;
pub const GPG_ERR_TIMEOUT: gpg_err_code_t = 62;
pub const GPG_ERR_INTERNAL: gpg_err_code_t = 63;
pub const GPG_ERR_EOF_GCRYPT: gpg_err_code_t = 64;
pub const GPG_ERR_INV_OBJ: gpg_err_code_t = 65;
pub const GPG_ERR_TOO_SHORT: gpg_err_code_t = 66;
pub const GPG_ERR_TOO_LARGE: gpg_err_code_t = 67;
pub const GPG_ERR_NO_OBJ: gpg_err_code_t = 68;
pub const GPG_ERR_NOT_IMPLEMENTED: gpg_err_code_t = 69;
pub const GPG_ERR_CONFLICT: gpg_err_code_t = 70;
pub const GPG_ERR_INV_CIPHER_MODE: gpg_err_code_t = 71;
pub const GPG_ERR_INV_FLAG: gpg_err_code_t = 72;
pub const GPG_ERR_INV_HANDLE: gpg_err_code_t = 73;
pub const GPG_ERR_TRUNCATED: gpg_err_code_t = 74;
pub const GPG_ERR_INCOMPLETE_LINE: gpg_err_code_t = 75;
pub const GPG_ERR_INV_RESPONSE: gpg_err_code_t = 76;
pub const GPG_ERR_NO_AGENT: gpg_err_code_t = 77;
pub const GPG_ERR_AGENT: gpg_err_code_t = 78;
pub const GPG_ERR_INV_DATA: gpg_err_code_t = 79;
pub const GPG_ERR_ASSUAN_SERVER_FAULT: gpg_err_code_t = 80;
pub const GPG_ERR_ASSUAN: gpg_err_code_t = 81;
pub const GPG_ERR_INV_SESSION_KEY: gpg_err_code_t = 82;
pub const GPG_ERR_INV_SEXP: gpg_err_code_t = 83;
pub const GPG_ERR_UNSUPPORTED_ALGORITHM: gpg_err_code_t = 84;
pub const GPG_ERR_NO_PIN_ENTRY: gpg_err_code_t = 85;
pub const GPG_ERR_PIN_ENTRY: gpg_err_code_t = 86;
pub const GPG_ERR_BAD_PIN: gpg_err_code_t = 87;
pub const GPG_ERR_INV_NAME: gpg_err_code_t = 88;
pub const GPG_ERR_BAD_DATA: gpg_err_code_t = 89;
pub const GPG_ERR_INV_PARAMETER: gpg_err_code_t = 90;
pub const GPG_ERR_WRONG_CARD: gpg_err_code_t = 91;
pub const GPG_ERR_NO_DIRMNGR: gpg_err_code_t = 92;
pub const GPG_ERR_DIRMNGR: gpg_err_code_t = 93;
pub const GPG_ERR_CERT_REVOKED: gpg_err_code_t = 94;
pub const GPG_ERR_NO_CRL_KNOWN: gpg_err_code_t = 95;
pub const GPG_ERR_CRL_TOO_OLD: gpg_err_code_t = 96;
pub const GPG_ERR_LINE_TOO_LONG: gpg_err_code_t = 97;
pub const GPG_ERR_NOT_TRUSTED: gpg_err_code_t = 98;
pub const GPG_ERR_CANCELED: gpg_err_code_t = 99;
pub const GPG_ERR_BAD_CA_CERT: gpg_err_code_t = 100;
pub const GPG_ERR_CERT_EXPIRED: gpg_err_code_t = 101;
pub const GPG_ERR_CERT_TOO_YOUNG: gpg_err_code_t = 102;
pub const GPG_ERR_UNSUPPORTED_CERT: gpg_err_code_t = 103;
pub const GPG_ERR_UNKNOWN_SEXP: gpg_err_code_t = 104;
pub const GPG_ERR_UNSUPPORTED_PROTECTION: gpg_err_code_t = 105;
pub const GPG_ERR_CORRUPTED_PROTECTION: gpg_err_code_t = 106;
pub const GPG_ERR_AMBIGUOUS_NAME: gpg_err_code_t = 107;
pub const GPG_ERR_CARD: gpg_err_code_t = 108;
pub const GPG_ERR_CARD_RESET: gpg_err_code_t = 109;
pub const GPG_ERR_CARD_REMOVED: gpg_err_code_t = 110;
pub const GPG_ERR_INV_CARD: gpg_err_code_t = 111;
pub const GPG_ERR_CARD_NOT_PRESENT: gpg_err_code_t = 112;
pub const GPG_ERR_NO_PKCS15_APP: gpg_err_code_t = 113;
pub const GPG_ERR_NOT_CONFIRMED: gpg_err_code_t = 114;
pub const GPG_ERR_CONFIGURATION: gpg_err_code_t = 115;
pub const GPG_ERR_NO_POLICY_MATCH: gpg_err_code_t = 116;
pub const GPG_ERR_INV_INDEX: gpg_err_code_t = 117;
pub const GPG_ERR_INV_ID: gpg_err_code_t = 118;
pub const GPG_ERR_NO_SCDAEMON: gpg_err_code_t = 119;
pub const GPG_ERR_SCDAEMON: gpg_err_code_t = 120;
pub const GPG_ERR_UNSUPPORTED_PROTOCOL: gpg_err_code_t = 121;
pub const GPG_ERR_BAD_PIN_METHOD: gpg_err_code_t = 122;
pub const GPG_ERR_CARD_NOT_INITIALIZED: gpg_err_code_t = 123;
pub const GPG_ERR_UNSUPPORTED_OPERATION: gpg_err_code_t = 124;
pub const GPG_ERR_WRONG_KEY_USAGE: gpg_err_code_t = 125;
pub const GPG_ERR_NOTHING_FOUND: gpg_err_code_t = 126;
pub const GPG_ERR_WRONG_BLOB_TYPE: gpg_err_code_t = 127;
pub const GPG_ERR_MISSING_VALUE: gpg_err_code_t = 128;
pub const GPG_ERR_HARDWARE: gpg_err_code_t = 129;
pub const GPG_ERR_PIN_BLOCKED: gpg_err_code_t = 130;
pub const GPG_ERR_USE_CONDITIONS: gpg_err_code_t = 131;
pub const GPG_ERR_PIN_NOT_SYNCED: gpg_err_code_t = 132;
pub const GPG_ERR_INV_CRL: gpg_err_code_t = 133;
pub const GPG_ERR_BAD_BER: gpg_err_code_t = 134;
pub const GPG_ERR_INV_BER: gpg_err_code_t = 135;
pub const GPG_ERR_ELEMENT_NOT_FOUND: gpg_err_code_t = 136;
pub const GPG_ERR_IDENTIFIER_NOT_FOUND: gpg_err_code_t = 137;
pub const GPG_ERR_INV_TAG: gpg_err_code_t = 138;
pub const GPG_ERR_INV_LENGTH: gpg_err_code_t = 139;
pub const GPG_ERR_INV_KEYINFO: gpg_err_code_t = 140;
pub const GPG_ERR_UNEXPECTED_TAG: gpg_err_code_t = 141;
pub const GPG_ERR_NOT_DER_ENCODED: gpg_err_code_t = 142;
pub const GPG_ERR_NO_CMS_OBJ: gpg_err_code_t = 143;
pub const GPG_ERR_INV_CMS_OBJ: gpg_err_code_t = 144;
pub const GPG_ERR_UNKNOWN_CMS_OBJ: gpg_err_code_t = 145;
pub const GPG_ERR_UNSUPPORTED_CMS_OBJ: gpg_err_code_t = 146;
pub const GPG_ERR_UNSUPPORTED_ENCODING: gpg_err_code_t = 147;
pub const GPG_ERR_UNSUPPORTED_CMS_VERSION: gpg_err_code_t = 148;
pub const GPG_ERR_UNKNOWN_ALGORITHM: gpg_err_code_t = 149;
pub const GPG_ERR_INV_ENGINE: gpg_err_code_t = 150;
pub const GPG_ERR_PUBKEY_NOT_TRUSTED: gpg_err_code_t = 151;
pub const GPG_ERR_DECRYPT_FAILED: gpg_err_code_t = 152;
pub const GPG_ERR_KEY_EXPIRED: gpg_err_code_t = 153;
pub const GPG_ERR_SIG_EXPIRED: gpg_err_code_t = 154;
pub const GPG_ERR_ENCODING_PROBLEM: gpg_err_code_t = 155;
pub const GPG_ERR_INV_STATE: gpg_err_code_t = 156;
pub const GPG_ERR_DUP_VALUE: gpg_err_code_t = 157;
pub const GPG_ERR_MISSING_ACTION: gpg_err_code_t = 158;
pub const GPG_ERR_MODULE_NOT_FOUND: gpg_err_code_t = 159;
pub const GPG_ERR_INV_OID_STRING: gpg_err_code_t = 160;
pub const GPG_ERR_INV_TIME: gpg_err_code_t = 161;
pub const GPG_ERR_INV_CRL_OBJ: gpg_err_code_t = 162;
pub const GPG_ERR_UNSUPPORTED_CRL_VERSION: gpg_err_code_t = 163;
pub const GPG_ERR_INV_CERT_OBJ: gpg_err_code_t = 164;
pub const GPG_ERR_UNKNOWN_NAME: gpg_err_code_t = 165;
pub const GPG_ERR_LOCALE_PROBLEM: gpg_err_code_t = 166;
pub const GPG_ERR_NOT_LOCKED: gpg_err_code_t = 167;
pub const GPG_ERR_PROTOCOL_VIOLATION: gpg_err_code_t = 168;
pub const GPG_ERR_INV_MAC: gpg_err_code_t = 169;
pub const GPG_ERR_INV_REQUEST: gpg_err_code_t = 170;
pub const GPG_ERR_UNKNOWN_EXTN: gpg_err_code_t = 171;
pub const GPG_ERR_UNKNOWN_CRIT_EXTN: gpg_err_code_t = 172;
pub const GPG_ERR_LOCKED: gpg_err_code_t = 173;
pub const GPG_ERR_UNKNOWN_OPTION: gpg_err_code_t = 174;
pub const GPG_ERR_UNKNOWN_COMMAND: gpg_err_code_t = 175;
pub const GPG_ERR_NOT_OPERATIONAL: gpg_err_code_t = 176;
pub const GPG_ERR_NO_PASSPHRASE: gpg_err_code_t = 177;
pub const GPG_ERR_NO_PIN: gpg_err_code_t = 178;
pub const GPG_ERR_NOT_ENABLED: gpg_err_code_t = 179;
pub const GPG_ERR_NO_ENGINE: gpg_err_code_t = 180;
pub const GPG_ERR_MISSING_KEY: gpg_err_code_t = 181;
pub const GPG_ERR_TOO_MANY: gpg_err_code_t = 182;
pub const GPG_ERR_LIMIT_REACHED: gpg_err_code_t = 183;
pub const GPG_ERR_NOT_INITIALIZED: gpg_err_code_t = 184;
pub const GPG_ERR_MISSING_ISSUER_CERT: gpg_err_code_t = 185;
pub const GPG_ERR_NO_KEYSERVER: gpg_err_code_t = 186;
pub const GPG_ERR_INV_CURVE: gpg_err_code_t = 187;
pub const GPG_ERR_UNKNOWN_CURVE: gpg_err_code_t = 188;
pub const GPG_ERR_DUP_KEY: gpg_err_code_t = 189;
pub const GPG_ERR_AMBIGUOUS: gpg_err_code_t = 190;
pub const GPG_ERR_NO_CRYPT_CTX: gpg_err_code_t = 191;
pub const GPG_ERR_WRONG_CRYPT_CTX: gpg_err_code_t = 192;
pub const GPG_ERR_BAD_CRYPT_CTX: gpg_err_code_t = 193;
pub const GPG_ERR_CRYPT_CTX_CONFLICT: gpg_err_code_t = 194;
pub const GPG_ERR_BROKEN_PUBKEY: gpg_err_code_t = 195;
pub const GPG_ERR_BROKEN_SECKEY: gpg_err_code_t = 196;
pub const GPG_ERR_MAC_ALGO: gpg_err_code_t = 197;
pub const GPG_ERR_FULLY_CANCELED: gpg_err_code_t = 198;
pub const GPG_ERR_UNFINISHED: gpg_err_code_t = 199;
pub const GPG_ERR_BUFFER_TOO_SHORT: gpg_err_code_t = 200;
pub const GPG_ERR_SEXP_INV_LEN_SPEC: gpg_err_code_t = 201;
pub const GPG_ERR_SEXP_STRING_TOO_LONG: gpg_err_code_t = 202;
pub const GPG_ERR_SEXP_UNMATCHED_PAREN: gpg_err_code_t = 203;
pub const GPG_ERR_SEXP_NOT_CANONICAL: gpg_err_code_t = 204;
pub const GPG_ERR_SEXP_BAD_CHARACTER: gpg_err_code_t = 205;
pub const GPG_ERR_SEXP_BAD_QUOTATION: gpg_err_code_t = 206;
pub const GPG_ERR_SEXP_ZERO_PREFIX: gpg_err_code_t = 207;
pub const GPG_ERR_SEXP_NESTED_DH: gpg_err_code_t = 208;
pub const GPG_ERR_SEXP_UNMATCHED_DH: gpg_err_code_t = 209;
pub const GPG_ERR_SEXP_UNEXPECTED_PUNC: gpg_err_code_t = 210;
pub const GPG_ERR_SEXP_BAD_HEX_CHAR: gpg_err_code_t = 211;
pub const GPG_ERR_SEXP_ODD_HEX_NUMBERS: gpg_err_code_t = 212;
pub const GPG_ERR_SEXP_BAD_OCT_CHAR: gpg_err_code_t = 213;
pub const GPG_ERR_SUBKEYS_EXP_OR_REV: gpg_err_code_t = 217;
pub const GPG_ERR_DB_CORRUPTED: gpg_err_code_t = 218;
pub const GPG_ERR_SERVER_FAILED: gpg_err_code_t = 219;
pub const GPG_ERR_NO_NAME: gpg_err_code_t = 220;
pub const GPG_ERR_NO_KEY: gpg_err_code_t = 221;
pub const GPG_ERR_LEGACY_KEY: gpg_err_code_t = 222;
pub const GPG_ERR_REQUEST_TOO_SHORT: gpg_err_code_t = 223;
pub const GPG_ERR_REQUEST_TOO_LONG: gpg_err_code_t = 224;
pub const GPG_ERR_OBJ_TERM_STATE: gpg_err_code_t = 225;
pub const GPG_ERR_NO_CERT_CHAIN: gpg_err_code_t = 226;
pub const GPG_ERR_CERT_TOO_LARGE: gpg_err_code_t = 227;
pub const GPG_ERR_INV_RECORD: gpg_err_code_t = 228;
pub const GPG_ERR_BAD_MAC: gpg_err_code_t = 229;
pub const GPG_ERR_UNEXPECTED_MSG: gpg_err_code_t = 230;
pub const GPG_ERR_COMPR_FAILED: gpg_err_code_t = 231;
pub const GPG_ERR_WOULD_WRAP: gpg_err_code_t = 232;
pub const GPG_ERR_FATAL_ALERT: gpg_err_code_t = 233;
pub const GPG_ERR_NO_CIPHER: gpg_err_code_t = 234;
pub const GPG_ERR_MISSING_CLIENT_CERT: gpg_err_code_t = 235;
pub const GPG_ERR_CLOSE_NOTIFY: gpg_err_code_t = 236;
pub const GPG_ERR_TICKET_EXPIRED: gpg_err_code_t = 237;
pub const GPG_ERR_BAD_TICKET: gpg_err_code_t = 238;
pub const GPG_ERR_UNKNOWN_IDENTITY: gpg_err_code_t = 239;
pub const GPG_ERR_BAD_HS_CERT: gpg_err_code_t = 240;
pub const GPG_ERR_BAD_HS_CERT_REQ: gpg_err_code_t = 241;
pub const GPG_ERR_BAD_HS_CERT_VER: gpg_err_code_t = 242;
pub const GPG_ERR_BAD_HS_CHANGE_CIPHER: gpg_err_code_t = 243;
pub const GPG_ERR_BAD_HS_CLIENT_HELLO: gpg_err_code_t = 244;
pub const GPG_ERR_BAD_HS_SERVER_HELLO: gpg_err_code_t = 245;
pub const GPG_ERR_BAD_HS_SERVER_HELLO_DONE: gpg_err_code_t = 246;
pub const GPG_ERR_BAD_HS_FINISHED: gpg_err_code_t = 247;
pub const GPG_ERR_BAD_HS_SERVER_KEX: gpg_err_code_t = 248;
pub const GPG_ERR_BAD_HS_CLIENT_KEX: gpg_err_code_t = 249;
pub const GPG_ERR_BOGUS_STRING: gpg_err_code_t = 250;
pub const GPG_ERR_FORBIDDEN: gpg_err_code_t = 251;
pub const GPG_ERR_KEY_DISABLED: gpg_err_code_t = 252;
pub const GPG_ERR_KEY_ON_CARD: gpg_err_code_t = 253;
pub const GPG_ERR_INV_LOCK_OBJ: gpg_err_code_t = 254;
pub const GPG_ERR_TRUE: gpg_err_code_t = 255;
pub const GPG_ERR_FALSE: gpg_err_code_t = 256;
pub const GPG_ERR_ASS_GENERAL: gpg_err_code_t = 257;
pub const GPG_ERR_ASS_ACCEPT_FAILED: gpg_err_code_t = 258;
pub const GPG_ERR_ASS_CONNECT_FAILED: gpg_err_code_t = 259;
pub const GPG_ERR_ASS_INV_RESPONSE: gpg_err_code_t = 260;
pub const GPG_ERR_ASS_INV_VALUE: gpg_err_code_t = 261;
pub const GPG_ERR_ASS_INCOMPLETE_LINE: gpg_err_code_t = 262;
pub const GPG_ERR_ASS_LINE_TOO_LONG: gpg_err_code_t = 263;
pub const GPG_ERR_ASS_NESTED_COMMANDS: gpg_err_code_t = 264;
pub const GPG_ERR_ASS_NO_DATA_CB: gpg_err_code_t = 265;
pub const GPG_ERR_ASS_NO_INQUIRE_CB: gpg_err_code_t = 266;
pub const GPG_ERR_ASS_NOT_A_SERVER: gpg_err_code_t = 267;
pub const GPG_ERR_ASS_NOT_A_CLIENT: gpg_err_code_t = 268;
pub const GPG_ERR_ASS_SERVER_START: gpg_err_code_t = 269;
pub const GPG_ERR_ASS_READ_ERROR: gpg_err_code_t = 270;
pub const GPG_ERR_ASS_WRITE_ERROR: gpg_err_code_t = 271;
pub const GPG_ERR_ASS_TOO_MUCH_DATA: gpg_err_code_t = 273;
pub const GPG_ERR_ASS_UNEXPECTED_CMD: gpg_err_code_t = 274;
pub const GPG_ERR_ASS_UNKNOWN_CMD: gpg_err_code_t = 275;
pub const GPG_ERR_ASS_SYNTAX: gpg_err_code_t = 276;
pub const GPG_ERR_ASS_CANCELED: gpg_err_code_t = 277;
pub const GPG_ERR_ASS_NO_INPUT: gpg_err_code_t = 278;
pub const GPG_ERR_ASS_NO_OUTPUT: gpg_err_code_t = 279;
pub const GPG_ERR_ASS_PARAMETER: gpg_err_code_t = 280;
pub const GPG_ERR_ASS_UNKNOWN_INQUIRE: gpg_err_code_t = 281;
pub const GPG_ERR_ENGINE_TOO_OLD: gpg_err_code_t = 300;
pub const GPG_ERR_WINDOW_TOO_SMALL: gpg_err_code_t = 301;
pub const GPG_ERR_WINDOW_TOO_LARGE: gpg_err_code_t = 302;
pub const GPG_ERR_MISSING_ENVVAR: gpg_err_code_t = 303;
pub const GPG_ERR_USER_ID_EXISTS: gpg_err_code_t = 304;
pub const GPG_ERR_NAME_EXISTS: gpg_err_code_t = 305;
pub const GPG_ERR_DUP_NAME: gpg_err_code_t = 306;
pub const GPG_ERR_TOO_YOUNG: gpg_err_code_t = 307;
pub const GPG_ERR_TOO_OLD: gpg_err_code_t = 308;
pub const GPG_ERR_UNKNOWN_FLAG: gpg_err_code_t = 309;
pub const GPG_ERR_INV_ORDER: gpg_err_code_t = 310;
pub const GPG_ERR_ALREADY_FETCHED: gpg_err_code_t = 311;
pub const GPG_ERR_TRY_LATER: gpg_err_code_t = 312;
pub const GPG_ERR_WRONG_NAME: gpg_err_code_t = 313;
pub const GPG_ERR_NO_AUTH: gpg_err_code_t = 314;
pub const GPG_ERR_BAD_AUTH: gpg_err_code_t = 315;
pub const GPG_ERR_NO_KEYBOXD: gpg_err_code_t = 316;
pub const GPG_ERR_KEYBOXD: gpg_err_code_t = 317;
pub const GPG_ERR_NO_SERVICE: gpg_err_code_t = 318;
pub const GPG_ERR_SERVICE: gpg_err_code_t = 319;
pub const GPG_ERR_SYSTEM_BUG: gpg_err_code_t = 666;
pub const GPG_ERR_DNS_UNKNOWN: gpg_err_code_t = 711;
pub const GPG_ERR_DNS_SECTION: gpg_err_code_t = 712;
pub const GPG_ERR_DNS_ADDRESS: gpg_err_code_t = 713;
pub const GPG_ERR_DNS_NO_QUERY: gpg_err_code_t = 714;
pub const GPG_ERR_DNS_NO_ANSWER: gpg_err_code_t = 715;
pub const GPG_ERR_DNS_CLOSED: gpg_err_code_t = 716;
pub const GPG_ERR_DNS_VERIFY: gpg_err_code_t = 717;
pub const GPG_ERR_DNS_TIMEOUT: gpg_err_code_t = 718;
pub const GPG_ERR_LDAP_GENERAL: gpg_err_code_t = 721;
pub const GPG_ERR_LDAP_ATTR_GENERAL: gpg_err_code_t = 722;
pub const GPG_ERR_LDAP_NAME_GENERAL: gpg_err_code_t = 723;
pub const GPG_ERR_LDAP_SECURITY_GENERAL: gpg_err_code_t = 724;
pub const GPG_ERR_LDAP_SERVICE_GENERAL: gpg_err_code_t = 725;
pub const GPG_ERR_LDAP_UPDATE_GENERAL: gpg_err_code_t = 726;
pub const GPG_ERR_LDAP_E_GENERAL: gpg_err_code_t = 727;
pub const GPG_ERR_LDAP_X_GENERAL: gpg_err_code_t = 728;
pub const GPG_ERR_LDAP_OTHER_GENERAL: gpg_err_code_t = 729;
pub const GPG_ERR_LDAP_X_CONNECTING: gpg_err_code_t = 750;
pub const GPG_ERR_LDAP_REFERRAL_LIMIT: gpg_err_code_t = 751;
pub const GPG_ERR_LDAP_CLIENT_LOOP: gpg_err_code_t = 752;
pub const GPG_ERR_LDAP_NO_RESULTS: gpg_err_code_t = 754;
pub const GPG_ERR_LDAP_CONTROL_NOT_FOUND: gpg_err_code_t = 755;
pub const GPG_ERR_LDAP_NOT_SUPPORTED: gpg_err_code_t = 756;
pub const GPG_ERR_LDAP_CONNECT: gpg_err_code_t = 757;
pub const GPG_ERR_LDAP_NO_MEMORY: gpg_err_code_t = 758;
pub const GPG_ERR_LDAP_PARAM: gpg_err_code_t = 759;
pub const GPG_ERR_LDAP_USER_CANCELLED: gpg_err_code_t = 760;
pub const GPG_ERR_LDAP_FILTER: gpg_err_code_t = 761;
pub const GPG_ERR_LDAP_AUTH_UNKNOWN: gpg_err_code_t = 762;
pub const GPG_ERR_LDAP_TIMEOUT: gpg_err_code_t = 763;
pub const GPG_ERR_LDAP_DECODING: gpg_err_code_t = 764;
pub const GPG_ERR_LDAP_ENCODING: gpg_err_code_t = 765;
pub const GPG_ERR_LDAP_LOCAL: gpg_err_code_t = 766;
pub const GPG_ERR_LDAP_SERVER_DOWN: gpg_err_code_t = 767;
pub const GPG_ERR_LDAP_SUCCESS: gpg_err_code_t = 768;
pub const GPG_ERR_LDAP_OPERATIONS: gpg_err_code_t = 769;
pub const GPG_ERR_LDAP_PROTOCOL: gpg_err_code_t = 770;
pub const GPG_ERR_LDAP_TIMELIMIT: gpg_err_code_t = 771;
pub const GPG_ERR_LDAP_SIZELIMIT: gpg_err_code_t = 772;
pub const GPG_ERR_LDAP_COMPARE_FALSE: gpg_err_code_t = 773;
pub const GPG_ERR_LDAP_COMPARE_TRUE: gpg_err_code_t = 774;
pub const GPG_ERR_LDAP_UNSUPPORTED_AUTH: gpg_err_code_t = 775;
pub const GPG_ERR_LDAP_STRONG_AUTH_RQRD: gpg_err_code_t = 776;
pub const GPG_ERR_LDAP_PARTIAL_RESULTS: gpg_err_code_t = 777;
pub const GPG_ERR_LDAP_REFERRAL: gpg_err_code_t = 778;
pub const GPG_ERR_LDAP_ADMINLIMIT: gpg_err_code_t = 779;
pub const GPG_ERR_LDAP_UNAVAIL_CRIT_EXTN: gpg_err_code_t = 780;
pub const GPG_ERR_LDAP_CONFIDENT_RQRD: gpg_err_code_t = 781;
pub const GPG_ERR_LDAP_SASL_BIND_INPROG: gpg_err_code_t = 782;
pub const GPG_ERR_LDAP_NO_SUCH_ATTRIBUTE: gpg_err_code_t = 784;
pub const GPG_ERR_LDAP_UNDEFINED_TYPE: gpg_err_code_t = 785;
pub const GPG_ERR_LDAP_BAD_MATCHING: gpg_err_code_t = 786;
pub const GPG_ERR_LDAP_CONST_VIOLATION: gpg_err_code_t = 787;
pub const GPG_ERR_LDAP_TYPE_VALUE_EXISTS: gpg_err_code_t = 788;
pub const GPG_ERR_LDAP_INV_SYNTAX: gpg_err_code_t = 789;
pub const GPG_ERR_LDAP_NO_SUCH_OBJ: gpg_err_code_t = 800;
pub const GPG_ERR_LDAP_ALIAS_PROBLEM: gpg_err_code_t = 801;
pub const GPG_ERR_LDAP_INV_DN_SYNTAX: gpg_err_code_t = 802;
pub const GPG_ERR_LDAP_IS_LEAF: gpg_err_code_t = 803;
pub const GPG_ERR_LDAP_ALIAS_DEREF: gpg_err_code_t = 804;
pub const GPG_ERR_LDAP_X_PROXY_AUTH_FAIL: gpg_err_code_t = 815;
pub const GPG_ERR_LDAP_BAD_AUTH: gpg_err_code_t = 816;
pub const GPG_ERR_LDAP_INV_CREDENTIALS: gpg_err_code_t = 817;
pub const GPG_ERR_LDAP_INSUFFICIENT_ACC: gpg_err_code_t = 818;
pub const GPG_ERR_LDAP_BUSY: gpg_err_code_t = 819;
pub const GPG_ERR_LDAP_UNAVAILABLE: gpg_err_code_t = 820;
pub const GPG_ERR_LDAP_UNWILL_TO_PERFORM: gpg_err_code_t = 821;
pub const GPG_ERR_LDAP_LOOP_DETECT: gpg_err_code_t = 822;
pub const GPG_ERR_LDAP_NAMING_VIOLATION: gpg_err_code_t = 832;
pub const GPG_ERR_LDAP_OBJ_CLS_VIOLATION: gpg_err_code_t = 833;
pub const GPG_ERR_LDAP_NOT_ALLOW_NONLEAF: gpg_err_code_t = 834;
pub const GPG_ERR_LDAP_NOT_ALLOW_ON_RDN: gpg_err_code_t = 835;
pub const GPG_ERR_LDAP_ALREADY_EXISTS: gpg_err_code_t = 836;
pub const GPG_ERR_LDAP_NO_OBJ_CLASS_MODS: gpg_err_code_t = 837;
pub const GPG_ERR_LDAP_RESULTS_TOO_LARGE: gpg_err_code_t = 838;
pub const GPG_ERR_LDAP_AFFECTS_MULT_DSAS: gpg_err_code_t = 839;
pub const GPG_ERR_LDAP_VLV: gpg_err_code_t = 844;
pub const GPG_ERR_LDAP_OTHER: gpg_err_code_t = 848;
pub const GPG_ERR_LDAP_CUP_RESOURCE_LIMIT: gpg_err_code_t = 881;
pub const GPG_ERR_LDAP_CUP_SEC_VIOLATION: gpg_err_code_t = 882;
pub const GPG_ERR_LDAP_CUP_INV_DATA: gpg_err_code_t = 883;
pub const GPG_ERR_LDAP_CUP_UNSUP_SCHEME: gpg_err_code_t = 884;
pub const GPG_ERR_LDAP_CUP_RELOAD: gpg_err_code_t = 885;
pub const GPG_ERR_LDAP_CANCELLED: gpg_err_code_t = 886;
pub const GPG_ERR_LDAP_NO_SUCH_OPERATION: gpg_err_code_t = 887;
pub const GPG_ERR_LDAP_TOO_LATE: gpg_err_code_t = 888;
pub const GPG_ERR_LDAP_CANNOT_CANCEL: gpg_err_code_t = 889;
pub const GPG_ERR_LDAP_ASSERTION_FAILED: gpg_err_code_t = 890;
pub const GPG_ERR_LDAP_PROX_AUTH_DENIED: gpg_err_code_t = 891;
pub const GPG_ERR_USER_1: gpg_err_code_t = 1024;
pub const GPG_ERR_USER_2: gpg_err_code_t = 1025;
pub const GPG_ERR_USER_3: gpg_err_code_t = 1026;
pub const GPG_ERR_USER_4: gpg_err_code_t = 1027;
pub const GPG_ERR_USER_5: gpg_err_code_t = 1028;
pub const GPG_ERR_USER_6: gpg_err_code_t = 1029;
pub const GPG_ERR_USER_7: gpg_err_code_t = 1030;
pub const GPG_ERR_USER_8: gpg_err_code_t = 1031;
pub const GPG_ERR_USER_9: gpg_err_code_t = 1032;
pub const GPG_ERR_USER_10: gpg_err_code_t = 1033;
pub const GPG_ERR_USER_11: gpg_err_code_t = 1034;
pub const GPG_ERR_USER_12: gpg_err_code_t = 1035;
pub const GPG_ERR_USER_13: gpg_err_code_t = 1036;
pub const GPG_ERR_USER_14: gpg_err_code_t = 1037;
pub const GPG_ERR_USER_15: gpg_err_code_t = 1038;
pub const GPG_ERR_USER_16: gpg_err_code_t = 1039;
pub const GPG_ERR_SQL_OK: gpg_err_code_t = 1500;
pub const GPG_ERR_SQL_ERROR: gpg_err_code_t = 1501;
pub const GPG_ERR_SQL_INTERNAL: gpg_err_code_t = 1502;
pub const GPG_ERR_SQL_PERM: gpg_err_code_t = 1503;
pub const GPG_ERR_SQL_ABORT: gpg_err_code_t = 1504;
pub const GPG_ERR_SQL_BUSY: gpg_err_code_t = 1505;
pub const GPG_ERR_SQL_LOCKED: gpg_err_code_t = 1506;
pub const GPG_ERR_SQL_NOMEM: gpg_err_code_t = 1507;
pub const GPG_ERR_SQL_READONLY: gpg_err_code_t = 1508;
pub const GPG_ERR_SQL_INTERRUPT: gpg_err_code_t = 1509;
pub const GPG_ERR_SQL_IOERR: gpg_err_code_t = 1510;
pub const GPG_ERR_SQL_CORRUPT: gpg_err_code_t = 1511;
pub const GPG_ERR_SQL_NOTFOUND: gpg_err_code_t = 1512;
pub const GPG_ERR_SQL_FULL: gpg_err_code_t = 1513;
pub const GPG_ERR_SQL_CANTOPEN: gpg_err_code_t = 1514;
pub const GPG_ERR_SQL_PROTOCOL: gpg_err_code_t = 1515;
pub const GPG_ERR_SQL_EMPTY: gpg_err_code_t = 1516;
pub const GPG_ERR_SQL_SCHEMA: gpg_err_code_t = 1517;
pub const GPG_ERR_SQL_TOOBIG: gpg_err_code_t = 1518;
pub const GPG_ERR_SQL_CONSTRAINT: gpg_err_code_t = 1519;
pub const GPG_ERR_SQL_MISMATCH: gpg_err_code_t = 1520;
pub const GPG_ERR_SQL_MISUSE: gpg_err_code_t = 1521;
pub const GPG_ERR_SQL_NOLFS: gpg_err_code_t = 1522;
pub const GPG_ERR_SQL_AUTH: gpg_err_code_t = 1523;
pub const GPG_ERR_SQL_FORMAT: gpg_err_code_t = 1524;
pub const GPG_ERR_SQL_RANGE: gpg_err_code_t = 1525;
pub const GPG_ERR_SQL_NOTADB: gpg_err_code_t = 1526;
pub const GPG_ERR_SQL_NOTICE: gpg_err_code_t = 1527;
pub const GPG_ERR_SQL_WARNING: gpg_err_code_t = 1528;
pub const GPG_ERR_SQL_ROW: gpg_err_code_t = 1600;
pub const GPG_ERR_SQL_DONE: gpg_err_code_t = 1601;
pub const GPG_ERR_MISSING_ERRNO: gpg_err_code_t = 16381;
pub const GPG_ERR_UNKNOWN_ERRNO: gpg_err_code_t = 16382;
pub const GPG_ERR_EOF: gpg_err_code_t = 16383;
pub const GPG_ERR_E2BIG: gpg_err_code_t = 32768;
pub const GPG_ERR_EACCES: gpg_err_code_t = 32769;
pub const GPG_ERR_EADDRINUSE: gpg_err_code_t = 32770;
pub const GPG_ERR_EADDRNOTAVAIL: gpg_err_code_t = 32771;
pub const GPG_ERR_EADV: gpg_err_code_t = 32772;
pub const GPG_ERR_EAFNOSUPPORT: gpg_err_code_t = 32773;
pub const GPG_ERR_EAGAIN: gpg_err_code_t = 32774;
pub const GPG_ERR_EALREADY: gpg_err_code_t = 32775;
pub const GPG_ERR_EAUTH: gpg_err_code_t = 32776;
pub const GPG_ERR_EBACKGROUND: gpg_err_code_t = 32777;
pub const GPG_ERR_EBADE: gpg_err_code_t = 32778;
pub const GPG_ERR_EBADF: gpg_err_code_t = 32779;
pub const GPG_ERR_EBADFD: gpg_err_code_t = 32780;
pub const GPG_ERR_EBADMSG: gpg_err_code_t = 32781;
pub const GPG_ERR_EBADR: gpg_err_code_t = 32782;
pub const GPG_ERR_EBADRPC: gpg_err_code_t = 32783;
pub const GPG_ERR_EBADRQC: gpg_err_code_t = 32784;
pub const GPG_ERR_EBADSLT: gpg_err_code_t = 32785;
pub const GPG_ERR_EBFONT: gpg_err_code_t = 32786;
pub const GPG_ERR_EBUSY: gpg_err_code_t = 32787;
pub const GPG_ERR_ECANCELED: gpg_err_code_t = 32788;
pub const GPG_ERR_ECHILD: gpg_err_code_t = 32789;
pub const GPG_ERR_ECHRNG: gpg_err_code_t = 32790;
pub const GPG_ERR_ECOMM: gpg_err_code_t = 32791;
pub const GPG_ERR_ECONNABORTED: gpg_err_code_t = 32792;
pub const GPG_ERR_ECONNREFUSED: gpg_err_code_t = 32793;
pub const GPG_ERR_ECONNRESET: gpg_err_code_t = 32794;
pub const GPG_ERR_ED: gpg_err_code_t = 32795;
pub const GPG_ERR_EDEADLK: gpg_err_code_t = 32796;
pub const GPG_ERR_EDEADLOCK: gpg_err_code_t = 32797;
pub const GPG_ERR_EDESTADDRREQ: gpg_err_code_t = 32798;
pub const GPG_ERR_EDIED: gpg_err_code_t = 32799;
pub const GPG_ERR_EDOM: gpg_err_code_t = 32800;
pub const GPG_ERR_EDOTDOT: gpg_err_code_t = 32801;
pub const GPG_ERR_EDQUOT: gpg_err_code_t = 32802;
pub const GPG_ERR_EEXIST: gpg_err_code_t = 32803;
pub const GPG_ERR_EFAULT: gpg_err_code_t = 32804;
pub const GPG_ERR_EFBIG: gpg_err_code_t = 32805;
pub const GPG_ERR_EFTYPE: gpg_err_code_t = 32806;
pub const GPG_ERR_EGRATUITOUS: gpg_err_code_t = 32807;
pub const GPG_ERR_EGREGIOUS: gpg_err_code_t = 32808;
pub const GPG_ERR_EHOSTDOWN: gpg_err_code_t = 32809;
pub const GPG_ERR_EHOSTUNREACH: gpg_err_code_t = 32810;
pub const GPG_ERR_EIDRM: gpg_err_code_t = 32811;
pub const GPG_ERR_EIEIO: gpg_err_code_t = 32812;
pub const GPG_ERR_EILSEQ: gpg_err_code_t = 32813;
pub const GPG_ERR_EINPROGRESS: gpg_err_code_t = 32814;
pub const GPG_ERR_EINTR: gpg_err_code_t = 32815;
pub const GPG_ERR_EINVAL: gpg_err_code_t = 32816;
pub const GPG_ERR_EIO: gpg_err_code_t = 32817;
pub const GPG_ERR_EISCONN: gpg_err_code_t = 32818;
pub const GPG_ERR_EISDIR: gpg_err_code_t = 32819;
pub const GPG_ERR_EISNAM: gpg_err_code_t = 32820;
pub const GPG_ERR_EL2HLT: gpg_err_code_t = 32821;
pub const GPG_ERR_EL2NSYNC: gpg_err_code_t = 32822;
pub const GPG_ERR_EL3HLT: gpg_err_code_t = 32823;
pub const GPG_ERR_EL3RST: gpg_err_code_t = 32824;
pub const GPG_ERR_ELIBACC: gpg_err_code_t = 32825;
pub const GPG_ERR_ELIBBAD: gpg_err_code_t = 32826;
pub const GPG_ERR_ELIBEXEC: gpg_err_code_t = 32827;
pub const GPG_ERR_ELIBMAX: gpg_err_code_t = 32828;
pub const GPG_ERR_ELIBSCN: gpg_err_code_t = 32829;
pub const GPG_ERR_ELNRNG: gpg_err_code_t = 32830;
pub const GPG_ERR_ELOOP: gpg_err_code_t = 32831;
pub const GPG_ERR_EMEDIUMTYPE: gpg_err_code_t = 32832;
pub const GPG_ERR_EMFILE: gpg_err_code_t = 32833;
pub const GPG_ERR_EMLINK: gpg_err_code_t = 32834;
pub const GPG_ERR_EMSGSIZE: gpg_err_code_t = 32835;
pub const GPG_ERR_EMULTIHOP: gpg_err_code_t = 32836;
pub const GPG_ERR_ENAMETOOLONG: gpg_err_code_t = 32837;
pub const GPG_ERR_ENAVAIL: gpg_err_code_t = 32838;
pub const GPG_ERR_ENEEDAUTH: gpg_err_code_t = 32839;
pub const GPG_ERR_ENETDOWN: gpg_err_code_t = 32840;
pub const GPG_ERR_ENETRESET: gpg_err_code_t = 32841;
pub const GPG_ERR_ENETUNREACH: gpg_err_code_t = 32842;
pub const GPG_ERR_ENFILE: gpg_err_code_t = 32843;
pub const GPG_ERR_ENOANO: gpg_err_code_t = 32844;
pub const GPG_ERR_ENOBUFS: gpg_err_code_t = 32845;
pub const GPG_ERR_ENOCSI: gpg_err_code_t = 32846;
pub const GPG_ERR_ENODATA: gpg_err_code_t = 32847;
pub const GPG_ERR_ENODEV: gpg_err_code_t = 32848;
pub const GPG_ERR_ENOENT: gpg_err_code_t = 32849;
pub const GPG_ERR_ENOEXEC: gpg_err_code_t = 32850;
pub const GPG_ERR_ENOLCK: gpg_err_code_t = 32851;
pub const GPG_ERR_ENOLINK: gpg_err_code_t = 32852;
pub const GPG_ERR_ENOMEDIUM: gpg_err_code_t = 32853;
pub const GPG_ERR_ENOMEM: gpg_err_code_t = 32854;
pub const GPG_ERR_ENOMSG: gpg_err_code_t = 32855;
pub const GPG_ERR_ENONET: gpg_err_code_t = 32856;
pub const GPG_ERR_ENOPKG: gpg_err_code_t = 32857;
pub const GPG_ERR_ENOPROTOOPT: gpg_err_code_t = 32858;
pub const GPG_ERR_ENOSPC: gpg_err_code_t = 32859;
pub const GPG_ERR_ENOSR: gpg_err_code_t = 32860;
pub const GPG_ERR_ENOSTR: gpg_err_code_t = 32861;
pub const GPG_ERR_ENOSYS: gpg_err_code_t = 32862;
pub const GPG_ERR_ENOTBLK: gpg_err_code_t = 32863;
pub const GPG_ERR_ENOTCONN: gpg_err_code_t = 32864;
pub const GPG_ERR_ENOTDIR: gpg_err_code_t = 32865;
pub const GPG_ERR_ENOTEMPTY: gpg_err_code_t = 32866;
pub const GPG_ERR_ENOTNAM: gpg_err_code_t = 32867;
pub const GPG_ERR_ENOTSOCK: gpg_err_code_t = 32868;
pub const GPG_ERR_ENOTSUP: gpg_err_code_t = 32869;
pub const GPG_ERR_ENOTTY: gpg_err_code_t = 32870;
pub const GPG_ERR_ENOTUNIQ: gpg_err_code_t = 32871;
pub const GPG_ERR_ENXIO: gpg_err_code_t = 32872;
pub const GPG_ERR_EOPNOTSUPP: gpg_err_code_t = 32873;
pub const GPG_ERR_EOVERFLOW: gpg_err_code_t = 32874;
pub const GPG_ERR_EPERM: gpg_err_code_t = 32875;
pub const GPG_ERR_EPFNOSUPPORT: gpg_err_code_t = 32876;
pub const GPG_ERR_EPIPE: gpg_err_code_t = 32877;
pub const GPG_ERR_EPROCLIM: gpg_err_code_t = 32878;
pub const GPG_ERR_EPROCUNAVAIL: gpg_err_code_t = 32879;
pub const GPG_ERR_EPROGMISMATCH: gpg_err_code_t = 32880;
pub const GPG_ERR_EPROGUNAVAIL: gpg_err_code_t = 32881;
pub const GPG_ERR_EPROTO: gpg_err_code_t = 32882;
pub const GPG_ERR_EPROTONOSUPPORT: gpg_err_code_t = 32883;
pub const GPG_ERR_EPROTOTYPE: gpg_err_code_t = 32884;
pub const GPG_ERR_ERANGE: gpg_err_code_t = 32885;
pub const GPG_ERR_EREMCHG: gpg_err_code_t = 32886;
pub const GPG_ERR_EREMOTE: gpg_err_code_t = 32887;
pub const GPG_ERR_EREMOTEIO: gpg_err_code_t = 32888;
pub const GPG_ERR_ERESTART: gpg_err_code_t = 32889;
pub const GPG_ERR_EROFS: gpg_err_code_t = 32890;
pub const GPG_ERR_ERPCMISMATCH: gpg_err_code_t = 32891;
pub const GPG_ERR_ESHUTDOWN: gpg_err_code_t = 32892;
pub const GPG_ERR_ESOCKTNOSUPPORT: gpg_err_code_t = 32893;
pub const GPG_ERR_ESPIPE: gpg_err_code_t = 32894;
pub const GPG_ERR_ESRCH: gpg_err_code_t = 32895;
pub const GPG_ERR_ESRMNT: gpg_err_code_t = 32896;
pub const GPG_ERR_ESTALE: gpg_err_code_t = 32897;
pub const GPG_ERR_ESTRPIPE: gpg_err_code_t = 32898;
pub const GPG_ERR_ETIME: gpg_err_code_t = 32899;
pub const GPG_ERR_ETIMEDOUT: gpg_err_code_t = 32900;
pub const GPG_ERR_ETOOMANYREFS: gpg_err_code_t = 32901;
pub const GPG_ERR_ETXTBSY: gpg_err_code_t = 32902;
pub const GPG_ERR_EUCLEAN: gpg_err_code_t = 32903;
pub const GPG_ERR_EUNATCH: gpg_err_code_t = 32904;
pub const GPG_ERR_EUSERS: gpg_err_code_t = 32905;
pub const GPG_ERR_EWOULDBLOCK: gpg_err_code_t = 32906;
pub const GPG_ERR_EXDEV: gpg_err_code_t = 32907;
pub const GPG_ERR_EXFULL: gpg_err_code_t = 32908;
pub const GPG_ERR_CODE_DIM: gpg_err_code_t = 65536;
pub type gpg_err_code_t = ::std::os::raw::c_uint;
pub type gpg_error_t = ::std::os::raw::c_uint;
extern "C" {
  pub fn gpg_err_init() -> gpg_error_t;
}
extern "C" {
  pub fn gpg_err_deinit(mode: ::std::os::raw::c_int);
}
extern "C" {
  pub fn gpgrt_set_syscall_clamp(
    pre: ::std::option::Option<unsafe extern "C" fn()>,
    post: ::std::option::Option<unsafe extern "C" fn()>,
  );
}
extern "C" {
  pub fn gpgrt_get_syscall_clamp(
    r_pre: *mut ::std::option::Option<unsafe extern "C" fn()>,
    r_post: *mut ::std::option::Option<unsafe extern "C" fn()>,
  );
}
extern "C" {
  pub fn gpgrt_set_alloc_func(
    f: ::std::option::Option<
      unsafe extern "C" fn(a: *mut ::std::os::raw::c_void, n: usize) -> *mut ::std::os::raw::c_void,
    >,
  );
}
extern "C" {
  pub fn gpgrt_add_emergency_cleanup(f: ::std::option::Option<unsafe extern "C" fn()>);
}
extern "C" {
  pub fn gpgrt_abort();
}
extern "C" {
  pub fn gpg_strerror(err: gpg_error_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpg_strerror_r(err: gpg_error_t, buf: *mut ::std::os::raw::c_char, buflen: usize) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpg_strsource(err: gpg_error_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpg_err_code_from_errno(err: ::std::os::raw::c_int) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpg_err_code_to_errno(code: gpg_err_code_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpg_err_code_from_syserror() -> gpg_err_code_t;
}
extern "C" {
  pub fn gpg_err_set_errno(err: ::std::os::raw::c_int);
}
extern "C" {
  pub fn gpgrt_check_version(req_version: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpg_error_check_version(req_version: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
pub type gpgrt_ssize_t = isize;
pub type gpgrt_off_t = ::std::os::raw::c_long;
extern "C" {
  pub fn gpgrt_realloc(a: *mut ::std::os::raw::c_void, n: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn gpgrt_reallocarray(
    a: *mut ::std::os::raw::c_void,
    oldnmemb: usize,
    nmemb: usize,
    size: usize,
  ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn gpgrt_malloc(n: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn gpgrt_calloc(n: usize, m: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn gpgrt_strdup(string: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_strconcat(s1: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_free(a: *mut ::std::os::raw::c_void);
}
extern "C" {
  pub fn gpgrt_getenv(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_setenv(
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
    overwrite: ::std::os::raw::c_int,
  ) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_mkdir(name: *const ::std::os::raw::c_char, modestr: *const ::std::os::raw::c_char) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_chdir(name: *const ::std::os::raw::c_char) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_getcwd() -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_access(fname: *const ::std::os::raw::c_char, mode: ::std::os::raw::c_int) -> gpg_err_code_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpgrt_lock_t
{
  pub _vers: ::std::os::raw::c_long,
  pub u: gpgrt_lock_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union gpgrt_lock_t__bindgen_ty_1
{
  pub _priv: [::std::os::raw::c_char; 40usize],
  pub _x_align: ::std::os::raw::c_long,
  pub _xp_align: *mut ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_gpgrt_lock_t__bindgen_ty_1()
{
  assert_eq!(
    ::std::mem::size_of::<gpgrt_lock_t__bindgen_ty_1>(),
    40usize,
    concat!("Size of: ", stringify!(gpgrt_lock_t__bindgen_ty_1))
  );
  assert_eq!(
    ::std::mem::align_of::<gpgrt_lock_t__bindgen_ty_1>(),
    8usize,
    concat!("Alignment of ", stringify!(gpgrt_lock_t__bindgen_ty_1))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_lock_t__bindgen_ty_1>()))._priv as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_lock_t__bindgen_ty_1),
      "::",
      stringify!(_priv)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_lock_t__bindgen_ty_1>()))._x_align as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_lock_t__bindgen_ty_1),
      "::",
      stringify!(_x_align)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_lock_t__bindgen_ty_1>()))._xp_align as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_lock_t__bindgen_ty_1),
      "::",
      stringify!(_xp_align)
    )
  );
}
#[test]
fn bindgen_test_layout_gpgrt_lock_t()
{
  assert_eq!(
    ::std::mem::size_of::<gpgrt_lock_t>(),
    48usize,
    concat!("Size of: ", stringify!(gpgrt_lock_t))
  );
  assert_eq!(
    ::std::mem::align_of::<gpgrt_lock_t>(),
    8usize,
    concat!("Alignment of ", stringify!(gpgrt_lock_t))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_lock_t>()))._vers as *const _ as usize },
    0usize,
    concat!("Offset of field: ", stringify!(gpgrt_lock_t), "::", stringify!(_vers))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_lock_t>())).u as *const _ as usize },
    8usize,
    concat!("Offset of field: ", stringify!(gpgrt_lock_t), "::", stringify!(u))
  );
}
extern "C" {
  pub fn gpgrt_lock_init(lockhd: *mut gpgrt_lock_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_lock_lock(lockhd: *mut gpgrt_lock_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_lock_trylock(lockhd: *mut gpgrt_lock_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_lock_unlock(lockhd: *mut gpgrt_lock_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_lock_destroy(lockhd: *mut gpgrt_lock_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_yield() -> gpg_err_code_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt_stream_internal
{
  _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt__stream
{
  pub flags: _gpgrt__stream__bindgen_ty_1,
  pub buffer: *mut ::std::os::raw::c_uchar,
  pub buffer_size: usize,
  pub data_len: usize,
  pub data_offset: usize,
  pub data_flushed: usize,
  pub unread_buffer: *mut ::std::os::raw::c_uchar,
  pub unread_buffer_size: usize,
  pub unread_data_len: usize,
  pub intern: *mut _gpgrt_stream_internal,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt__stream__bindgen_ty_1
{
  pub _bitfield_align_1: [u16; 0],
  pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout__gpgrt__stream__bindgen_ty_1()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt__stream__bindgen_ty_1>(),
    4usize,
    concat!("Size of: ", stringify!(_gpgrt__stream__bindgen_ty_1))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt__stream__bindgen_ty_1>(),
    4usize,
    concat!("Alignment of ", stringify!(_gpgrt__stream__bindgen_ty_1))
  );
}
impl _gpgrt__stream__bindgen_ty_1
{
  #[inline]
  pub fn magic(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
  }

  #[inline]
  pub fn set_magic(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(0usize, 16u8, val as u64)
    }
  }

  #[inline]
  pub fn writing(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_writing(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(16usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn reserved(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 15u8) as u32) }
  }

  #[inline]
  pub fn set_reserved(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(17usize, 15u8, val as u64)
    }
  }

  #[inline]
  pub fn new_bitfield_1(
    magic: ::std::os::raw::c_uint,
    writing: ::std::os::raw::c_uint,
    reserved: ::std::os::raw::c_uint,
  ) -> __BindgenBitfieldUnit<[u8; 4usize]>
  {
    let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
    __bindgen_bitfield_unit.set(0usize, 16u8, {
      let magic: u32 = unsafe { ::std::mem::transmute(magic) };
      magic as u64
    });
    __bindgen_bitfield_unit.set(16usize, 1u8, {
      let writing: u32 = unsafe { ::std::mem::transmute(writing) };
      writing as u64
    });
    __bindgen_bitfield_unit.set(17usize, 15u8, {
      let reserved: u32 = unsafe { ::std::mem::transmute(reserved) };
      reserved as u64
    });
    __bindgen_bitfield_unit
  }
}
#[test]
fn bindgen_test_layout__gpgrt__stream()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt__stream>(),
    80usize,
    concat!("Size of: ", stringify!(_gpgrt__stream))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt__stream>(),
    8usize,
    concat!("Alignment of ", stringify!(_gpgrt__stream))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).flags as *const _ as usize },
    0usize,
    concat!("Offset of field: ", stringify!(_gpgrt__stream), "::", stringify!(flags))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).buffer as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(buffer)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).buffer_size as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(buffer_size)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).data_len as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(data_len)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).data_offset as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(data_offset)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).data_flushed as *const _ as usize },
    40usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(data_flushed)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).unread_buffer as *const _ as usize },
    48usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(unread_buffer)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).unread_buffer_size as *const _ as usize },
    56usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(unread_buffer_size)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).unread_data_len as *const _ as usize },
    64usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(unread_data_len)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt__stream>())).intern as *const _ as usize },
    72usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt__stream),
      "::",
      stringify!(intern)
    )
  );
}
pub type gpgrt_stream_t = *mut _gpgrt__stream;
pub type gpgrt_cookie_read_function_t = ::std::option::Option<
  unsafe extern "C" fn(cookie: *mut ::std::os::raw::c_void, buffer: *mut ::std::os::raw::c_void, size: usize) -> isize,
>;
pub type gpgrt_cookie_write_function_t = ::std::option::Option<
  unsafe extern "C" fn(
    cookie: *mut ::std::os::raw::c_void,
    buffer: *const ::std::os::raw::c_void,
    size: usize,
  ) -> isize,
>;
pub type gpgrt_cookie_seek_function_t = ::std::option::Option<
  unsafe extern "C" fn(
    cookie: *mut ::std::os::raw::c_void,
    pos: *mut gpgrt_off_t,
    whence: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int,
>;
pub type gpgrt_cookie_close_function_t =
  ::std::option::Option<unsafe extern "C" fn(cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt_cookie_io_functions
{
  pub func_read: gpgrt_cookie_read_function_t,
  pub func_write: gpgrt_cookie_write_function_t,
  pub func_seek: gpgrt_cookie_seek_function_t,
  pub func_close: gpgrt_cookie_close_function_t,
}
#[test]
fn bindgen_test_layout__gpgrt_cookie_io_functions()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt_cookie_io_functions>(),
    32usize,
    concat!("Size of: ", stringify!(_gpgrt_cookie_io_functions))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt_cookie_io_functions>(),
    8usize,
    concat!("Alignment of ", stringify!(_gpgrt_cookie_io_functions))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_cookie_io_functions>())).func_read as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_cookie_io_functions),
      "::",
      stringify!(func_read)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_cookie_io_functions>())).func_write as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_cookie_io_functions),
      "::",
      stringify!(func_write)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_cookie_io_functions>())).func_seek as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_cookie_io_functions),
      "::",
      stringify!(func_seek)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_cookie_io_functions>())).func_close as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_cookie_io_functions),
      "::",
      stringify!(func_close)
    )
  );
}
pub type gpgrt_cookie_io_functions_t = _gpgrt_cookie_io_functions;
pub const GPGRT_SYSHD_NONE: gpgrt_syshd_types = 0;
pub const GPGRT_SYSHD_FD: gpgrt_syshd_types = 1;
pub const GPGRT_SYSHD_SOCK: gpgrt_syshd_types = 2;
pub const GPGRT_SYSHD_RVID: gpgrt_syshd_types = 3;
pub const GPGRT_SYSHD_HANDLE: gpgrt_syshd_types = 4;
pub type gpgrt_syshd_types = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _gpgrt_syshd
{
  pub type_: gpgrt_syshd_types,
  pub u: _gpgrt_syshd__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _gpgrt_syshd__bindgen_ty_1
{
  pub fd: ::std::os::raw::c_int,
  pub sock: ::std::os::raw::c_int,
  pub rvid: ::std::os::raw::c_int,
  pub handle: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__gpgrt_syshd__bindgen_ty_1()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt_syshd__bindgen_ty_1>(),
    8usize,
    concat!("Size of: ", stringify!(_gpgrt_syshd__bindgen_ty_1))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt_syshd__bindgen_ty_1>(),
    8usize,
    concat!("Alignment of ", stringify!(_gpgrt_syshd__bindgen_ty_1))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd__bindgen_ty_1>())).fd as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_syshd__bindgen_ty_1),
      "::",
      stringify!(fd)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd__bindgen_ty_1>())).sock as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_syshd__bindgen_ty_1),
      "::",
      stringify!(sock)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd__bindgen_ty_1>())).rvid as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_syshd__bindgen_ty_1),
      "::",
      stringify!(rvid)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd__bindgen_ty_1>())).handle as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_gpgrt_syshd__bindgen_ty_1),
      "::",
      stringify!(handle)
    )
  );
}
#[test]
fn bindgen_test_layout__gpgrt_syshd()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt_syshd>(),
    16usize,
    concat!("Size of: ", stringify!(_gpgrt_syshd))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt_syshd>(),
    8usize,
    concat!("Alignment of ", stringify!(_gpgrt_syshd))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd>())).type_ as *const _ as usize },
    0usize,
    concat!("Offset of field: ", stringify!(_gpgrt_syshd), "::", stringify!(type_))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_syshd>())).u as *const _ as usize },
    8usize,
    concat!("Offset of field: ", stringify!(_gpgrt_syshd), "::", stringify!(u))
  );
}
pub type gpgrt_syshd_t = _gpgrt_syshd;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt_poll_s
{
  pub stream: gpgrt_stream_t,
  pub _bitfield_align_1: [u8; 0],
  pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
  pub __bindgen_padding_0: u32,
}
#[test]
fn bindgen_test_layout__gpgrt_poll_s()
{
  assert_eq!(
    ::std::mem::size_of::<_gpgrt_poll_s>(),
    16usize,
    concat!("Size of: ", stringify!(_gpgrt_poll_s))
  );
  assert_eq!(
    ::std::mem::align_of::<_gpgrt_poll_s>(),
    8usize,
    concat!("Alignment of ", stringify!(_gpgrt_poll_s))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_gpgrt_poll_s>())).stream as *const _ as usize },
    0usize,
    concat!("Offset of field: ", stringify!(_gpgrt_poll_s), "::", stringify!(stream))
  );
}
impl _gpgrt_poll_s
{
  #[inline]
  pub fn want_read(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_want_read(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(0usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn want_write(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_want_write(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(1usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn want_oob(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_want_oob(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(2usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn want_rdhup(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_want_rdhup(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(3usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn _reserv1(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32) }
  }

  #[inline]
  pub fn set__reserv1(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(4usize, 4u8, val as u64)
    }
  }

  #[inline]
  pub fn got_read(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_read(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(8usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn got_write(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_write(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(9usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn got_oob(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_oob(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(10usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn got_rdhup(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_rdhup(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(11usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn _reserv2(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u32) }
  }

  #[inline]
  pub fn set__reserv2(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(12usize, 4u8, val as u64)
    }
  }

  #[inline]
  pub fn got_err(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_err(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(16usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn got_hup(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_hup(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(17usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn got_nval(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_got_nval(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(18usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn _reserv3(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 4u8) as u32) }
  }

  #[inline]
  pub fn set__reserv3(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(19usize, 4u8, val as u64)
    }
  }

  #[inline]
  pub fn ignore(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
  }

  #[inline]
  pub fn set_ignore(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(23usize, 1u8, val as u64)
    }
  }

  #[inline]
  pub fn user(&self) -> ::std::os::raw::c_uint
  {
    unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
  }

  #[inline]
  pub fn set_user(&mut self, val: ::std::os::raw::c_uint)
  {
    unsafe {
      let val: u32 = ::std::mem::transmute(val);
      self._bitfield_1.set(24usize, 8u8, val as u64)
    }
  }

  #[inline]
  pub fn new_bitfield_1(
    want_read: ::std::os::raw::c_uint,
    want_write: ::std::os::raw::c_uint,
    want_oob: ::std::os::raw::c_uint,
    want_rdhup: ::std::os::raw::c_uint,
    _reserv1: ::std::os::raw::c_uint,
    got_read: ::std::os::raw::c_uint,
    got_write: ::std::os::raw::c_uint,
    got_oob: ::std::os::raw::c_uint,
    got_rdhup: ::std::os::raw::c_uint,
    _reserv2: ::std::os::raw::c_uint,
    got_err: ::std::os::raw::c_uint,
    got_hup: ::std::os::raw::c_uint,
    got_nval: ::std::os::raw::c_uint,
    _reserv3: ::std::os::raw::c_uint,
    ignore: ::std::os::raw::c_uint,
    user: ::std::os::raw::c_uint,
  ) -> __BindgenBitfieldUnit<[u8; 4usize]>
  {
    let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
    __bindgen_bitfield_unit.set(0usize, 1u8, {
      let want_read: u32 = unsafe { ::std::mem::transmute(want_read) };
      want_read as u64
    });
    __bindgen_bitfield_unit.set(1usize, 1u8, {
      let want_write: u32 = unsafe { ::std::mem::transmute(want_write) };
      want_write as u64
    });
    __bindgen_bitfield_unit.set(2usize, 1u8, {
      let want_oob: u32 = unsafe { ::std::mem::transmute(want_oob) };
      want_oob as u64
    });
    __bindgen_bitfield_unit.set(3usize, 1u8, {
      let want_rdhup: u32 = unsafe { ::std::mem::transmute(want_rdhup) };
      want_rdhup as u64
    });
    __bindgen_bitfield_unit.set(4usize, 4u8, {
      let _reserv1: u32 = unsafe { ::std::mem::transmute(_reserv1) };
      _reserv1 as u64
    });
    __bindgen_bitfield_unit.set(8usize, 1u8, {
      let got_read: u32 = unsafe { ::std::mem::transmute(got_read) };
      got_read as u64
    });
    __bindgen_bitfield_unit.set(9usize, 1u8, {
      let got_write: u32 = unsafe { ::std::mem::transmute(got_write) };
      got_write as u64
    });
    __bindgen_bitfield_unit.set(10usize, 1u8, {
      let got_oob: u32 = unsafe { ::std::mem::transmute(got_oob) };
      got_oob as u64
    });
    __bindgen_bitfield_unit.set(11usize, 1u8, {
      let got_rdhup: u32 = unsafe { ::std::mem::transmute(got_rdhup) };
      got_rdhup as u64
    });
    __bindgen_bitfield_unit.set(12usize, 4u8, {
      let _reserv2: u32 = unsafe { ::std::mem::transmute(_reserv2) };
      _reserv2 as u64
    });
    __bindgen_bitfield_unit.set(16usize, 1u8, {
      let got_err: u32 = unsafe { ::std::mem::transmute(got_err) };
      got_err as u64
    });
    __bindgen_bitfield_unit.set(17usize, 1u8, {
      let got_hup: u32 = unsafe { ::std::mem::transmute(got_hup) };
      got_hup as u64
    });
    __bindgen_bitfield_unit.set(18usize, 1u8, {
      let got_nval: u32 = unsafe { ::std::mem::transmute(got_nval) };
      got_nval as u64
    });
    __bindgen_bitfield_unit.set(19usize, 4u8, {
      let _reserv3: u32 = unsafe { ::std::mem::transmute(_reserv3) };
      _reserv3 as u64
    });
    __bindgen_bitfield_unit.set(23usize, 1u8, {
      let ignore: u32 = unsafe { ::std::mem::transmute(ignore) };
      ignore as u64
    });
    __bindgen_bitfield_unit.set(24usize, 8u8, {
      let user: u32 = unsafe { ::std::mem::transmute(user) };
      user as u64
    });
    __bindgen_bitfield_unit
  }
}
pub type gpgrt_poll_t = _gpgrt_poll_s;
pub type gpgrt_string_filter_t = ::std::option::Option<
  unsafe extern "C" fn(
    s: *const ::std::os::raw::c_char,
    n: ::std::os::raw::c_int,
    opaque: *mut ::std::os::raw::c_void,
  ) -> *mut ::std::os::raw::c_char,
>;
extern "C" {
  pub fn gpgrt_fopen(path: *const ::std::os::raw::c_char, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_mopen(
    data: *mut ::std::os::raw::c_void,
    data_n: usize,
    data_len: usize,
    grow: ::std::os::raw::c_uint,
    func_realloc: ::std::option::Option<
      unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void, size: usize) -> *mut ::std::os::raw::c_void,
    >,
    func_free: ::std::option::Option<unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void)>,
    mode: *const ::std::os::raw::c_char,
  ) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fopenmem(memlimit: usize, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fopenmem_init(
    memlimit: usize,
    mode: *const ::std::os::raw::c_char,
    data: *const ::std::os::raw::c_void,
    datalen: usize,
  ) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fdopen(filedes: ::std::os::raw::c_int, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fdopen_nc(filedes: ::std::os::raw::c_int, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_sysopen(syshd: *mut gpgrt_syshd_t, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_sysopen_nc(syshd: *mut gpgrt_syshd_t, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fpopen(fp: *mut FILE, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fpopen_nc(fp: *mut FILE, mode: *const ::std::os::raw::c_char) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_freopen(
    path: *const ::std::os::raw::c_char,
    mode: *const ::std::os::raw::c_char,
    stream: gpgrt_stream_t,
  ) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fopencookie(
    cookie: *mut ::std::os::raw::c_void,
    mode: *const ::std::os::raw::c_char,
    functions: gpgrt_cookie_io_functions_t,
  ) -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_fclose(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fcancel(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fclose_snatch(
    stream: gpgrt_stream_t,
    r_buffer: *mut *mut ::std::os::raw::c_void,
    r_buflen: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_onclose(
    stream: gpgrt_stream_t,
    mode: ::std::os::raw::c_int,
    fnc: ::std::option::Option<unsafe extern "C" fn(arg1: gpgrt_stream_t, arg2: *mut ::std::os::raw::c_void)>,
    fnc_value: *mut ::std::os::raw::c_void,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fileno(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fileno_unlocked(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_syshd(stream: gpgrt_stream_t, syshd: *mut gpgrt_syshd_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_syshd_unlocked(stream: gpgrt_stream_t, syshd: *mut gpgrt_syshd_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_flockfile(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_ftrylockfile(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_funlockfile(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_feof(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_feof_unlocked(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_ferror(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_ferror_unlocked(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_clearerr(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_clearerr_unlocked(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_fflush(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fseek(
    stream: gpgrt_stream_t,
    offset: ::std::os::raw::c_long,
    whence: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fseeko(
    stream: gpgrt_stream_t,
    offset: gpgrt_off_t,
    whence: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_ftruncate(stream: gpgrt_stream_t, length: gpgrt_off_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_ftell(stream: gpgrt_stream_t) -> ::std::os::raw::c_long;
}
extern "C" {
  pub fn gpgrt_ftello(stream: gpgrt_stream_t) -> gpgrt_off_t;
}
extern "C" {
  pub fn gpgrt_rewind(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_fgetc(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fputc(c: ::std::os::raw::c_int, stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_ungetc(c: ::std::os::raw::c_int, stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_read(
    stream: gpgrt_stream_t,
    buffer: *mut ::std::os::raw::c_void,
    bytes_to_read: usize,
    bytes_read: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_write(
    stream: gpgrt_stream_t,
    buffer: *const ::std::os::raw::c_void,
    bytes_to_write: usize,
    bytes_written: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_write_sanitized(
    stream: gpgrt_stream_t,
    buffer: *const ::std::os::raw::c_void,
    length: usize,
    delimiters: *const ::std::os::raw::c_char,
    bytes_written: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_write_hexstring(
    stream: gpgrt_stream_t,
    buffer: *const ::std::os::raw::c_void,
    length: usize,
    reserved: ::std::os::raw::c_int,
    bytes_written: *mut usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fread(ptr: *mut ::std::os::raw::c_void, size: usize, nitems: usize, stream: gpgrt_stream_t) -> usize;
}
extern "C" {
  pub fn gpgrt_fwrite(ptr: *const ::std::os::raw::c_void, size: usize, nitems: usize, stream: gpgrt_stream_t) -> usize;
}
extern "C" {
  pub fn gpgrt_fgets(
    s: *mut ::std::os::raw::c_char,
    n: ::std::os::raw::c_int,
    stream: gpgrt_stream_t,
  ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_fputs(s: *const ::std::os::raw::c_char, stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fputs_unlocked(s: *const ::std::os::raw::c_char, stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_getline(lineptr: *mut *mut ::std::os::raw::c_char, n: *mut usize, stream: gpgrt_stream_t) -> isize;
}
extern "C" {
  pub fn gpgrt_read_line(
    stream: gpgrt_stream_t,
    addr_of_buffer: *mut *mut ::std::os::raw::c_char,
    length_of_buffer: *mut usize,
    max_length: *mut usize,
  ) -> isize;
}
extern "C" {
  pub fn gpgrt_fprintf(stream: gpgrt_stream_t, format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fprintf_unlocked(
    stream: gpgrt_stream_t,
    format: *const ::std::os::raw::c_char,
    ...
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fprintf_sf(
    stream: gpgrt_stream_t,
    sf: gpgrt_string_filter_t,
    sfvalue: *mut ::std::os::raw::c_void,
    format: *const ::std::os::raw::c_char,
    ...
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fprintf_sf_unlocked(
    stream: gpgrt_stream_t,
    sf: gpgrt_string_filter_t,
    sfvalue: *mut ::std::os::raw::c_void,
    format: *const ::std::os::raw::c_char,
    ...
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_printf(format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_printf_unlocked(format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_vfprintf(
    stream: gpgrt_stream_t,
    format: *const ::std::os::raw::c_char,
    ap: *mut __va_list_tag,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_vfprintf_unlocked(
    stream: gpgrt_stream_t,
    format: *const ::std::os::raw::c_char,
    ap: *mut __va_list_tag,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_setvbuf(
    stream: gpgrt_stream_t,
    buf: *mut ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    size: usize,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_setbuf(stream: gpgrt_stream_t, buf: *mut ::std::os::raw::c_char);
}
extern "C" {
  pub fn gpgrt_set_binary(stream: gpgrt_stream_t);
}
extern "C" {
  pub fn gpgrt_set_nonblock(stream: gpgrt_stream_t, onoff: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_get_nonblock(stream: gpgrt_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_poll(
    fdlist: *mut gpgrt_poll_t,
    nfds: ::std::os::raw::c_uint,
    timeout: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_tmpfile() -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_opaque_set(stream: gpgrt_stream_t, opaque: *mut ::std::os::raw::c_void);
}
extern "C" {
  pub fn gpgrt_opaque_get(stream: gpgrt_stream_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
  pub fn gpgrt_fname_set(stream: gpgrt_stream_t, fname: *const ::std::os::raw::c_char);
}
extern "C" {
  pub fn gpgrt_fname_get(stream: gpgrt_stream_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_asprintf(
    r_buf: *mut *mut ::std::os::raw::c_char,
    format: *const ::std::os::raw::c_char,
    ...
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_vasprintf(
    r_buf: *mut *mut ::std::os::raw::c_char,
    format: *const ::std::os::raw::c_char,
    ap: *mut __va_list_tag,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_bsprintf(format: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_vbsprintf(format: *const ::std::os::raw::c_char, ap: *mut __va_list_tag) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_snprintf(
    buf: *mut ::std::os::raw::c_char,
    bufsize: usize,
    format: *const ::std::os::raw::c_char,
    ...
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_vsnprintf(
    buf: *mut ::std::os::raw::c_char,
    bufsize: usize,
    format: *const ::std::os::raw::c_char,
    arg_ptr: *mut __va_list_tag,
  ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt_b64state
{
  _unused: [u8; 0],
}
pub type gpgrt_b64state_t = *mut _gpgrt_b64state;
extern "C" {
  pub fn gpgrt_b64enc_start(stream: gpgrt_stream_t, title: *const ::std::os::raw::c_char) -> gpgrt_b64state_t;
}
extern "C" {
  pub fn gpgrt_b64enc_write(
    state: gpgrt_b64state_t,
    buffer: *const ::std::os::raw::c_void,
    nbytes: usize,
  ) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_b64enc_finish(state: gpgrt_b64state_t) -> gpg_err_code_t;
}
extern "C" {
  pub fn gpgrt_b64dec_start(title: *const ::std::os::raw::c_char) -> gpgrt_b64state_t;
}
extern "C" {
  pub fn gpgrt_b64dec_proc(
    state: gpgrt_b64state_t,
    buffer: *mut ::std::os::raw::c_void,
    length: usize,
    r_nbytes: *mut usize,
  ) -> gpg_error_t;
}
extern "C" {
  pub fn gpgrt_b64dec_finish(state: gpgrt_b64state_t) -> gpg_error_t;
}
pub const GPGRT_LOGLVL_BEGIN: gpgrt_log_levels = 0;
pub const GPGRT_LOGLVL_CONT: gpgrt_log_levels = 1;
pub const GPGRT_LOGLVL_INFO: gpgrt_log_levels = 2;
pub const GPGRT_LOGLVL_WARN: gpgrt_log_levels = 3;
pub const GPGRT_LOGLVL_ERROR: gpgrt_log_levels = 4;
pub const GPGRT_LOGLVL_FATAL: gpgrt_log_levels = 5;
pub const GPGRT_LOGLVL_BUG: gpgrt_log_levels = 6;
pub const GPGRT_LOGLVL_DEBUG: gpgrt_log_levels = 7;
pub type gpgrt_log_levels = ::std::os::raw::c_uint;
extern "C" {
  pub fn gpgrt_log_set_sink(name: *const ::std::os::raw::c_char, stream: gpgrt_stream_t, fd: ::std::os::raw::c_int);
}
extern "C" {
  pub fn gpgrt_log_set_socket_dir_cb(
    fnc: ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
  );
}
extern "C" {
  pub fn gpgrt_log_set_pid_suffix_cb(
    cb: ::std::option::Option<unsafe extern "C" fn(r_value: *mut ::std::os::raw::c_ulong) -> ::std::os::raw::c_int>,
  );
}
extern "C" {
  pub fn gpgrt_log_set_prefix(text: *const ::std::os::raw::c_char, flags: ::std::os::raw::c_uint);
}
extern "C" {
  pub fn gpgrt_get_errorcount(clear: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_inc_errorcount();
}
extern "C" {
  pub fn gpgrt_log_get_prefix(flags: *mut ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_log_test_fd(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_log_get_fd() -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_log_get_stream() -> gpgrt_stream_t;
}
extern "C" {
  pub fn gpgrt_log(level: ::std::os::raw::c_int, fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_logv(level: ::std::os::raw::c_int, fmt: *const ::std::os::raw::c_char, arg_ptr: *mut __va_list_tag);
}
extern "C" {
  pub fn gpgrt_logv_prefix(
    level: ::std::os::raw::c_int,
    prefix: *const ::std::os::raw::c_char,
    fmt: *const ::std::os::raw::c_char,
    arg_ptr: *mut __va_list_tag,
  );
}
extern "C" {
  pub fn gpgrt_log_string(level: ::std::os::raw::c_int, string: *const ::std::os::raw::c_char);
}
extern "C" {
  pub fn gpgrt_log_bug(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_fatal(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_error(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_info(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_debug(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_debug_string(string: *const ::std::os::raw::c_char, fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_printf(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_printhex(
    buffer: *const ::std::os::raw::c_void,
    length: usize,
    fmt: *const ::std::os::raw::c_char,
    ...
  );
}
extern "C" {
  pub fn gpgrt_log_clock(fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
  pub fn gpgrt_log_flush();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _gpgrt_argparse_internal_s
{
  _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpgrt_argparse_t
{
  pub argc: *mut ::std::os::raw::c_int,
  pub argv: *mut *mut *mut ::std::os::raw::c_char,
  pub flags: ::std::os::raw::c_uint,
  pub err: ::std::os::raw::c_int,
  pub lineno: ::std::os::raw::c_uint,
  pub r_opt: ::std::os::raw::c_int,
  pub r_type: ::std::os::raw::c_int,
  pub r: gpgrt_argparse_t__bindgen_ty_1,
  pub internal: *mut _gpgrt_argparse_internal_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union gpgrt_argparse_t__bindgen_ty_1
{
  pub ret_int: ::std::os::raw::c_int,
  pub ret_long: ::std::os::raw::c_long,
  pub ret_ulong: ::std::os::raw::c_ulong,
  pub ret_str: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_gpgrt_argparse_t__bindgen_ty_1()
{
  assert_eq!(
    ::std::mem::size_of::<gpgrt_argparse_t__bindgen_ty_1>(),
    8usize,
    concat!("Size of: ", stringify!(gpgrt_argparse_t__bindgen_ty_1))
  );
  assert_eq!(
    ::std::mem::align_of::<gpgrt_argparse_t__bindgen_ty_1>(),
    8usize,
    concat!("Alignment of ", stringify!(gpgrt_argparse_t__bindgen_ty_1))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t__bindgen_ty_1>())).ret_int as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t__bindgen_ty_1),
      "::",
      stringify!(ret_int)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t__bindgen_ty_1>())).ret_long as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t__bindgen_ty_1),
      "::",
      stringify!(ret_long)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t__bindgen_ty_1>())).ret_ulong as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t__bindgen_ty_1),
      "::",
      stringify!(ret_ulong)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t__bindgen_ty_1>())).ret_str as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t__bindgen_ty_1),
      "::",
      stringify!(ret_str)
    )
  );
}
#[test]
fn bindgen_test_layout_gpgrt_argparse_t()
{
  assert_eq!(
    ::std::mem::size_of::<gpgrt_argparse_t>(),
    56usize,
    concat!("Size of: ", stringify!(gpgrt_argparse_t))
  );
  assert_eq!(
    ::std::mem::align_of::<gpgrt_argparse_t>(),
    8usize,
    concat!("Alignment of ", stringify!(gpgrt_argparse_t))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).argc as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(argc)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).argv as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(argv)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).flags as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(flags)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).err as *const _ as usize },
    20usize,
    concat!("Offset of field: ", stringify!(gpgrt_argparse_t), "::", stringify!(err))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).lineno as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(lineno)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).r_opt as *const _ as usize },
    28usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(r_opt)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).r_type as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(r_type)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).r as *const _ as usize },
    40usize,
    concat!("Offset of field: ", stringify!(gpgrt_argparse_t), "::", stringify!(r))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_argparse_t>())).internal as *const _ as usize },
    48usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_argparse_t),
      "::",
      stringify!(internal)
    )
  );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpgrt_opt_t
{
  pub short_opt: ::std::os::raw::c_int,
  pub long_opt: *const ::std::os::raw::c_char,
  pub flags: ::std::os::raw::c_uint,
  pub description: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_gpgrt_opt_t()
{
  assert_eq!(
    ::std::mem::size_of::<gpgrt_opt_t>(),
    32usize,
    concat!("Size of: ", stringify!(gpgrt_opt_t))
  );
  assert_eq!(
    ::std::mem::align_of::<gpgrt_opt_t>(),
    8usize,
    concat!("Alignment of ", stringify!(gpgrt_opt_t))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_opt_t>())).short_opt as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_opt_t),
      "::",
      stringify!(short_opt)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_opt_t>())).long_opt as *const _ as usize },
    8usize,
    concat!("Offset of field: ", stringify!(gpgrt_opt_t), "::", stringify!(long_opt))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_opt_t>())).flags as *const _ as usize },
    16usize,
    concat!("Offset of field: ", stringify!(gpgrt_opt_t), "::", stringify!(flags))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<gpgrt_opt_t>())).description as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(gpgrt_opt_t),
      "::",
      stringify!(description)
    )
  );
}
extern "C" {
  pub fn gpgrt_argparse(
    fp: gpgrt_stream_t,
    arg: *mut gpgrt_argparse_t,
    opts: *mut gpgrt_opt_t,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_argparser(
    arg: *mut gpgrt_argparse_t,
    opts: *mut gpgrt_opt_t,
    confname: *const ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_usage(level: ::std::os::raw::c_int);
}
extern "C" {
  pub fn gpgrt_strusage(level: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_set_strusage(
    f: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
  );
}
extern "C" {
  pub fn gpgrt_set_usage_outfnc(
    f: ::std::option::Option<
      unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
  );
}
extern "C" {
  pub fn gpgrt_set_fixed_string_mapper(
    f: ::std::option::Option<
      unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char,
    >,
  );
}
extern "C" {
  pub fn gpgrt_set_confdir(what: ::std::os::raw::c_int, name: *const ::std::os::raw::c_char);
}
extern "C" {
  pub fn gpgrt_cmp_version(
    a: *const ::std::os::raw::c_char,
    b: *const ::std::os::raw::c_char,
    level: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn gpgrt_fnameconcat(first: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char;
}
extern "C" {
  pub fn gpgrt_absfnameconcat(first: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag
{
  pub gp_offset: ::std::os::raw::c_uint,
  pub fp_offset: ::std::os::raw::c_uint,
  pub overflow_arg_area: *mut ::std::os::raw::c_void,
  pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag()
{
  assert_eq!(
    ::std::mem::size_of::<__va_list_tag>(),
    24usize,
    concat!("Size of: ", stringify!(__va_list_tag))
  );
  assert_eq!(
    ::std::mem::align_of::<__va_list_tag>(),
    8usize,
    concat!("Alignment of ", stringify!(__va_list_tag))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(__va_list_tag),
      "::",
      stringify!(gp_offset)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(__va_list_tag),
      "::",
      stringify!(fp_offset)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(__va_list_tag),
      "::",
      stringify!(overflow_arg_area)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(__va_list_tag),
      "::",
      stringify!(reg_save_area)
    )
  );
}

//! implement the operation type for multipart crypto API
use core::fmt;
use core::mem::MaybeUninit;


#[derive(Debug,Copy,Clone)]
/// The operation used in multi step crypto API
pub enum Operation {
    /// operation for multipart MAC
    MacOperation,
    /// Operation for multipart Aead
    AeadOperation,
    /// Operation for multipart hashing
    HashOperation
}


/// The wrapper of the C type for mac operation 
pub struct MacOperation(pub psa_crypto_sys::psa_mac_operation_t);
impl Default for MacOperation {
    fn default() -> Self {
        unsafe {
            MacOperation(MaybeUninit::zeroed().assume_init())
        }
    }
}


impl fmt::Debug for MacOperation {
    fn fmt(&self, f: &mut fmt:: Formatter<'_>) -> fmt::Result {
        write!(f, "MacOperation: (opaque C struct)")
    }
}

/// convert from rust type to C type
impl From<MacOperation> for psa_crypto_sys::psa_mac_operation_t {
    fn from(mac_oper: MacOperation) -> Self {
        mac_oper.0
    }
}

/// Function ta take the pointer of the inner type of MacOperation (pointer to psa_mac_operation_t)
impl MacOperation {
/// Function ta take the pointer of the inner type of MacOperation (pointer to psa_mac_operation_t)
    pub fn as_mut_ptr(&mut self) -> *mut psa_crypto_sys::psa_mac_operation_t {
        &mut self.0 as *mut _
    }
}
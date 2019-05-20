use core_foundation_sys::base::OSStatus;
use core_foundation_sys::string::CFStringRef;
use libc::c_void;
use MacTypes_sys::OSType;

pub enum OpaqueSecKeychainRef {}
pub type SecKeychainRef = *mut OpaqueSecKeychainRef;

pub enum OpaqueSecKeychainItemRef {}
pub type SecKeychainItemRef = *mut OpaqueSecKeychainItemRef;

pub type SecKeychainAttrType = OSType;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttribute {
    pub tag: SecKeychainAttrType,
    pub length: u32,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttributeList {
    pub count: u32,
    pub attr: *mut SecKeychainAttribute,
}

pub enum OpaqueSecCertificateRef {}
pub type SecCertificateRef = *mut OpaqueSecCertificateRef;

pub enum OpaqueSecAccessRef {}
pub type SecAccessRef = *mut OpaqueSecAccessRef;

pub enum OpaqueSecKeyRef {}
pub type SecKeyRef = *mut OpaqueSecKeyRef;

pub enum OpaqueSecIdentityRef {}
pub type SecIdentityRef = *mut OpaqueSecIdentityRef;

pub enum OpaqueSecPolicyRef {}
pub type SecPolicyRef = *mut OpaqueSecPolicyRef;

pub const errSecSuccess: OSStatus = 0;
pub const errSecUnimplemented: OSStatus = -4;
pub const errSecIO: OSStatus = -36;
pub const errSecParam: OSStatus = -50;
pub const errSecBadReq: OSStatus = -909;
pub const errSecAuthFailed: OSStatus = -25293;
pub const errSecTrustSettingDeny: OSStatus = -67654;
pub const errSecNotTrusted: OSStatus = -67843;

extern "C" {
    #[cfg(target_os = "macos")]
    pub fn SecCopyErrorMessageString(status: OSStatus, reserved: *mut c_void) -> CFStringRef;
}

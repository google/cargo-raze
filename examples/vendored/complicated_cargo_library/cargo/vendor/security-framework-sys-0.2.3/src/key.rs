use core_foundation_sys::base::CFTypeID;
#[cfg(target_os = "macos")]
use core_foundation_sys::data::CFDataRef;
#[cfg(target_os = "macos")]
use core_foundation_sys::dictionary::CFDictionaryRef;
#[cfg(target_os = "macos")]
use core_foundation_sys::error::CFErrorRef;

use base::SecKeyRef;

extern "C" {
    pub fn SecKeyGetTypeID() -> CFTypeID;

    #[cfg(target_os = "macos")]
    pub fn SecKeyCreateFromData(
        parameters: CFDictionaryRef,
        keyData: CFDataRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;

    #[cfg(feature = "OSX_10_12")]
    pub fn SecKeyCopyExternalRepresentation(key: SecKeyRef, error: *mut CFErrorRef) -> CFDataRef;
}

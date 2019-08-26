use libc::{c_int, c_void, size_t};

pub enum __SecRandom {}
pub type SecRandomRef = *const __SecRandom;

extern "C" {
    pub static kSecRandomDefault: SecRandomRef;

    pub fn SecRandomCopyBytes(rnd: SecRandomRef, count: size_t, bytes: *mut c_void) -> c_int;
}

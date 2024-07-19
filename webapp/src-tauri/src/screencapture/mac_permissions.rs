#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#[cfg(any(target_arch = "x86", target_arch = "arm", target_arch = "aarch64"))]
pub type boolean_t = libc::c_int;
#[cfg(target_arch = "x86_64")]
pub type boolean_t = libc::c_uint;

#[derive(Default)]
struct ScreenCaptureAccess;

impl ScreenCaptureAccess {
    #[inline]
    pub fn request_access(&self) -> bool {
        unsafe { CGRequestScreenCaptureAccess() == 1 }
    }

    #[inline]
    pub fn has_permission(&self) -> bool {
        unsafe { (CGPreflightScreenCaptureAccess() & 1) == 1 }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    // Screen Capture Access
    fn CGRequestScreenCaptureAccess() -> boolean_t;
    fn CGPreflightScreenCaptureAccess() -> boolean_t;
}


pub fn has_permission() -> bool {
  ScreenCaptureAccess::default().has_permission()
}

pub fn request_access() -> bool {
  ScreenCaptureAccess::default().request_access()
}
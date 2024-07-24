
pub fn request_access() -> bool {
  #[cfg(target_os = "macos")]
  return super::mac_permissions::request_access();

  // assume windows to be true
  #[cfg(target_os = "windows")]
  return true;

  // TODO: check if linux has permission system
  #[cfg(target_os = "linux")]
  return true;
}

pub fn has_permission() -> bool {
  #[cfg(target_os = "macos")]
  return super::mac_permissions::has_permission();

  #[cfg(target_os = "windows")]
  return true;

  #[cfg(target_os = "linux")]
  return true;
}
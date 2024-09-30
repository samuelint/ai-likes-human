
#[tauri::command]
pub fn assert_screen_capture_permissions() -> Result<bool, String> {
  if super::permissions::has_permission() {
    return Ok(true);
  }

  if super::permissions::request_access() {
    return Ok(true);
  }
  else {
    return Err("Screen capture permission denied".to_string());
  }
}

#[tauri::command]
pub async fn capture_screen(max_size: Option<i32>) -> Result<Vec<String>, String>  {
  let assert_permissions_result = assert_screen_capture_permissions();

  match assert_permissions_result {
    Ok(_) => {},
    Err(err) => return Err(err)
  }

  return super::screencapture::capture_screen(max_size).await;
}

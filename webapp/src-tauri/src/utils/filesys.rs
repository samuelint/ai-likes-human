use dirs;

pub fn local_data_dir_path() -> std::path::PathBuf {
    let home_dir = dirs::data_local_dir().unwrap();
    let base_path = home_dir.join("super_assistant");
    return base_path;
}
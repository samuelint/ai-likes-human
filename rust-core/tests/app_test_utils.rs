use app_core::{app_configuration::AppConfiguration, app_container::AppContainer};
use platform_dirs::AppDirs;
use std::fs::{self};

pub fn get_tests_directory() -> String {
    let app_dirs = AppDirs::new(Some("rust-integration-test-tmp"), false).unwrap();
    fs::create_dir_all(&app_dirs.cache_dir).unwrap();
    let directory_path = app_dirs.cache_dir.display();

    directory_path.to_string()
}

pub fn get_test_db_file_path() -> String {
    let directory_path = get_tests_directory();

    format!("{}{}", directory_path, "/test.sqlite.db")
}

pub fn get_test_db_url() -> String {
    let db_file_path = get_test_db_file_path();

    format!("sqlite://{}?mode=rwc", db_file_path)
}

fn delete_test_db() {
    let db_file_path = get_test_db_file_path();

    if fs::metadata(&db_file_path).is_ok() {
        fs::remove_file(&db_file_path).expect("Could not delete old test database");
    }
}

pub fn reset_test_environment() {
    delete_test_db();
}

pub async fn create_app_container() -> AppContainer {
    let database_url = get_test_db_url();

    AppContainer::create(AppConfiguration { database_url })
        .await
        .unwrap()
}

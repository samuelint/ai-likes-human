use app_core::{app_configuration::AppConfiguration, app_container::AppContainer};

pub fn get_test_db_url() -> String {
    format!("sqlite::memory:")
}

pub async fn create_app_container() -> AppContainer {
    let database_url = get_test_db_url();

    AppContainer::create(AppConfiguration { database_url })
        .await
        .unwrap()
}

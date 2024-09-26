use app_core::{AppContainer, CoreConfiguration};

pub fn get_test_db_url() -> String {
    format!("sqlite::memory:")
}

pub async fn create_app_container() -> AppContainer {
    let database_url = get_test_db_url();

    AppContainer::new(CoreConfiguration { database_url })
        .await
        .unwrap()
}

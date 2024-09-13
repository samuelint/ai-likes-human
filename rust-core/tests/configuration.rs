use app_core::configuration::domain::repository::{ConfigurationRepository, NewModel};
use app_core::HasProvider;
use app_test_utils::create_app_container;
use serial_test::serial;

mod app_test_utils;

#[tokio::test]
#[serial]
async fn test_configuration_create_read() {
    let app = create_app_container().await;
    let config_repo: Box<dyn ConfigurationRepository> = app.container.provide().unwrap();

    let find_result = config_repo.find_by_key("aa").await.unwrap();
    assert!(find_result.is_none());

    let upsert_result = config_repo
        .upsert_value_for_key(NewModel {
            key: "aa".to_string(),
            value: "AAA".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(upsert_result.value, "AAA");
    assert_eq!(upsert_result.key, "aa");

    let find_result = config_repo
        .find_by_key("aa")
        .await
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(
        find_result.value, "AAA",
        "Value does not exist after being find"
    );
}

#[tokio::test]
#[serial]
async fn test_configuration_upsert() {
    let app = create_app_container().await;
    let config_repo: Box<dyn ConfigurationRepository> = app.container.provide().unwrap();

    config_repo
        .upsert_value_for_key(NewModel {
            key: "bb".to_string(),
            value: "BB".to_string(),
        })
        .await
        .unwrap();

    let upsert_result = config_repo
        .upsert_value_for_key(NewModel {
            key: "bb".to_string(),
            value: "Hello".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(upsert_result.value, "Hello", "Value should be updated");

    let find_result = config_repo
        .find_by_key("bb")
        .await
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(find_result.value, "Hello", "Value should be updated");
}

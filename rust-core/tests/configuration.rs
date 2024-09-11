use app_core::configuration::domain::{
    dto::NewConfigurationItemDto, repository::ConfigurationRepository,
};
use app_core::HasProvider;
use app_test_utils::{create_app_container, reset_test_environment};
use serial_test::serial;

mod app_test_utils;

#[test]
#[serial]
fn test_configuration_create_read() {
    reset_test_environment();
    let app = create_app_container();
    let mut config_repo: Box<dyn ConfigurationRepository> = app.container.provide().unwrap();

    let find_result = config_repo.find_by_key("aa").unwrap();
    assert!(find_result.is_none());

    let upsert_result = config_repo
        .upsert_value_for_key(NewConfigurationItemDto {
            key: "aa",
            value: "AAA",
        })
        .unwrap();

    assert_eq!(upsert_result.value, "AAA");
    assert_eq!(upsert_result.key, "aa");

    let find_result = config_repo
        .find_by_key("aa")
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(
        find_result.value, "AAA",
        "Value does not exist after being find"
    );
}

#[test]
#[serial]
fn test_configuration_upsert() {
    reset_test_environment();
    let app = create_app_container();
    let mut config_repo: Box<dyn ConfigurationRepository> = app.container.provide().unwrap();

    config_repo
        .upsert_value_for_key(NewConfigurationItemDto {
            key: "bb",
            value: "BB",
        })
        .unwrap();

    let upsert_result = config_repo
        .upsert_value_for_key(NewConfigurationItemDto {
            key: "bb",
            value: "Hello",
        })
        .unwrap();
    assert_eq!(upsert_result.value, "Hello", "Value should be updated");

    let find_result = config_repo
        .find_by_key("bb")
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(find_result.value, "Hello", "Value should be updated");
}

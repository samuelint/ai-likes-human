use app_test_utils::create_app_container;

mod app_test_utils;

#[tokio::test]
async fn test_app_have_default_selected_llm_model() {
    let app = create_app_container().await;

    let service = app.configuration_module.get_configuration_service();

    let configuration = service.find("SELECTED_LLM_MODEL").await.unwrap();
    assert!(configuration.is_some());
    let configuration = configuration.unwrap();
    assert_eq!(configuration.key, "SELECTED_LLM_MODEL");
    assert!(configuration.value.len() > 0);
}

#[tokio::test]
async fn test_app_default_selected_llm_model_is_mutable() {
    let app = create_app_container().await;

    let service = app.configuration_module.get_configuration_service();

    let configuration = service
        .upsert("SELECTED_LLM_MODEL", "super-duper model")
        .await
        .unwrap();
    assert_eq!(configuration.value, "super-duper model");
    let configuration = service.find("SELECTED_LLM_MODEL").await.unwrap().unwrap();
    assert_eq!(configuration.value, "super-duper model");
}

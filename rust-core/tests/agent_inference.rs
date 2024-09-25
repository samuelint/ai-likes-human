use app_core::assistant::domain::agent_factory::CreateAgentParameters;
use app_test_utils::create_app_container;
use langchain_rust::prompt_args;
use serial_test::serial;

mod app_test_utils;

#[tokio::test]
#[serial]
async fn test_agent_creation_and_inference() {
    let app = create_app_container().await;
    let service = app.agent_module.get_agent_factory();

    let agent = service
        .create(CreateAgentParameters {
            model: "openai:gpt-4o-mini".to_string(),
        })
        .unwrap();

    let input_variables = prompt_args! {
        "input" => "Tell me a joke",
    };

    let response = agent.invoke(input_variables).await.unwrap();

    assert!(response.len() > 0);
}

use app_core::assistant::domain::dto::{
    ApiCreateRunDto, ApiCreateThreadAndRunDto, ApiCreateThreadDto, ApiCreateThreadMessageDto,
    MessageContent, RunDto, ThreadDto, ThreadEvent,
};
use futures::{Stream, StreamExt};
use hyper::StatusCode;

use super::router_client::RouterClient;

#[allow(dead_code)]
pub struct AssistantApiClient {
    client: RouterClient,
}

impl AssistantApiClient {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        let client = RouterClient::from_app("/openai/v1").await;

        Self { client }
    }
    #[allow(dead_code)]
    pub async fn create_thread(&self, request: &ApiCreateThreadDto) -> (ThreadDto, StatusCode) {
        let (dto, status) = self
            .client
            .post::<ApiCreateThreadDto, ThreadDto>("/threads", request)
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn create_thread_with_prompt(&self, prompt: &str) -> ThreadDto {
        let r = self
            .create_thread(&ApiCreateThreadDto {
                messages: vec![ApiCreateThreadMessageDto {
                    role: "user".to_string(),
                    content: vec![MessageContent::text(prompt)],
                    ..ApiCreateThreadMessageDto::default()
                }],
                ..ApiCreateThreadDto::default()
            })
            .await
            .0;

        r
    }

    #[allow(dead_code)]
    pub async fn stream_create_thread_and_run(
        &self,
        request: &ApiCreateThreadAndRunDto,
    ) -> impl Stream<Item = Result<ThreadEvent, axum::Error>> {
        let stream = self
            .client
            .post_json_stream::<ApiCreateThreadAndRunDto, ThreadEvent>("/threads/runs", request);

        stream
    }

    #[allow(dead_code)]
    pub async fn create_run(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> (RunDto, StatusCode) {
        let (dto, status) = self
            .client
            .post::<ApiCreateRunDto, RunDto>(
                format!("/threads/{}/runs", thread_id).as_str(),
                &request,
            )
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn stream_run(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> impl Stream<Item = Result<ThreadEvent, axum::Error>> {
        let stream = self
            .client
            .post_json_stream::<ApiCreateRunDto, ThreadEvent>(
                format!("/threads/{}/runs", thread_id).as_str(),
                &ApiCreateRunDto {
                    stream: Some(true),
                    ..request.clone()
                },
            );

        stream
    }

    #[allow(dead_code)]
    pub async fn stream_run_as_chunks_array(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> Vec<ThreadEvent> {
        let mut stream = self.stream_run(thread_id, request).await;

        let mut responses = Vec::new();
        while let Some(chunk) = stream.next().await {
            let response = chunk.unwrap();
            responses.push(response);
        }

        responses
    }

    #[allow(dead_code)]
    pub async fn get_run(&self, thread_id: &str, run_id: &str) -> (RunDto, StatusCode) {
        let (dto, status) = self
            .client
            .get::<RunDto>(&format!("/threads/{}/runs/{}", thread_id, run_id))
            .await
            .unwrap();

        (dto.unwrap(), status)
    }
}

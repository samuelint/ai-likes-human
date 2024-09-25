use futures::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc};

use super::{
    dto::{CreateRunDto, CreateThreadAndRunDto, ThreadEvent},
    run_factory::RunFactory,
    thread_chat_completions_inference::ThreadChatCompletionInference,
    thread_repository::ThreadRepository,
};
use crate::assistant::domain::dto::ThreadEventDto;
use futures::StreamExt;

pub struct StreamInferenceService {
    run_factory: Arc<RunFactory>,
    inference_service: Arc<ThreadChatCompletionInference>,
    thread_repository: Arc<dyn ThreadRepository>,
}

pub type AssistantStream = Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send>>;

impl StreamInferenceService {
    pub fn new(
        run_factory: Arc<RunFactory>,
        inference_service: Arc<ThreadChatCompletionInference>,
        thread_repository: Arc<dyn ThreadRepository>,
    ) -> Self {
        Self {
            run_factory,
            inference_service,
            thread_repository,
        }
    }

    pub fn stream_new_thread(&self, dto: &CreateThreadAndRunDto) -> AssistantStream {
        let dto = dto.clone();
        let thread_repository = self.thread_repository.clone();
        let self_clone = self.clone();

        let s = async_stream::try_stream! {
            let thread = match thread_repository.create((&dto).into()).await {
                Ok(thread) => {
                    yield ThreadEvent::ThreadCreated(ThreadEventDto::created_thread(&thread));
                    thread
                },
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            let new_run_dto: CreateRunDto = (&dto.clone()).into();
            let mut stream_run = self_clone.stream_new_run(&thread.id, &new_run_dto);

            while let Some(run_chunk) = stream_run.next().await {
                yield run_chunk.unwrap();
            }
        };

        Box::pin(s)
    }

    pub fn stream_new_run(&self, thread_id: &str, dto: &CreateRunDto) -> AssistantStream {
        let dto = dto.clone();
        let thread_id = thread_id.to_string();
        let run_factory = self.run_factory.clone();
        let thread_repository = self.thread_repository.clone();
        let inference_service = self.inference_service.clone();

        let s = async_stream::try_stream! {
            let run = match run_factory.create_run(&thread_id, &dto).await {
                Ok(res) => res,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(&run));

            let messages = match thread_repository.find_messages(&thread_id).await {
                Ok(messages) => messages,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            let mut stream = inference_service.stream(&run.model, &messages);

            while let Some(chunk) = stream.next().await {
                // yield run_chunk.unwrap();
            }

            yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::completed(&run));
        };

        Box::pin(s)
    }
}

impl Clone for StreamInferenceService {
    fn clone(&self) -> Self {
        StreamInferenceService {
            run_factory: self.run_factory.clone(),
            inference_service: self.inference_service.clone(),
            thread_repository: self.thread_repository.clone(),
        }
    }
}

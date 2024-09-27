#[cfg(test)]
#[path = "./stream_thread_run_service_test.rs"]
mod stream_thread_run_service_test;

use std::sync::Arc;

use super::{
    dto::{ApiCreateRunDto, ApiCreateThreadAndRunDto, RunStepDto, ThreadEvent},
    message::{message_status_mutator::MessageStatusMutator, MessageDeltaUpdateService},
    run::{run_status_mutator::RunStatusMutator, RunFactory},
    stream_types::AssistantStream,
    thread::{ThreadMessageFactory, ThreadRepository},
    thread_chat_completions_inference::ThreadChatCompletionInference,
};
use crate::assistant::domain::dto::ThreadEventDto;
use futures::StreamExt;

pub struct StreamThreadRunService {
    run_factory: Arc<RunFactory>,
    inference_service: Arc<ThreadChatCompletionInference>,
    thread_repository: Arc<dyn ThreadRepository>,
    thread_message_factory: Arc<ThreadMessageFactory>,
    message_delta_update_service: Arc<MessageDeltaUpdateService>,
    run_status_mutator: Arc<RunStatusMutator>,
    message_status_mutator: Arc<MessageStatusMutator>,
}

impl StreamThreadRunService {
    pub fn new(
        run_factory: Arc<RunFactory>,
        inference_service: Arc<ThreadChatCompletionInference>,
        thread_repository: Arc<dyn ThreadRepository>,
        thread_message_factory: Arc<ThreadMessageFactory>,
        message_delta_update_service: Arc<MessageDeltaUpdateService>,
        run_status_mutator: Arc<RunStatusMutator>,
        message_status_mutator: Arc<MessageStatusMutator>,
    ) -> Self {
        Self {
            run_factory,
            inference_service,
            thread_repository,
            thread_message_factory,
            message_delta_update_service,
            run_status_mutator,
            message_status_mutator,
        }
    }

    pub fn stream_new_thread(&self, dto: &ApiCreateThreadAndRunDto) -> AssistantStream {
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

            let new_run_dto: ApiCreateRunDto = (&dto.clone()).into();
            let mut stream_run = self_clone.stream_new_run(&thread.id, &new_run_dto);

            while let Some(run_chunk) = stream_run.next().await {
                yield run_chunk.unwrap();
            }
        };

        Box::pin(s)
    }

    // Follow openai Assistant API streaming
    // https://platform.openai.com/docs/api-reference/runs/createRun
    pub fn stream_new_run(&self, thread_id: &str, dto: &ApiCreateRunDto) -> AssistantStream {
        let dto = dto.clone();
        let thread_id = thread_id.to_string();
        let run_factory = self.run_factory.clone();
        let thread_repository = self.thread_repository.clone();
        let inference_service = self.inference_service.clone();
        let thread_message_factory = self.thread_message_factory.clone();
        let message_delta_update_service = self.message_delta_update_service.clone();
        let run_status_mutator = self.run_status_mutator.clone();
        let message_status_mutator = self.message_status_mutator.clone();

        let s = async_stream::try_stream! {

            // Create
            let run = match run_factory.create_run(&thread_id, &dto).await {
                Ok(res) => res,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };
            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(&run));
            yield ThreadEvent::ThreadRunQueued(ThreadEventDto::run_queued(&run));

            let run = match run_status_mutator.mutate_to_in_progress(&run).await {
                Ok(run) => run,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };
            yield ThreadEvent::ThreadRunInProgress(ThreadEventDto::run_in_progress(&run));


            let messages = match thread_repository.find_messages(&thread_id).await {
                Ok(messages) => messages,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            let last_message = match messages.last() {
                Some(message) => message,
                None => {
                    let run = match run_status_mutator.mutate_to_completed(&run).await {
                        Ok(run) => run,
                        Err(e) => {
                            yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                            return;
                        }
                    };
                    yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::run_completed(&run));
                    yield ThreadEvent::Done(ThreadEventDto::done());
                    return;
                }
            };

            // Step
            let mut run_step = RunStepDto::message_creation_from_run("step-1", &last_message.id, "in_progress", &run);
            yield ThreadEvent::ThreadRunStepCreated(ThreadEventDto::run_step_created(&run_step));
            run_step.status = "in_progress".to_string();
            yield ThreadEvent::ThreadRunStepInProgress(ThreadEventDto::run_step_in_progress(&run_step));


            let mut stream = inference_service.stream(&run.model, &messages);

            let mut message = match thread_message_factory.create_assistant(&thread_id, &run.id).await {
                Ok(message) => message,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            yield ThreadEvent::ThreadMessageCreated(ThreadEventDto::thread_message_created(&message));
            yield ThreadEvent::ThreadMessageInProgress(ThreadEventDto::thread_message_in_progress(&message));

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                        return;
                    }
                };

                let (message_delta, updated_message) = match message_delta_update_service.from_chunk(&chunk, &message).await {
                    Ok(res) => res,
                    Err(e) => {
                        yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                        return;
                    }
                };
                message = updated_message;

                yield ThreadEvent::ThreadMessageDelta(ThreadEventDto::thread_message_delta(&message_delta));
            }


            let message = match message_status_mutator.mutate_to_completed(&message).await {
                Ok(message) => message,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };
            yield ThreadEvent::ThreadMessageCompleted(ThreadEventDto::thread_message_completed(&message));

            run_step.status = "completed".to_string();
            yield ThreadEvent::ThreadRunStepCompleted(ThreadEventDto::run_step_completed(&run_step));
            // Step - End


            let run = match run_status_mutator.mutate_to_completed(&run).await {
                Ok(run) => run,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };
            yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::run_completed(&run));
            yield ThreadEvent::Done(ThreadEventDto::done());
        };

        Box::pin(s)
    }
}

impl Clone for StreamThreadRunService {
    fn clone(&self) -> Self {
        StreamThreadRunService {
            run_factory: self.run_factory.clone(),
            inference_service: self.inference_service.clone(),
            thread_repository: self.thread_repository.clone(),
            thread_message_factory: self.thread_message_factory.clone(),
            message_delta_update_service: self.message_delta_update_service.clone(),
            run_status_mutator: self.run_status_mutator.clone(),
            message_status_mutator: self.message_status_mutator.clone(),
        }
    }
}

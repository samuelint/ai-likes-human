use futures::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc};

use super::{
    dto::{CreateRunDto, CreateThreadAndRunDto, ThreadEvent},
    run_factory::RunFactory,
    thread_repository::ThreadRepository,
};
use crate::{
    assistant::domain::dto::ThreadEventDto,
    llm::domain::llm_factory::{CreateLLMParameters, LLMFactory},
};
use futures::StreamExt;

pub struct StreamRunService {
    run_factory: Arc<RunFactory>,
    llm_factory: Arc<dyn LLMFactory>,
    thread_repository: Arc<dyn ThreadRepository>,
}

impl StreamRunService {
    pub fn new(
        run_factory: Arc<RunFactory>,
        llm_factory: Arc<dyn LLMFactory>,
        thread_repository: Arc<dyn ThreadRepository>,
    ) -> Self {
        Self {
            run_factory,
            llm_factory,
            thread_repository,
        }
    }

    pub fn stream_new_thread(
        &self,
        dto: &CreateThreadAndRunDto,
    ) -> Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>> {
        let dto = dto.clone();
        let s = async_stream::try_stream! {
            let thread = self.thread_repository.create((&dto).into()).await.unwrap();

            yield ThreadEvent::ThreadCreated(ThreadEventDto::created_thread(&thread));

            let new_run_dto: CreateRunDto = (&dto).into();
            let mut stream_run = self.stream_new_run(&thread.id, &new_run_dto);

            while let Some(run_chunk) = stream_run.next().await {
                yield run_chunk.unwrap();
            }
        };

        Box::pin(s)
    }

    pub fn stream_new_run<'a>(
        &'a self,
        thread_id: &'a str,
        dto: &'a CreateRunDto,
    ) -> Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>> {
        let dto = dto.clone();
        let s = async_stream::try_stream! {
            let run = match self.run_factory.create_run(&thread_id, &dto).await {
                Ok(res) => res,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(&run));

            let llm = match self.llm_factory.create(CreateLLMParameters {
                model: dto.model,
            }) {
                Ok(llm) => llm,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                },
            };

            // let stream = llm.stream(vec![Message {}]).await;


            yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::completed(&run));
        };

        Box::pin(s)
    }
}

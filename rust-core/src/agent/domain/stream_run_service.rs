use futures::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc};

use super::{
    dto::{CreateRunDto, CreateThreadAndRunDto, RunDto, ThreadEvent},
    run_factory::RunFactory,
};
use crate::{
    agent::domain::dto::{ThreadDto, ThreadEventDto},
    llm::domain::llm_factory::{CreateLLMParameters, LLMFactory},
};

pub struct StreamRunService {
    run_factory: Arc<RunFactory>,
    llm_factory: Arc<dyn LLMFactory>,
}

impl StreamRunService {
    pub fn new(run_factory: Arc<RunFactory>, llm_factory: Arc<dyn LLMFactory>) -> Self {
        Self {
            run_factory,
            llm_factory,
        }
    }

    pub fn stream_new_thread(
        &self,
        dto: CreateThreadAndRunDto,
    ) -> Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>> {
        let s = async_stream::try_stream! {
            let (thread, run) = match self.run_factory.create_thread_and_run(&dto).await {
                Ok(res) => res,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };

            let thread_dto: ThreadDto = thread.into();
            let run_dto: RunDto = run.into();

            yield ThreadEvent::ThreadCreated(ThreadEventDto::created_thread(thread_dto));
            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(run_dto.clone()));

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


            yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::completed(run_dto));
        };

        Box::pin(s)
    }

    pub fn stream_new_run(
        &self,
        thread_id: &String,
        dto: &CreateRunDto,
    ) -> Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>> {
        let dto = dto.clone();
        let thread_id = thread_id.clone();
        let s = async_stream::try_stream! {
            let dto = dto.clone();
            let run = match self.run_factory.create_run(&thread_id, &dto).await {
                Ok(res) => res,
                Err(e) => {
                    yield ThreadEvent::Error(ThreadEventDto::std_error(e));
                    return;
                }
            };


            let run_dto: RunDto = run.into();

            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(run_dto.clone()));

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


            yield ThreadEvent::ThreadRunCompleted(ThreadEventDto::completed(run_dto));
        };

        Box::pin(s)
    }
}

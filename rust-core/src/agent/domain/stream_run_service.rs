use futures::Stream;
use std::{convert::Infallible, pin::Pin, sync::Arc};

use super::{
    dto::{stream_error::StreamErrorDto, CreateThreadAndRunDto, RunDto, ThreadEvent},
    run_factory::RunFactory,
};
use crate::agent::domain::dto::{ThreadDto, ThreadEventDto};

pub struct StreamRunService {
    run_factory: Arc<RunFactory>,
}

impl StreamRunService {
    pub fn new(run_factory: Arc<RunFactory>) -> Self {
        Self { run_factory }
    }

    pub fn create_thread_run_and_stream(
        &self,
        dto: CreateThreadAndRunDto,
    ) -> Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>> {
        let s = async_stream::try_stream! {
            let (thread, run) = match self.run_factory.create_thread_and_run(&dto).await {
                Ok(res) => res,
                Err(e) => {
                    let error = StreamErrorDto {
                        message: e.to_string(),
                        r#type: "error".to_string(),
                        ..StreamErrorDto::default()
                    };

                    yield ThreadEvent::Error(ThreadEventDto::error(error));
                    return;
                }
            };

            let thread_dto: ThreadDto = thread.into();
            let run_dto: RunDto = run.into();

            yield ThreadEvent::ThreadCreated(ThreadEventDto::created_thread(thread_dto));
            yield ThreadEvent::ThreadRunCreated(ThreadEventDto::created_run(run_dto));
        };

        Box::pin(s)
    }
}

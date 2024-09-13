use std::error::Error;

use super::dto::{MessageDto, NewMessageDto};

pub trait MessageRepository {
    fn find(&mut self, id: &i32) -> Result<Option<MessageDto>, Box<dyn Error>>;

    fn upsert(&mut self, message: NewMessageDto) -> Result<MessageDto, Box<dyn Error>>;

    fn delete(&mut self, id: &i32) -> Result<MessageDto, Box<dyn Error>>;
}

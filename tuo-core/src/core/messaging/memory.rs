use async_trait::async_trait;
use crate::core::messaging::message::Message;
use crate::error::TuoError;

#[derive(Default)]
pub struct Memory {
    pub messages: Vec<Message>,
}

#[async_trait]
pub trait MemoryTrait {
    fn memorize(&mut self, message: Message) -> Result<(), TuoError>;
}

#[async_trait]
impl MemoryTrait for Memory {
    fn memorize(&mut self, message: Message) -> Result<(), TuoError> {
        todo!()
    }
}
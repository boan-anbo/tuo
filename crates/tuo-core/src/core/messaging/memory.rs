use async_trait::async_trait;

use tuo_shared::types::return_type::TuoResult;

use crate::core::messaging::message::Message;

#[derive(Default)]
pub struct Memory {
    pub messages: Vec<Message>,
}

#[async_trait]
pub trait MemoryTrait {
    fn memorize(&mut self, message: Message) -> TuoResult<()>;
}

#[async_trait]
impl MemoryTrait for Memory {
    fn memorize(&mut self, message: Message) -> TuoResult<()> {
        todo!()
    }
}
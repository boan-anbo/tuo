use async_trait::async_trait;
use crate::error::TuoError;

pub struct Agenda {}


pub struct Accomplishment {}

#[async_trait]
pub trait AgentTrait {
    async fn act(&self, agenda: Agenda) -> Result<Accomplishment, TuoError>;
}

use crate::error::TuoError;
use crate::retrieval::retriever::RetrievedResult;

pub trait RetrievalPostProcessorTrait {
    fn process(&self, retrieved_result: RetrievedResult) -> Result<RetrievedResult, TuoError>;
}
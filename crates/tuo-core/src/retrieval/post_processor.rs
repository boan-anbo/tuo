use tuo_shared::types::return_type::TuoResult;

use crate::retrieval::retriever::RetrievedResult;

pub trait RetrievalPostProcessorTrait {
    fn process(&self, retrieved_result: RetrievedResult) -> TuoResult<RetrievedResult>;
}
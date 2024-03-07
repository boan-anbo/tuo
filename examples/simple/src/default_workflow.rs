use tuo::tuo_core::extraction::reader::UniFolderReaderTrait;
use tuo::tuo_core::query::engine::{QueryEngineTrait, QueryResult};
use tuo::tuo_parts::readers::uni_folder_reader::UniFolderReader;
use tuo::tuo_shared::errors::tuo::TuoError;

pub async fn default_workflow_complete() -> Result<QueryResult, TuoError> {
    // Extraction
    let source_folder_name = "source_folder_name";
    let folder_reader = UniFolderReader::default();
    let read_result = folder_reader.read_folder(source_folder_name).await?;

    todo!();
    // Index
    // let mut loaded_index = LanceDb.load(source_folder_name, read_result).await?;
    // loaded_index.embed().await?;
    //
    // // Storage
    // // skippable because query engine can be used directly with indexing in memory
    // let store_path = "store_path";
    // let store = LanceDb::builder().store_uri(store_path.to_string()).build();
    // store.create_index(loaded_index).await?;
    //
    // // Target indexing
    // let index = store.open_index_by_name(source_folder_name).await?.expect("Index not found");
    //
    // // Query
    // let query_result = QueryEngine::default().load_index(index).await?;
    //
    // let result = query_result.query(QueryRequest::default()).await?;
    //
    // Ok(result)
}
//
// pub async fn default_workflow_two_steps() -> Result<QueryResult, TuoError> {
//     let source_folder_name = "source_folder_name";
//
//     // Step 1: Engine-building
//     let engine = QueryEngine::from_folder(source_folder_name).await?;
//
//     // Step 2: Query
//     let result = engine.query(QueryRequest::default()).await?;
//
//     Ok(result)
// }
//
// pub async fn default_workflow_three_steps() -> Result<QueryResult, TuoError> {
//     let source_folder_name = "source_folder_name";
//
//     // Step 1: Indexing
//     let index = Index::from_folder(source_folder_name).await?;
//
//     // Step 2: Engine-building
//     let engine = QueryEngine::default().load_index(index).await?;
//
//     // Step 3: Query
//     let result = engine.query(QueryRequest::default()).await?;
//
//     Ok(result)
// }
//
// pub async fn default_workflow_four_steps() -> Result<QueryResult, TuoError> {
//     let source_folder_name = "source_folder_name";
//
//     // Step 1: Extraction
//     let documents = UniFolderReader::default().read_folder(source_folder_name).await?;
//
//     // Step 2: Index
//     let index = Index::default().load(source_folder_name, documents).await?;
//
//     // Step 3: Engine-building
//     let query_result = QueryEngine::default().load_index(index).await?;
//
//     // Step 4: Query
//     let result = query_result.query(QueryRequest::default()).await?;
//
//     Ok(result)
// }


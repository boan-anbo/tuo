use crate::default_workflow::default_process_separated;

mod lib;
mod main_test;


pub mod default_workflow;
pub mod custom_workflow;

#[tokio::main]
async fn main() {
    default_process_separated().await;
}
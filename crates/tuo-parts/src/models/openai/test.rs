#[cfg(test)]
mod test {
    use async_openai::Client;
    use dotenv::dotenv;
    use test_log::test;

    use tuo_core::core::messaging::message::Message;
    use tuo_core::model::model::{CompletionModelTrait, ModelTrait};
    use crate::models::openai::models::OpenAIChatModels;


    #[test(tokio::test)]
    async fn can_check_health() {
        dotenv().ok();
        let wrong_model = OpenAIChatModels::ChatGpt4_128k.get_model(None, None);
        let result = wrong_model.is_healthy().await;
        assert!(result);
    }

    #[test(tokio::test)]
    async fn list_models() {
        dotenv().ok();
        let client = Client::new();
        let result = client.models().list().await.unwrap();
        for model in result.data {
            println!("{:?}", model);
        }
    }

    #[ignore]
    #[test(tokio::test)]
    async fn basic_completion() {
        dotenv().ok();
        let model = OpenAIChatModels::ChatGpt4_128k.get_model(None, None);
        let message_text = r#"Hello, please respond with only the word "READY""#.to_string();
        let message = Message::draft(message_text, None);
        let response_message = model.complete(message).await.unwrap();
        assert_eq!(response_message.content, "READY");
    }
}

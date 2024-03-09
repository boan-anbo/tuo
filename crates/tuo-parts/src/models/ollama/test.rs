#[cfg(test)]
mod tests {
    use tuo_core::model::model::ModelTrait;
    use tuo_core::model::model_metadata::EmbeddingModelMetadataTrait;

    use crate::models::ollama::embedder::{OllamaEmbedder, OllamaEmbeddingModels};
    use crate::models::ollama::models::NomicEmbedTextVariant;

    fn crate_ollama_embedder() -> OllamaEmbedder {
        OllamaEmbeddingModels::NomicEmbedText(NomicEmbedTextVariant::Dim768).get_embedder(None)
    }

    #[tokio::test]
    async fn model_returns_health_status() {
        let embedder = crate_ollama_embedder();
        let health = embedder.is_healthy().await;
        assert!(health);
    }
}

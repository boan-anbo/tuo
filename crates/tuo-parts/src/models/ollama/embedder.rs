use ollama_rs::Ollama;
use async_trait::async_trait;
use tuo_core::embedding::embedder::EmbedderTrait;
use tuo_core::embedding::embeddings::Embeddings;
use tuo_core::model::model::ModelTrait;
use tuo_core::model::model_metadata::{EmbeddingModelMetadata, EmbeddingModelMetadataTrait};
use tuo_shared::errors::parts::TuoPartsError;
use tuo_shared::types::return_type::TuoResult;
use crate::models::ollama::models::OllamaConfig;
use crate::models::ollama::models::{ALL_MINILM_AUTHORS, ALL_MINILM_MODEL_NAME, ALL_MINILM_WEBPAGE, NOMIC_AUTHOR, NOMIC_EMBED_TEXT_MODEL_NAME, NOMIC_EMBED_TEXT_WEBPAGE, NomicEmbedTextVariant};

pub struct OllamaEmbedder {
    model_metadata: EmbeddingModelMetadata,
    client: Ollama,
}

#[async_trait]
impl ModelTrait for OllamaEmbedder {
    async fn is_healthy(&self) -> bool {
        let res = self.client.list_local_models().await;
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn get_model_name(&self) -> String {
        self.model_metadata.name.clone()
    }

    fn get_model_metadata(&self) -> EmbeddingModelMetadata {
        self.model_metadata.clone()
    }
}

impl OllamaEmbedder {
    pub fn new(model: &OllamaEmbeddingModels, _config: Option<OllamaConfig>) -> OllamaEmbedder {
        let client = Ollama::default();
        OllamaEmbedder {
            model_metadata: model.get_embedding_model(),
            client,
        }
    }
}

#[async_trait]
impl EmbedderTrait for OllamaEmbedder {
    async fn embed_string(&self, text: &str) -> TuoResult<Embeddings> {
        let res = self
            .client
            .generate_embeddings(self.model_metadata.name.clone(), text.to_string(), None)
            .await;
        match res {
            Ok(embeddings) => Ok(Embeddings::builder()
                .model(self.get_model_name())
                .vector(
                    embeddings
                        .embeddings
                        .iter()
                        .map(|x| *x as f32)
                        .collect::<Vec<f32>>(),
                )
                .build()),
            Err(e) => Err(TuoPartsError::ApiError(e.to_string()).into()),
        }
    }
}

pub enum OllamaEmbeddingModels {
    NomicEmbedText(NomicEmbedTextVariant),
    AllMiniLM,
}

impl EmbeddingModelMetadataTrait<OllamaEmbeddingModels> for OllamaEmbeddingModels {
    type Embedder = OllamaEmbedder;
    type Config = OllamaConfig;

    fn get_embedder(&self, opt: Option<Self::Config>) -> Self::Embedder {
        OllamaEmbedder::new(self, opt)
    }

    fn get_embedding_model(&self) -> EmbeddingModelMetadata {
        match self {
            OllamaEmbeddingModels::NomicEmbedText(variant) => {
                let dimensions = match variant {
                    NomicEmbedTextVariant::Dim64 => 64,
                    NomicEmbedTextVariant::Dim128 => 128,
                    NomicEmbedTextVariant::Dim256 => 256,
                    NomicEmbedTextVariant::Dim512 => 512,
                    NomicEmbedTextVariant::Dim768 => 768,
                };
                EmbeddingModelMetadata::builder()
                    .name(format!("{}-{}", NOMIC_EMBED_TEXT_MODEL_NAME, dimensions))
                    .author(NOMIC_AUTHOR.to_string())
                    .url(NOMIC_EMBED_TEXT_WEBPAGE.to_string())
                    .dimensions(dimensions)
                    .pricing_per_1k_tokens(0.0) // free
                    .max_input(8192)
                    .build()
            }
            OllamaEmbeddingModels::AllMiniLM => {
                EmbeddingModelMetadata::builder()
                    .name(ALL_MINILM_MODEL_NAME.to_string())
                    .author(ALL_MINILM_AUTHORS.to_string())
                    .url(ALL_MINILM_WEBPAGE.to_string())
                    .dimensions(384)
                    .pricing_per_1k_tokens(0.0) // free
                    .max_input(256)
                    .build()
            }
        }
    }

    fn from_model_name(name: &str) -> Option<OllamaEmbeddingModels> {
        // Implement this method to return the correct enum variant based on the model name
        match name {
            NOMIC_EMBED_TEXT_MODEL_NAME => Some(OllamaEmbeddingModels::NomicEmbedText(
                NomicEmbedTextVariant::Dim64,
            )), // Update this to handle different dimensions
            ALL_MINILM_MODEL_NAME => Some(OllamaEmbeddingModels::AllMiniLM),
            _ => None,
        }
    }

    fn get_model_name(&self) -> String {
        // Implement this method to return the correct model name based on the enum variant
        match self {
            OllamaEmbeddingModels::NomicEmbedText(variant) => {
                NOMIC_EMBED_TEXT_MODEL_NAME.to_string()
            }
            OllamaEmbeddingModels::AllMiniLM => ALL_MINILM_MODEL_NAME.to_string(),
        }
    }
}

// --- Nomic Embed Text ---
pub const NOMIC_EMBED_TEXT_MODEL_NAME: &str = "nomic-embed-text";
pub const NOMIC_AUTHOR: &str = "Nomic AI";
pub const NOMIC_EMBED_TEXT_WEBPAGE: &str = "https://huggingface.co/nomic-ai/nomic-embed-text-v1.5";

// --- All MiniLM ---
pub const ALL_MINILM_MODEL_NAME: &str = "all-minilm";
pub const ALL_MINILM_WEBPAGE: &str =
    "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2";
pub const ALL_MINILM_AUTHORS: &str = "Nils Reimers";

pub enum NomicEmbedTextVariant {
    Dim64,
    Dim128,
    Dim256,
    Dim512,
    Dim768,
}

pub struct OllamaConfig {}

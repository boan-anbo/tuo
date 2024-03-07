use tiktoken_rs::p50k_base;

pub trait TokenUtility {
    fn count_tokens(&self) -> usize;
}

// lazy once cell
// let bpe = p50k_base().unwrap();
static BPE: once_cell::sync::Lazy<tiktoken_rs::CoreBPE> =
    once_cell::sync::Lazy::new(|| p50k_base().unwrap());
pub fn count_tokens(text: &str) -> usize {
    BPE.encode_with_special_tokens(text).len()
}

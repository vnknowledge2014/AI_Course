use anyhow::Result;
use async_trait::async_trait;
use rig::providers::openai::Client;

/// The Strategy Pattern for Document Parsing
#[async_trait]
pub trait DocumentParser {
    async fn parse(&self, file_path: &str) -> Result<String>;
}

/// Strategy for Vision-native models (e.g., GPT-4o, Claude 3.5 Sonnet)
/// Renders PDF pages as images and passes them directly to the LLM for lossless layout understanding.
pub struct VisionNativeStrategy {
    pub client: Client, // Rig Client capable of vision
}

#[async_trait]
impl DocumentParser for VisionNativeStrategy {
    async fn parse(&self, file_path: &str) -> Result<String> {
        println!("Extracting {} using Native Vision...", file_path);
        // Concept: Render PDF to image -> Call Rig Vision API
        Ok("Parsed via Vision".to_string())
    }
}

/// Strategy for Text-only models (e.g., Llama 3)
/// Pipes the document through an OCR engine (e.g. Tesseract) before feeding text to the LLM.
pub struct OcrFallbackStrategy {
    pub ocr_engine_path: String,
}

#[async_trait]
impl DocumentParser for OcrFallbackStrategy {
    async fn parse(&self, file_path: &str) -> Result<String> {
        println!("Extracting {} using OCR Fallback...", file_path);
        // Concept: Call Tesseract -> Feed text to LLM
        Ok("Parsed via OCR".to_string())
    }
}

/// Computes the delta ($\Delta$) for Incremental Sync.
pub fn compute_delta(file_path: &str, old_hash: &str) -> Result<bool> {
    // Concept: Hash the AST or Markdown sections, compare with `old_hash`
    // Only re-index if there is a change.
    Ok(true)
}

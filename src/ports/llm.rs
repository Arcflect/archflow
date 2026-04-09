/// Request model for an LLM completion operation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
}

/// Response model for an LLM completion operation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
}

/// LLM capability boundary.
#[allow(dead_code)]
pub trait LlmPort {
    type Error;

    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, Self::Error>;
}

pub struct NoopLlmAdapter;

impl crate::ports::LlmPort for NoopLlmAdapter {
    type Error = String;

    fn complete(
        &self,
        _request: &crate::ports::LlmRequest,
    ) -> Result<crate::ports::LlmResponse, Self::Error> {
        Err("No LLM adapter configured".to_string())
    }
}

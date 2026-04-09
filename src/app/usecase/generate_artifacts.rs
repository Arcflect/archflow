#[derive(Debug, Clone, Default)]
pub struct GenerateArtifactsInput;

#[derive(Debug, Clone)]
pub struct GenerateArtifactsOutput {
    pub success: bool,
}

pub struct GenerateArtifactsUseCase;

impl GenerateArtifactsUseCase {
    pub fn execute(_input: GenerateArtifactsInput) -> GenerateArtifactsOutput {
        crate::commands::scaffold::execute();
        GenerateArtifactsOutput { success: true }
    }
}

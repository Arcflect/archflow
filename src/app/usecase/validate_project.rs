#[derive(Debug, Clone, Default)]
pub struct ValidateProjectInput;

#[derive(Debug, Clone)]
pub struct ValidateProjectOutput {
    pub success: bool,
}

pub struct ValidateProjectUseCase;

impl ValidateProjectUseCase {
    pub fn execute(_input: ValidateProjectInput) -> ValidateProjectOutput {
        crate::commands::verify::execute();
        ValidateProjectOutput { success: true }
    }
}

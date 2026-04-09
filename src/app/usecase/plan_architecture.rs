#[derive(Debug, Clone, Default)]
pub struct PlanArchitectureInput;

#[derive(Debug, Clone)]
pub struct PlanArchitectureOutput {
    pub success: bool,
}

pub struct PlanArchitectureUseCase;

impl PlanArchitectureUseCase {
    pub fn execute(_input: PlanArchitectureInput) -> PlanArchitectureOutput {
        crate::commands::plan::execute();
        PlanArchitectureOutput { success: true }
    }
}

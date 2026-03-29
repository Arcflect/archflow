use serde::{Deserialize, Serialize};

/// The internal Prompt representation used during Phase 3 generation.
/// This model acts strictly down-stream from contracts; it is not a parallel source of truth.
/// It translates the semantic constraints of a contract into an AI-ready delivery payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub artifact_name: String,
    pub role: String,
    pub module: String,
    
    // Core constraints (inherited from Contract or Role expectations)
    pub responsibilities: Vec<String>,
    pub must_not: Vec<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_dependencies: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_dependencies: Option<Vec<String>>,

    // Artifact interface boundaries (downstream from Artifact definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,

    // Final AI fulfillment checks (derived from contract intent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<Vec<String>>,
}

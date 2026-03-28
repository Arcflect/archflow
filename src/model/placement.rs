use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePlacement {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<SidecarPlacement>,
}

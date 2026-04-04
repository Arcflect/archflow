use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::error::ConfigError;

pub const SUPPORTED_GUARD_SIDECAR_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardSidecarConfig {
    pub version: u32,
    pub hooks: GuardHooks,
    #[serde(default)]
    pub checks: GuardChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardHooks {
    pub init: bool,
    pub plan: bool,
    pub ci: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardChecks {
    #[serde(default = "default_true")]
    pub require_contracts_template: bool,
    #[serde(default = "default_true")]
    pub require_role_templates_for_artifact_roles: bool,
    #[serde(default = "default_true")]
    pub enforce_sidecar_suffixes: bool,
}

fn default_true() -> bool {
    true
}

impl Default for GuardChecks {
    fn default() -> Self {
        Self {
            require_contracts_template: true,
            require_role_templates_for_artifact_roles: true,
            enforce_sidecar_suffixes: true,
        }
    }
}

impl GuardSidecarConfig {
    pub fn load_optional<P: AsRef<Path>>(path: P) -> Result<Option<Self>, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let config: GuardSidecarConfig =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
                path: path.to_path_buf(),
                source: e,
            })?;

        config.validate(path)?;
        Ok(Some(config))
    }

    pub fn default_fallback() -> Self {
        Self {
            version: SUPPORTED_GUARD_SIDECAR_VERSION,
            hooks: GuardHooks {
                init: true,
                plan: true,
                ci: true,
            },
            checks: GuardChecks::default(),
        }
    }

    fn validate<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref().to_path_buf();
        if self.version != SUPPORTED_GUARD_SIDECAR_VERSION {
            return Err(ConfigError::Validation {
                path,
                message: format!(
                    "guard sidecar version must be '{}' (got '{}')",
                    SUPPORTED_GUARD_SIDECAR_VERSION, self.version
                ),
            });
        }

        Ok(())
    }
}

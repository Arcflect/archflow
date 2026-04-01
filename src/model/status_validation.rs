use crate::model::verify::{CheckResult, VerifyStatus, VerifyTarget};

pub const ALLOWED_STATUS_VALUES: [&str; 5] = [
    "planned",
    "scaffolded",
    "implementing",
    "reviewing",
    "done",
];

fn is_allowed_status(status: &str) -> bool {
    ALLOWED_STATUS_VALUES.contains(&status)
}

pub fn validate_artifact_status(
    artifact_name: &str,
    artifact_status: Option<&str>,
) -> CheckResult {
    match artifact_status {
        Some(status) if is_allowed_status(status) => CheckResult {
            check_id: "artifact-status-valid".to_string(),
            target: VerifyTarget::Artifact {
                name: artifact_name.to_string(),
            },
            status: VerifyStatus::Pass,
            message: format!("Artifact status is valid for {}: '{}'", artifact_name, status),
        },
        Some(status) => CheckResult {
            check_id: "artifact-status-valid".to_string(),
            target: VerifyTarget::Artifact {
                name: artifact_name.to_string(),
            },
            status: VerifyStatus::Fail,
            message: format!(
                "Artifact status is invalid for {}: '{}' (allowed: {})",
                artifact_name,
                status,
                ALLOWED_STATUS_VALUES.join(", ")
            ),
        },
        None => CheckResult {
            check_id: "artifact-status-valid".to_string(),
            target: VerifyTarget::Artifact {
                name: artifact_name.to_string(),
            },
            status: VerifyStatus::Skip,
            message: format!(
                "Artifact status is not set for {} (optional in artifacts.plan.yaml)",
                artifact_name
            ),
        },
    }
}

pub fn validate_contract_status(
    artifact_name: &str,
    contract_path: &str,
    contract_status: Option<&str>,
) -> CheckResult {
    match contract_status {
        Some(status) if is_allowed_status(status) => CheckResult {
            check_id: "contract-status-valid".to_string(),
            target: VerifyTarget::Contract {
                artifact_name: artifact_name.to_string(),
                path: contract_path.to_string(),
            },
            status: VerifyStatus::Pass,
            message: format!("Contract status is valid for {}: '{}'", artifact_name, status),
        },
        Some(status) => CheckResult {
            check_id: "contract-status-valid".to_string(),
            target: VerifyTarget::Contract {
                artifact_name: artifact_name.to_string(),
                path: contract_path.to_string(),
            },
            status: VerifyStatus::Fail,
            message: format!(
                "Contract status is invalid for {}: '{}' (allowed: {})",
                artifact_name,
                status,
                ALLOWED_STATUS_VALUES.join(", ")
            ),
        },
        None => CheckResult {
            check_id: "contract-status-valid".to_string(),
            target: VerifyTarget::Contract {
                artifact_name: artifact_name.to_string(),
                path: contract_path.to_string(),
            },
            status: VerifyStatus::Fail,
            message: format!("Contract status is missing for {} (required)", artifact_name),
        },
    }
}

pub fn validate_artifact_contract_status_consistency(
    artifact_name: &str,
    artifact_status: Option<&str>,
    contract_path: &str,
    contract_status: Option<&str>,
) -> CheckResult {
    let target = VerifyTarget::Contract {
        artifact_name: artifact_name.to_string(),
        path: contract_path.to_string(),
    };

    match (artifact_status, contract_status) {
        (Some(a), Some(c)) if is_allowed_status(a) && is_allowed_status(c) && a == c => {
            CheckResult {
                check_id: "artifact-contract-status-consistent".to_string(),
                target,
                status: VerifyStatus::Pass,
                message: format!(
                    "Artifact and contract status are consistent for {}: '{}'",
                    artifact_name, a
                ),
            }
        }
        (Some(a), Some(c)) if is_allowed_status(a) && is_allowed_status(c) => CheckResult {
            check_id: "artifact-contract-status-consistent".to_string(),
            target,
            status: VerifyStatus::Fail,
            message: format!(
                "Status conflict for {}: artifact='{}', contract='{}'",
                artifact_name, a, c
            ),
        },
        (None, Some(_)) => CheckResult {
            check_id: "artifact-contract-status-consistent".to_string(),
            target,
            status: VerifyStatus::Warn,
            message: format!(
                "Artifact status is missing for {} while contract status exists",
                artifact_name
            ),
        },
        _ => CheckResult {
            check_id: "artifact-contract-status-consistent".to_string(),
            target,
            status: VerifyStatus::Skip,
            message: format!(
                "Status consistency check skipped for {} due to missing or invalid status values",
                artifact_name
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_status_pass_for_allowed_value() {
        let result = validate_artifact_status("user", Some("planned"));
        assert_eq!(result.status, VerifyStatus::Pass);
    }

    #[test]
    fn artifact_status_fail_for_invalid_value() {
        let result = validate_artifact_status("user", Some("unknown"));
        assert_eq!(result.status, VerifyStatus::Fail);
    }

    #[test]
    fn contract_status_fail_when_missing() {
        let result = validate_contract_status("user", "user.contract.yaml", None);
        assert_eq!(result.status, VerifyStatus::Fail);
    }

    #[test]
    fn status_consistency_pass_when_equal() {
        let result = validate_artifact_contract_status_consistency(
            "user",
            Some("implementing"),
            "user.contract.yaml",
            Some("implementing"),
        );
        assert_eq!(result.status, VerifyStatus::Pass);
    }

    #[test]
    fn status_consistency_fail_when_different() {
        let result = validate_artifact_contract_status_consistency(
            "user",
            Some("planned"),
            "user.contract.yaml",
            Some("done"),
        );
        assert_eq!(result.status, VerifyStatus::Fail);
    }

    #[test]
    fn status_consistency_warn_when_artifact_missing() {
        let result = validate_artifact_contract_status_consistency(
            "user",
            None,
            "user.contract.yaml",
            Some("planned"),
        );
        assert_eq!(result.status, VerifyStatus::Warn);
    }
}
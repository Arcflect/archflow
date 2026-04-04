use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::artifact::Artifact;
use crate::model::placement::RolePlacement;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Error,
    Warn,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
        }
    }
}

#[derive(Debug, Clone)]
struct AuditFinding {
    rule_id: &'static str,
    severity: Severity,
    target: String,
    message: String,
    remediation: String,
}

pub fn execute(strict: bool) {
    let mut findings = Vec::new();

    check_required_root_file("project.arch.yaml", &mut findings);
    check_required_root_file("placement.rules.yaml", &mut findings);
    check_required_root_file("artifacts.plan.yaml", &mut findings);
    check_required_root_file("contracts.template.yaml", &mut findings);

    if findings.iter().any(|f| f.severity == Severity::Error) {
        render_report(&findings);
        std::process::exit(1);
    }

    let project_config = match ProjectConfig::load("project.arch.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "project-config-valid",
                severity: Severity::Error,
                target: "project.arch.yaml".to_string(),
                message: format!("project configuration is invalid: {}", err),
                remediation: "Fix project.arch.yaml based on the validation message and rerun `archflow audit`.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    let placement_config = match PlacementRulesConfig::load("placement.rules.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "placement-config-valid",
                severity: Severity::Error,
                target: "placement.rules.yaml".to_string(),
                message: format!("placement rules are invalid: {}", err),
                remediation: "Fix placement.rules.yaml and ensure each role has a valid path and optional extension.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    let artifacts_config = match ArtifactsPlanConfig::load("artifacts.plan.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "artifacts-plan-valid",
                severity: Severity::Error,
                target: "artifacts.plan.yaml".to_string(),
                message: format!("artifacts plan is invalid: {}", err),
                remediation: "Fix artifacts.plan.yaml and ensure each artifact has name/module/role.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    findings.extend(run_baseline_audit(
        &project_config,
        &placement_config,
        &artifacts_config,
    ));

    render_report(&findings);

    let has_errors = findings
        .iter()
        .any(|finding| finding.severity == Severity::Error);
    let has_warnings = findings
        .iter()
        .any(|finding| finding.severity == Severity::Warn);

    if has_errors || (strict && has_warnings) {
        std::process::exit(1);
    }
}

fn check_required_root_file(filename: &str, findings: &mut Vec<AuditFinding>) {
    if !Path::new(filename).exists() {
        findings.push(AuditFinding {
            rule_id: "required-root-file",
            severity: Severity::Error,
            target: filename.to_string(),
            message: format!("missing required root file: {}", filename),
            remediation: format!("Run `archflow init` to generate missing files, or add {} manually.", filename),
        });
    }
}

fn run_baseline_audit(
    project_config: &ProjectConfig,
    placement_config: &PlacementRulesConfig,
    artifacts_config: &ArtifactsPlanConfig,
) -> Vec<AuditFinding> {
    let mut findings = Vec::new();

    for artifact in &artifacts_config.artifacts {
        if !project_config.has_module(&artifact.module) {
            findings.push(AuditFinding {
                rule_id: "artifact-module-defined",
                severity: Severity::Error,
                target: format!("artifact:{}", artifact.name),
                message: format!(
                    "artifact '{}' references undefined module '{}'",
                    artifact.name, artifact.module
                ),
                remediation: format!(
                    "Add module '{}' to project.arch.yaml or update artifact '{}' to an existing module.",
                    artifact.module, artifact.name
                ),
            });
        }

        let role_config = match placement_config.roles.get(&artifact.role) {
            Some(role_config) => role_config,
            None => {
                findings.push(AuditFinding {
                    rule_id: "artifact-role-defined",
                    severity: Severity::Error,
                    target: format!("artifact:{}", artifact.name),
                    message: format!(
                        "artifact '{}' uses undefined role '{}'",
                        artifact.name, artifact.role
                    ),
                    remediation: format!(
                        "Define role '{}' in placement.rules.yaml or change artifact '{}' role.",
                        artifact.role, artifact.name
                    ),
                });
                continue;
            }
        };

        if let Some(explicit_path) = artifact.path.as_deref() {
            let expected = expected_role_path(artifact, role_config);
            if explicit_path != expected {
                findings.push(AuditFinding {
                    rule_id: "artifact-path-aligns-role",
                    severity: Severity::Warn,
                    target: format!("artifact:{}", artifact.name),
                    message: format!(
                        "explicit path '{}' deviates from role '{}' expected path '{}'",
                        explicit_path, artifact.role, expected
                    ),
                    remediation: "Either align artifact.path with role defaults or document the exception in project policy.".to_string(),
                });
            }
        }
    }

    findings
}

fn expected_role_path(artifact: &Artifact, role: &RolePlacement) -> String {
    let mut path = role.path.clone();
    if !path.ends_with('/') {
        path.push('/');
    }
    match role.file_extension.as_deref() {
        Some(ext) => format!("{}{}.{}", path, artifact.name, ext),
        None => format!("{}{}", path, artifact.name),
    }
}

fn render_report(findings: &[AuditFinding]) {
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Warn)
        .count();

    println!("Archflow Audit Report");
    println!("=====================");
    println!(
        "Summary: {} issue(s) detected (errors={}, warnings={})",
        findings.len(), errors, warnings
    );

    if findings.is_empty() {
        println!("Status: PASSED");
        return;
    }

    println!("Status: {}", if errors > 0 { "FAILED" } else { "PASSED WITH WARNINGS" });
    println!();

    for finding in findings {
        println!(
            "- [{}][{}] {}",
            finding.rule_id,
            finding.severity.as_str(),
            finding.message
        );
        println!("  target: {}", finding.target);
        println!("  remediation: {}", finding.remediation);
    }
}

#[cfg(test)]
mod tests {
    use super::{run_baseline_audit, Severity};
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
    use crate::model::artifact::Artifact;
    use crate::model::placement::RolePlacement;
    use crate::model::project::{Module, Project};
    use std::collections::HashMap;

    fn base_project() -> ProjectConfig {
        ProjectConfig {
            archflow: Some(crate::config::project::ArchflowMetadata {
                schema_version: crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION.to_string(),
                preset: None,
            }),
            project: Project {
                name: "demo-app".to_string(),
                architecture_style: "layered".to_string(),
                language: "generic".to_string(),
            },
            workspace: None,
            modules: vec![Module {
                name: "user".to_string(),
                features: None,
            }],
            metadata: None,
        }
    }

    #[test]
    fn baseline_audit_reports_undefined_module_and_role() {
        let project = base_project();
        let placement = PlacementRulesConfig { roles: HashMap::new() };
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_order".to_string(),
                module: "order".to_string(),
                role: "usecase".to_string(),
                path: None,
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let findings = run_baseline_audit(&project, &placement, &artifacts);

        assert_eq!(findings.len(), 2);
        assert!(findings.iter().any(|f| f.rule_id == "artifact-module-defined" && f.severity == Severity::Error));
        assert!(findings.iter().any(|f| f.rule_id == "artifact-role-defined" && f.severity == Severity::Error));
    }

    #[test]
    fn baseline_audit_warns_for_path_deviation() {
        let project = base_project();
        let mut roles = HashMap::new();
        roles.insert(
            "usecase".to_string(),
            RolePlacement {
                path: "src/application/usecases".to_string(),
                file_extension: Some("rs".to_string()),
                sidecar: None,
            },
        );
        let placement = PlacementRulesConfig { roles };
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_user".to_string(),
                module: "user".to_string(),
                role: "usecase".to_string(),
                path: Some("custom/path/create_user.rs".to_string()),
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let findings = run_baseline_audit(&project, &placement, &artifacts);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "artifact-path-aligns-role");
        assert_eq!(findings[0].severity, Severity::Warn);
        assert!(!findings[0].remediation.is_empty());
    }
}

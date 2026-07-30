use std::collections::BTreeMap;

use rust_ai_engineering::orchestration::{
    AgentError, AgentRun, McpManifest, ToolError, ToolRegistry, ToolRequest, ToolSpec,
};

#[test]
fn registry_accepts_only_declared_tools_and_arguments() {
    let mut registry = ToolRegistry::default();
    registry
        .register(ToolSpec::new("buscar", ["consulta"]))
        .expect("herramienta única");
    let request = ToolRequest::new(
        "buscar",
        BTreeMap::from([("consulta".into(), "rust".into())]),
    );

    assert_eq!(registry.validate(&request), Ok(()));
}

#[test]
fn registry_rejects_an_argument_outside_the_capability_contract() {
    let mut registry = ToolRegistry::default();
    registry
        .register(ToolSpec::new("buscar", ["consulta"]))
        .expect("herramienta única");
    let request = ToolRequest::new("buscar", BTreeMap::from([("borrar".into(), "todo".into())]));

    assert_eq!(
        registry.validate(&request),
        Err(ToolError::UndeclaredArgument)
    );
}

#[test]
fn agent_stops_after_its_declared_action_budget() {
    let mut run = AgentRun::new(1).expect("presupuesto válido");
    run.record(ToolRequest::new("buscar", BTreeMap::new()))
        .expect("primera acción");

    assert_eq!(
        run.record(ToolRequest::new("buscar", BTreeMap::new())),
        Err(AgentError::BudgetExhausted)
    );
}

#[test]
fn mcp_manifest_exposes_only_declared_capabilities() {
    let manifest = McpManifest::new("local", ["buscar", "leer"]).expect("manifiesto válido");

    assert!(manifest.allows("buscar"));
    assert!(!manifest.allows("escribir"));
}

use std::collections::BTreeMap;

use rust_ai_engineering::orchestration::{ToolError, ToolRegistry, ToolRequest, ToolSpec};

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

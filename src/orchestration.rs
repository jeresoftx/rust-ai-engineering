//! Contratos locales para herramientas y agentes acotados.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: String,
    allowed_args: BTreeSet<String>,
}

impl ToolSpec {
    pub fn new(
        name: impl Into<String>,
        allowed_args: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            allowed_args: allowed_args.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolRequest {
    pub name: String,
    pub args: BTreeMap<String, String>,
}

impl ToolRequest {
    pub fn new(name: impl Into<String>, args: BTreeMap<String, String>) -> Self {
        Self {
            name: name.into(),
            args,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolError {
    DuplicateTool,
    UnknownTool,
    UndeclaredArgument,
    EmptyArgument,
}

#[derive(Clone, Debug, Default)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolSpec>,
}

impl ToolRegistry {
    pub fn register(&mut self, spec: ToolSpec) -> Result<(), ToolError> {
        if self.tools.contains_key(&spec.name) {
            return Err(ToolError::DuplicateTool);
        }
        self.tools.insert(spec.name.clone(), spec);
        Ok(())
    }

    /// Valida una petición declarativa; no ejecuta la herramienta.
    pub fn validate(&self, request: &ToolRequest) -> Result<(), ToolError> {
        let spec = self
            .tools
            .get(&request.name)
            .ok_or(ToolError::UnknownTool)?;
        if request
            .args
            .keys()
            .any(|key| !spec.allowed_args.contains(key))
        {
            return Err(ToolError::UndeclaredArgument);
        }
        if request.args.values().any(|value| value.trim().is_empty()) {
            return Err(ToolError::EmptyArgument);
        }
        Ok(())
    }
}

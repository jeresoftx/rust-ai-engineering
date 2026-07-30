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

/// Registro local de acciones ya aceptadas por un agente.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentRun {
    budget: usize,
    actions: Vec<ToolRequest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentError {
    EmptyBudget,
    BudgetExhausted,
}

impl AgentRun {
    pub fn new(budget: usize) -> Result<Self, AgentError> {
        if budget == 0 {
            return Err(AgentError::EmptyBudget);
        }
        Ok(Self {
            budget,
            actions: Vec::new(),
        })
    }

    /// Registra una intención; ejecutar una herramienta sigue siendo otra decisión.
    pub fn record(&mut self, request: ToolRequest) -> Result<(), AgentError> {
        if self.actions.len() == self.budget {
            return Err(AgentError::BudgetExhausted);
        }
        self.actions.push(request);
        Ok(())
    }
}

/// Un contrato MCP mínimo que hace visibles las capacidades ofrecidas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct McpManifest {
    pub server: String,
    capabilities: BTreeSet<String>,
}

impl McpManifest {
    pub fn new(
        server: impl Into<String>,
        capabilities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, AgentError> {
        let server = server.into();
        let capabilities = capabilities
            .into_iter()
            .map(Into::into)
            .collect::<BTreeSet<_>>();
        if server.trim().is_empty() || capabilities.is_empty() {
            return Err(AgentError::EmptyBudget);
        }
        Ok(Self {
            server,
            capabilities,
        })
    }

    pub fn allows(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }
}

//! Construcción local de contexto con presupuesto y procedencia.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextFragment {
    pub source: String,
    pub text: String,
}

impl ContextFragment {
    pub fn new(source: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextBundle {
    pub text: String,
    pub sources: Vec<String>,
    pub used_chars: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextError {
    EmptyBudget,
    MissingSource,
    EmptyFragment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextBuilder {
    budget: usize,
}

impl ContextBuilder {
    pub fn new(budget: usize) -> Result<Self, ContextError> {
        if budget == 0 {
            return Err(ContextError::EmptyBudget);
        }
        Ok(Self { budget })
    }

    /// Conserva únicamente fragmentos completos que caben en el presupuesto.
    pub fn assemble(
        &self,
        fragments: impl IntoIterator<Item = ContextFragment>,
    ) -> Result<ContextBundle, ContextError> {
        let mut text = Vec::new();
        let mut sources = Vec::new();
        let mut used_chars = 0;

        for fragment in fragments {
            if fragment.source.trim().is_empty() {
                return Err(ContextError::MissingSource);
            }
            if fragment.text.trim().is_empty() {
                return Err(ContextError::EmptyFragment);
            }
            let chars = fragment.text.chars().count();
            if used_chars + chars > self.budget {
                continue;
            }
            used_chars += chars;
            sources.push(fragment.source);
            text.push(fragment.text);
        }

        Ok(ContextBundle {
            text: text.join("\n"),
            sources,
            used_chars,
        })
    }
}

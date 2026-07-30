//! Evaluación local y determinista de contratos didácticos.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvaluationCase {
    pub id: String,
    pub expected: bool,
    pub observed: bool,
}

impl EvaluationCase {
    pub fn new(id: impl Into<String>, expected: bool, observed: bool) -> Self {
        Self {
            id: id.into(),
            expected,
            observed,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluationError {
    EmptySuite,
    EmptyId,
}

/// Resume casos deterministas; la tasa no sustituye revisión de cobertura.
pub fn evaluate(
    cases: impl IntoIterator<Item = EvaluationCase>,
) -> Result<EvaluationSummary, EvaluationError> {
    let cases = cases.into_iter().collect::<Vec<_>>();
    if cases.is_empty() {
        return Err(EvaluationError::EmptySuite);
    }
    if cases.iter().any(|case| case.id.trim().is_empty()) {
        return Err(EvaluationError::EmptyId);
    }

    let total = cases.len();
    let passed = cases
        .iter()
        .filter(|case| case.expected == case.observed)
        .count();
    let failed = total - passed;
    Ok(EvaluationSummary {
        total,
        passed,
        failed,
        pass_rate: passed as f32 / total as f32,
    })
}

use rust_ai_engineering::evaluation::{EvaluationCase, EvaluationError, evaluate};

#[test]
fn evaluation_reports_passes_failures_and_rate() {
    let summary = evaluate([
        EvaluationCase::new("fuente", true, true),
        EvaluationCase::new("presupuesto", true, false),
    ])
    .expect("casos válidos");

    assert_eq!(summary.total, 2);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.pass_rate, 0.5);
}

#[test]
fn evaluation_rejects_an_empty_suite() {
    assert_eq!(evaluate([]), Err(EvaluationError::EmptySuite));
}

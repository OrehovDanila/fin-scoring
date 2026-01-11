use scoring_engine::scoring::model::{ScoringInput, ScoringOutput};
use scoring_engine::engine;

#[test]
fn integration_basic_case() {
    let input = ScoringInput {
        age: 25,
        income: 50_000.0,
        credit_history_months: 60,
        late_payments: 0,
        ..Default::default()
    };

    let output: ScoringOutput = engine::score(input);

    // Проверяем, что score > 0
    assert!(output.score > 0);

    // Проверяем, что присутствуют причины
    assert!(!output.reasons.is_empty());
}

#[test]
fn integration_age_dependency() {
    let input = ScoringInput {
        age: 20, // Age < 21
        income: 50_000.0,
        credit_history_months: 60,
        late_payments: 0,
        ..Default::default()
    };
    //TODO Доделать как появится правило с зависимостью

    let output = engine::score(input);
    let age_result = output.reasons.iter().find(|r| r.contains("age"));
    assert!(age_result.is_some());
    let income_result = output.reasons.iter().find(|r| r.contains("income"));
    assert!(income_result.is_some());
}
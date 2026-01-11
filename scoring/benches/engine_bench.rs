use criterion::{criterion_group, criterion_main, Criterion};
use scoring_engine::engine;
use scoring_engine::scoring::model::ScoringInput;

fn bench_small_dag(c: &mut Criterion) {
    let input = ScoringInput::default();

    let debug_output = engine::score(input.clone());
    println!(
        "[DEBUG] small DAG -> Score: {}, Reasons: {:?}",
        debug_output.score, debug_output.reasons
    );

    c.bench_function("engine small DAG", |b| {
        b.iter(|| engine::score(input.clone()))
    });
}

fn bench_large_dag(c: &mut Criterion) {
    let input = ScoringInput {
        age: 35,
        income: 100_000.0,
        credit_history_months: 120,
        late_payments: 0,
        ..Default::default()
    };

    let debug_output = engine::score(input.clone());
    println!(
        "[DEBUG] large DAG -> Score: {}, Reasons: {:?}",
        debug_output.score, debug_output.reasons
    );

    c.bench_function("engine large DAG", |b| {
        b.iter(|| engine::score(input.clone()))
    });
}

criterion_group!(benches, bench_small_dag, bench_large_dag);
criterion_main!(benches);
use crate::rules::*;
use crate::scoring::decision;
use crate::scoring::model::ScoringInput;
use crate::scoring::model::ScoringOutput;
use rayon::prelude::*;
use std::time::Instant;
use tracing::{debug, instrument};
use crate::model::RuleResult;
use crate::rules::rules::{RuleContext, RuleId, ScoringRule};
use crate::scoring::rule_node::build_layers;


#[instrument(name = "scoring.engine", skip(req))]
pub fn score(req: ScoringInput) -> ScoringOutput {
    let start = Instant::now();

    // Вектор функций, каждая считает один компонент скоринга
    let components_fns: Vec<Box<dyn ScoringRule + Send + Sync>> = vec![
        // Можно избавится от dyn если будет хороший енам
        Box::new(age::AgeRule),
        Box::new(income::IncomeRule),
        Box::new(history::CreditHistoryRule),
        Box::new(late_payment::LatePaymentRule),
    ];

    let layers = build_layers(components_fns).expect("invalid rule DAG");

    let mut ctx = RuleContext::default();
    let mut total = RuleResult::default();

    for layer in layers {
        // Параллельно считаем слой
        let results: Vec<(RuleId, RuleResult)> = layer
            .par_iter()
            .map(|rule| {
                let result = rule.score(&req, &ctx);
                (rule.id(), result)
            })
            .collect();

        for (id, result) in results {
            ctx.insert(id, result.clone());
            total += result;
        }
    }

    let (decision, score, reasons) = decision::decision(total);

    debug!( total_score = score, elapsed_ms = start.elapsed().as_millis(), "engine finished" );

    ScoringOutput {
        score,
        decision,
        reasons,
    }
}

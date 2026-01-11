use std::collections::HashMap;
use crate::model::{RuleResult, ScoringInput};

pub trait ScoringRule {
    fn score(&self, value: &ScoringInput, ctx: &RuleContext,) -> RuleResult;
    fn id(&self) -> RuleId;

    fn dependencies(&self) -> &[RuleId] {
        &[]
    }
}

/// Все правила должны иметь свой уникальный Id в этом enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Age,
    CreditHistory,
    Income,
    LatePayments
}




/// Контекст с результатами прочих правил для сложных правил
#[derive(Default)]
pub struct RuleContext {
    results: HashMap<RuleId, RuleResult>,
}

impl RuleContext {
    pub fn get(&self, id: RuleId) -> Option<&RuleResult> {
        self.results.get(&id)
    }

    pub fn insert(&mut self, id: RuleId, result: RuleResult) {
        self.results.insert(id, result);
    }
}
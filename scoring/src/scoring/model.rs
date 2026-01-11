use std::iter::Sum;
use std::ops::AddAssign;

#[derive(Default, Clone)]
pub struct ScoringInput {
    pub age: u32,
    pub income: f64,
    pub credit_history_months: u32,
    pub late_payments: u32,
    pub total_debt: f64,
    pub last_transactions: Vec<f64>,
    pub account_age_months: u32,
    pub last_6_month_late_payments: Vec<u32>,
}

pub struct ScoringOutput {
    pub score: u32,
    pub decision: i32,
    pub reasons: Vec<String>,
}

#[derive(Default, Clone)]
pub struct RuleResult {
    pub score: u32,
    pub reasons: Vec<String>,
}


impl Sum for RuleResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            RuleResult {
                score: 0,
                reasons: Vec::new(),
            },
            |mut acc, r| {
                acc.score += r.score;
                acc.reasons.extend(r.reasons);
                acc
            },
        )
    }
}

impl AddAssign for RuleResult {
    fn add_assign(&mut self, rhs: Self) {
        self.score += rhs.score;
        self.reasons.extend(rhs.reasons);
    }
}

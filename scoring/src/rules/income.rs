use crate::model::{RuleResult, ScoringInput};
use crate::rules::rules::RuleId;
use crate::rules::rules::{RuleContext, ScoringRule};

pub struct IncomeRule;

impl ScoringRule for IncomeRule {
    fn score(&self, value: &ScoringInput, _ctx: &RuleContext) -> RuleResult {
        let income = value.income;
        let score = (income / 1000.0).min(300.0) as u32;

        RuleResult {
            score,
            reasons: vec![format!("income = {}", income)],
        }
    }

    fn id(&self) -> RuleId {
        RuleId::Income
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn input(income: f64) -> ScoringInput {
        ScoringInput {
            income,
            age: 0,
            credit_history_months: 0,
            late_payments: 0,
            total_debt: 0.0,
            last_transactions: vec![],
            account_age_months: 0,
            last_6_month_late_payments: vec![],
        }
    }

    #[rstest]
    #[case(0.0, 0)]
    #[case(1_000.0, 1)]
    #[case(50_000.0, 50)]
    #[case(300_000.0, 300)]
    #[case(1_000_000.0, 300)]
    fn test_income_rule(#[case] income: f64, #[case] expected: u32) {
        let rule = IncomeRule;
        let ctx = RuleContext::default();

        let result = rule.score(&input(income), &ctx);

        assert_eq!(result.score, expected);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_income_rule_fuzz(income in 0.0f64..1_000_000.0) {
            let rule = IncomeRule;
            let ctx = RuleContext::default();
            let mut input = ScoringInput::default();
            input.income = income;

            let score = rule.score(&input, &ctx).score;
            let expected = (income / 1000.0).min(300.0) as u32;

            prop_assert_eq!(score, expected);
        }
    }
}

use crate::model::{RuleResult, ScoringInput};
use crate::rules::rules::RuleId;
use crate::rules::rules::{RuleContext, ScoringRule};

pub struct CreditHistoryRule;

impl ScoringRule for CreditHistoryRule {
    fn score(&self, value: &ScoringInput, _ctx: &RuleContext) -> RuleResult {
        let months = value.credit_history_months;
        let score = (months * 5).min(300);

        RuleResult {
            score,
            reasons: vec![format!("credit_history_months = {}", months)],
        }
    }

    fn id(&self) -> RuleId {
        RuleId::CreditHistory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn input(months: u32) -> ScoringInput {
        ScoringInput {
            credit_history_months: months,
            age: 0,
            income: 0.0,
            late_payments: 0,
            total_debt: 0.0,
            last_transactions: vec![],
            account_age_months: 0,
            last_6_month_late_payments: vec![],
        }
    }

    #[rstest]
    #[case(0, 0)]
    #[case(10, 50)]
    #[case(60, 300)]
    #[case(100, 300)]
    fn test_credit_history_rule(#[case] months: u32, #[case] expected: u32) {
        let rule = CreditHistoryRule;
        let ctx = RuleContext::default();

        let result = rule.score(&input(months), &ctx);

        assert_eq!(result.score, expected);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_credit_history_rule_fuzz(months in 0u32..=100) {
            let rule = CreditHistoryRule;
            let ctx = RuleContext::default();
            let mut input = ScoringInput::default();
            input.credit_history_months = months;

            let score = rule.score(&input, &ctx).score;

            if months >= 60 {
                prop_assert_eq!(score, 300);
            } else {
                prop_assert_eq!(score, months * 5);
            }
        }
    }
}

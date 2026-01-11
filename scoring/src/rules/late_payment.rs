use crate::model::{RuleResult, ScoringInput};
use crate::rules::rules::RuleId;
use crate::rules::rules::{RuleContext, ScoringRule};

pub struct LatePaymentRule;

impl ScoringRule for LatePaymentRule {
    fn score(&self, value: &ScoringInput, _ctx: &RuleContext) -> RuleResult {
        let late = value.late_payments;
        let score = if late == 0 { 200 } else { 0 };

        RuleResult {
            score,
            reasons: vec![format!("late_payments = {}", late)],
        }
    }

    fn id(&self) -> RuleId {
        RuleId::LatePayments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn input(late: u32) -> ScoringInput {
        ScoringInput {
            late_payments: late,
            age: 0,
            income: 0.0,
            credit_history_months: 0,
            total_debt: 0.0,
            last_transactions: vec![],
            account_age_months: 0,
            last_6_month_late_payments: vec![],
        }
    }

    #[rstest]
    #[case(0, 200)]
    #[case(1, 0)]
    #[case(5, 0)]
    #[case(100, 0)]
    fn test_late_payment_rule(#[case] late: u32, #[case] expected: u32) {
        let rule = LatePaymentRule;
        let ctx = RuleContext::default();

        let result = rule.score(&input(late), &ctx);

        assert_eq!(result.score, expected);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_late_payment_rule_fuzz(late in 0u32..=1000) {
            let rule = LatePaymentRule;
            let ctx = RuleContext::default();
            let mut input = ScoringInput::default();
            input.late_payments = late;

            let score = rule.score(&input, &ctx).score;
            let expected = if late == 0 { 200 } else { 0 };

            prop_assert_eq!(score, expected);
        }
    }
}

use crate::model::{RuleResult, ScoringInput};
use crate::rules::rules::RuleId;
use crate::rules::rules::{RuleContext, ScoringRule};

pub struct AgeRule;

impl ScoringRule for AgeRule {
    fn score(&self, value: &ScoringInput, _ctx: &RuleContext) -> RuleResult {
        let age = value.age;

        if age >= 21 {
            RuleResult {
                score: 200,
                reasons: vec!["age >= 21".to_string()],
            }
        } else {
            RuleResult {
                score: 0,
                reasons: vec!["age < 21".to_string()],
            }
        }
    }

    fn id(&self) -> RuleId {
        RuleId::Age
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn input(age: u32) -> ScoringInput {
        ScoringInput { age, income: 0.0, credit_history_months: 0, late_payments: 0, total_debt: 0.0, last_transactions: vec![], account_age_months: 0, last_6_month_late_payments: vec![] }
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(21, 200)]
    #[case(30, 200)]
    fn test_age_rule(#[case] age: u32, #[case] expected: u32) {
        let rule = AgeRule;
        let ctx = RuleContext::default();

        let result = rule.score(&input(age), &ctx);

        assert_eq!(result.score, expected);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_age_rule_fuzz(age in 0u32..=100) {
            let rule = AgeRule;
            let ctx = RuleContext::default();
            let mut input = ScoringInput::default();
            input.age = age;

            let result = rule.score(&input, &ctx).score;

            if age >= 21 {
                prop_assert_eq!(result, 200);
            } else {
                prop_assert_eq!(result, 0);
            }
        }
    }
}


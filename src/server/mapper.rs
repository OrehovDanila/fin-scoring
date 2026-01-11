use crate::server::server::pb;
use scoring_engine::model::*;

impl From<pb::ScoreRequest> for ScoringInput {
    fn from(req: pb::ScoreRequest) -> Self {
        let pb::ScoreRequest {
            age,
            income,
            credit_history_months,
            late_payments,
            total_debt,
            last_transactions,
            account_age_months,
            last_6_month_late_payments,
        } = req;

        Self {
            age,
            income,
            credit_history_months,
            late_payments,
            total_debt,
            last_transactions,
            account_age_months,
            last_6_month_late_payments,
        }
    }
}

impl From<ScoringOutput> for pb::ScoreResponse {
    fn from(output: ScoringOutput) -> Self {
        let ScoringOutput {
            score,
            decision,
            reasons,
        } = output;

        Self {
            score,
            decision,
            reasons,
        }
    }
}

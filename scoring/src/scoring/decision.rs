use crate::model::RuleResult;

pub fn decision(score: RuleResult) -> (i32, u32,  Vec<String>) {
    match score {
        s if s.score >= 700 => (0, s.score, s.reasons), // APPROVE
        s if s.score >= 500 => (1, s.score, s.reasons),
        s => (2, s.score, s.reasons),
    }
}

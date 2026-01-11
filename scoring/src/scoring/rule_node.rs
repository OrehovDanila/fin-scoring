use std::collections::{HashMap, HashSet};
use crate::rules::rules::{RuleId, ScoringRule};

struct RuleNode {
    rule: Box<dyn ScoringRule + Send + Sync>,
    deps: HashSet<RuleId>,
}

pub fn build_layers(
    rules: Vec<Box<dyn ScoringRule + Send + Sync>>,
) -> Result<Vec<Vec<Box<dyn ScoringRule + Send + Sync>>>, String> {
    let mut nodes: HashMap<RuleId, RuleNode> = rules
        .into_iter()
        .map(|rule| {
            let id = rule.id();
            let deps = rule.dependencies().iter().copied().collect();
            (
                id,
                RuleNode {
                    rule,
                    deps,
                },
            )
        })
        .collect();

    let mut layers = Vec::new();

    loop {
        let ready: Vec<RuleId> = nodes
            .iter()
            .filter(|(_, node)| node.deps.is_empty())
            .map(|(id, _)| *id)
            .collect();

        if ready.is_empty() {
            break;
        }

        let mut layer = Vec::new();
        for id in ready {
            let node = nodes.remove(&id).unwrap();
            layer.push(node.rule);
        }

        for node in nodes.values_mut() {
            for rule in &layer {
                node.deps.remove(&rule.id());
            }
        }

        layers.push(layer);
    }

    if !nodes.is_empty() {
        return Err("Cycle detected in rules DAG".into());
    }

    Ok(layers)
}

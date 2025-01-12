extern crate alloc;

use alloc::string::String;
use alloc::vec;
use hashbrown::{HashMap, HashSet};
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day23 {}

fn go(party: &HashSet<String>, idx: usize, nodes: &[(String, HashSet<String>)]) -> HashSet<String> {
    let mut max_set = party.clone();

    for (j, (node, node_set)) in nodes.iter().skip(idx).enumerate() {
        if party.contains(node) || !party.is_subset(node_set) {
            continue;
        }

        let mut new_party = party.clone();
        new_party.insert(node.clone());

        let result = go(&new_party, idx + j + 1, nodes);
        if result.len() > max_set.len() {
            max_set = result;
        }
    }
    max_set
}

#[public]
impl Day23 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result = 0;

        let data = input
            .lines()
            .map(|e| {
                let tmp = e.split("-").collect::<Vec<_>>();
                (tmp[0].to_string(), tmp[1].to_string())
            })
            .collect::<Vec<_>>();

        // Create adjacency list using HashMap
        let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();

        // Build undirected graph
        for (a, b) in &data {
            nodes
                .entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            nodes
                .entry(b.clone())
                .or_insert_with(HashSet::new)
                .insert(a.clone());
        }

        for (a, b) in &data {
            let overlap: HashSet<_> = nodes[a].intersection(&nodes[b]).cloned().collect();
            if overlap.is_empty() {
                continue;
            }

            if a.starts_with('t') || b.starts_with('t') {
                result += overlap.len();
            } else {
                result += overlap.iter().filter(|e| e.starts_with('t')).count();
            }
        }

        (result / 3).try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> String {
        let data = input
            .lines()
            .map(|e| {
                let tmp = e.split("-").collect::<Vec<_>>();
                (tmp[0].to_string(), tmp[1].to_string())
            })
            .collect::<Vec<_>>();

        // Create adjacency list using HashMap
        let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();

        // Build undirected graph
        for (a, b) in &data {
            nodes
                .entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            nodes
                .entry(b.clone())
                .or_insert_with(HashSet::new)
                .insert(a.clone());
        }

        let nodes_items: Vec<(String, HashSet<String>)> =
            nodes.into_iter().map(|(k, v)| (k, v)).collect();

        let result = go(&HashSet::new(), 0, &nodes_items);
        let mut sorted_result: Vec<String> = result.into_iter().collect();
        sorted_result.sort();

        sorted_result.join(",")
    }
}

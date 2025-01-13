extern crate alloc;

use crate::alloc::string::ToString;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day24 {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct GateInfo {
    input1: String,
    gate: Gate,
    input2: String,
}

fn evaluate(
    gate_name: &str,
    gate_info_map: &HashMap<String, GateInfo>,
    id_map: &HashMap<String, u64>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&value) = cache.get(gate_name) {
        return value;
    }

    if let Some(&value) = id_map.get(gate_name) {
        return value;
    }

    let gate_info = gate_info_map.get(gate_name).unwrap();
    let a = evaluate(&gate_info.input1, gate_info_map, id_map, cache);
    let b = evaluate(&gate_info.input2, gate_info_map, id_map, cache);

    let result = match gate_info.gate {
        Gate::AND => a & b,
        Gate::OR => a | b,
        Gate::XOR => a ^ b,
    };

    cache.insert(gate_name.to_string(), result);
    result
}

// Gate type function
fn get_gate_type(
    gate_name: &str,
    gate_info_map: &HashMap<String, GateInfo>,
    cache: &mut HashMap<String, i32>,
) -> i32 {
    if let Some(&gate_type) = cache.get(gate_name) {
        return gate_type;
    }

    let gate_info = gate_info_map.get(gate_name).expect("Gate not found");
    let result = if gate_info.gate == Gate::OR {
        5
    } else {
        let mut inputs = String::new();
        inputs.push(gate_info.input1.chars().next().unwrap());
        inputs.push(gate_info.input2.chars().next().unwrap());

        match (inputs.as_str(), &gate_info.gate) {
            ("xy" | "yx", Gate::AND) => 1,
            ("xy" | "yx", Gate::XOR) => 2,
            (_, Gate::AND) => 3,
            (_, Gate::XOR) => 4,
            _ => 0,
        }
    };

    cache.insert(gate_name.to_string(), result);
    result
}

fn validate(
    gate_name: &str,
    gate_info_map: &HashMap<String, GateInfo>,
    dependencies: &HashMap<String, Vec<String>>,
    last: &str,
    cache: &mut HashMap<String, i32>,
) -> bool {
    let gate_info = gate_info_map.get(gate_name).expect("Gate not found");
    if gate_name == last || &gate_info.input1[1..] == "00" {
        return true;
    }

    let v = get_gate_type(gate_name, gate_info_map, cache);
    let mut children = Vec::new();
    if let Some(deps) = dependencies.get(gate_name) {
        for dep in deps.iter() {
            children.push(get_gate_type(&dep, gate_info_map, cache));
        }
    }
    children.sort_unstable();

    match v {
        4 => gate_name.starts_with('z'),
        1 | 3 => children == vec![5],
        2 | 5 => children == vec![3, 4],
        _ => false,
    }
}

#[public]
impl Day24 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result = 0;

        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut ids: HashMap<String, u64> = HashMap::new();
        for line in parts[0].lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            ids.insert(
                words[0][..(words[0].len() - 1)].to_string(),
                words[1].parse().unwrap(),
            );
        }

        // Parse gates
        let mut gate_info_map: HashMap<String, GateInfo> = HashMap::new();
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for line in parts[1].lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let output = words[4].to_string();
            let gate_info = GateInfo {
                input1: words[0].to_string(),
                gate: match words[1] {
                    "AND" => Gate::AND,
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    _ => panic!("Unknown gate type"),
                },
                input2: words[2].to_string(),
            };

            // Add to dependencies
            dependencies
                .entry(gate_info.input1.clone())
                .or_default()
                .push(output.clone());
            dependencies
                .entry(gate_info.input2.clone())
                .or_default()
                .push(output.clone());

            gate_info_map.insert(output, gate_info);
        }

        let mut cache: HashMap<String, u64> = HashMap::new();

        for gate_name in gate_info_map.keys() {
            if gate_name.starts_with('z') {
                let value = evaluate(gate_name, &gate_info_map, &ids, &mut cache);
                if value != 0 {
                    if let Ok(position) = gate_name[1..].parse::<u32>() {
                        result += 2u64.pow(position);
                    }
                }
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> String {
        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut ids: HashMap<String, u64> = HashMap::new();
        for line in parts[0].lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            ids.insert(
                words[0][..(words[0].len() - 1)].to_string(),
                words[1].parse().unwrap(),
            );
        }

        // Parse gates
        let mut gate_info_map: HashMap<String, GateInfo> = HashMap::new();
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for line in parts[1].lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let output = words[4].to_string();
            let gate_info = GateInfo {
                input1: words[0].to_string(),
                gate: match words[1] {
                    "AND" => Gate::AND,
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    _ => panic!("Unknown gate type"),
                },
                input2: words[2].to_string(),
            };

            // Add to dependencies
            dependencies
                .entry(gate_info.input1.clone())
                .or_default()
                .push(output.clone());
            dependencies
                .entry(gate_info.input2.clone())
                .or_default()
                .push(output.clone());

            gate_info_map.insert(output, gate_info);
        }

        let mut gate_type_cache: HashMap<String, i32> = HashMap::new();

        let last = gate_info_map.keys().max().unwrap().clone();

        let mut invalid_gates: Vec<String> = gate_info_map
            .keys()
            .filter(|&g| {
                !validate(
                    g,
                    &gate_info_map,
                    &dependencies,
                    &last,
                    &mut gate_type_cache,
                )
            })
            .cloned()
            .collect();

        invalid_gates.sort();

        invalid_gates.join(",")
    }
}

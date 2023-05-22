use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde_json::Value;
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct DecisionRef {
    id: String,
    kind: String,
    rules: String,
    inputs: Option<Vec<String>>,
    outputs: Option<Vec<String>>,
    sources: Vec<String>,
    targets: Vec<String>
}

pub type Rule =  HashMap<String, String>;

#[derive(Clone)]
pub struct Decision {
    pub id: String,
    pub kind: String,
    pub rules: Vec<Rule>,
    pub expression: String,
    pub function: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub sources: Vec<String>,
    pub targets: Vec<String>
}

fn find_ext(uri: &str) -> String {
    let parts: Vec<&str> = uri.split(".").collect();
    let ext = parts.clone().into_iter().last().unwrap_or_default().to_string();
    ext
}

fn read_rules_csv(path: &str) -> Vec<Rule> {
    let file = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut rules: Vec<Rule> = vec![];
    for record in rdr.deserialize() {
        let rule: Rule = record.unwrap();
        rules.push(rule);
    }
    rules
}

fn read_rules_json(path: &str) -> Vec<Rule> {

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let rules: Vec<Rule> = serde_json::from_str(&data).unwrap();
    rules
}

fn slurp_function(kind: &str, path: &str) -> String {

    if kind == "function" {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data
    } else {
        "".to_string()
    }
}

fn read_rules(uri: &str) -> Vec<Rule> {

    let ext = find_ext(uri);
    let rules = match ext.as_ref() {
        "json" => read_rules_json(uri),
        "csv"  => read_rules_csv(uri),
        _ => vec![]
    };
    rules
}

fn maybe_opt(s: Option<Vec<String>>) -> Vec<String> {
    match s {
        Some(v) => v,
        None => vec![]
    }
}

pub async fn read_flow(path: PathBuf) -> Vec<Decision> {

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let refs: Vec<DecisionRef> = serde_json::from_str(&data).unwrap();

    let mut decisions: Vec<Decision> = vec![];
    for decref in refs {
        let DecisionRef { id, kind, rules, inputs, outputs, sources, targets } = decref;
        let rules_table = read_rules(&rules);
        let k = kind.clone();
        let r = rules.clone();
        let decision = Decision {
            id: id,
            kind: kind,
            rules: rules_table,
            inputs: maybe_opt(inputs),
            outputs: maybe_opt(outputs),
            sources: sources,
            expression: rules,
            function: slurp_function(&k, &r),
            targets: targets
        };
        decisions.push(decision);
    }
    decisions
}

pub async fn read_input(path: PathBuf) -> Value {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    v
}

pub async fn read_str(data: &str) -> Value {
    let v: Value = serde_json::from_str(&data).unwrap();
    v
}

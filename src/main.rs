use std::path::PathBuf;
use std::io::{self, BufRead};
use serde_json::Value;
use zen_engine::DecisionEngine;
use zen_engine::model::DecisionContent;
use clap::{Args, Parser, Subcommand};

mod graph;
mod reader;
mod evaluator;

#[derive(Debug, Parser)]
struct Orb {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Run(RunArgs),
    Eval(EvalArgs),
    Resolve(ResolveArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long, short = 'f')]
    flow: PathBuf,
    #[arg(long, short = 'i')]
    input: Option<PathBuf>,
    #[arg(long, short = 'e')]
    expression: Option<String>,
}

#[derive(Debug, Args)]
pub struct ResolveArgs {
    #[arg(long, short = 'f')]
    flow: PathBuf,
}

#[derive(Debug, Args)]
pub struct EvalArgs {
    #[arg(long, short = 'e')]
    expression: String,
    #[arg(long, short = 'd')]
    data: Option<String>,
}

async fn decide(decision_content: DecisionContent, input: Value) {

    let engine = DecisionEngine::default();
    let decision = engine.create_decision(decision_content.into());
    let result = decision.evaluate(&input).await.unwrap().result;
    println!("{}", result.to_string());
}

async fn resolve(args: ResolveArgs) {

    let ResolveArgs { flow } = args;
    let flow = reader::read_flow(flow).await;
    let decision_content = graph::build(flow).await;
    let j = serde_json::to_string_pretty(&decision_content.clone()).unwrap();
    println!("{}", j);

}

async fn run(args: RunArgs) {
    let RunArgs { flow, input, expression } = args;

    let flow = reader::read_flow(flow).await;
    let decision_content = graph::build(flow).await;

    match input {
        Some(path) => {
            let query = reader::read_input(path).await;
            decide(decision_content.clone(), query).await;
        },
        None => ()
    };

    match expression {
        Some(e) => {
            let query = reader::read_str(&e).await;
            decide(decision_content.clone(), query).await;
        },
        None => ()
    }
}

async fn eval(args: EvalArgs) {
    let EvalArgs { expression, data } = args;

    let query = match data {
        Some(d) => reader::read_str(&d).await,
        None => {
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).expect("Could not read line");
            reader::read_str(&line).await
        }
    };

    let result = evaluator::eval(&expression, query).await;
    println!("{}", result.to_string());
}


#[tokio::main]
async fn main() {

    let args = Orb::parse();

    match args.cmd {
        Cmd::Eval(args) => eval(args).await,
        Cmd::Run(args) => run(args).await,
        Cmd::Resolve(args) => resolve(args).await,
    }
}

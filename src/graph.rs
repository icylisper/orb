use zen_engine::model::{DecisionContent,
                        DecisionNode,
                        DecisionEdge,
                        DecisionNodeKind,
                        DecisionTableHitPolicy,
                        DecisionTableInputField,
                        DecisionTableOutputField,
                        DecisionTableContent,
                        Expression,
                        ExpressionNodeContent};

use crate::reader::Decision;

fn make_inputs(fields: Vec<String>) -> Vec<DecisionTableInputField> {
    let mut xs: Vec<DecisionTableInputField> = vec![];
    for f in fields {
        let field = DecisionTableInputField {
            id: f.clone(),
            name: f.clone(),
            field: f.clone()
        };
        xs.push(field);
    }
    xs
}

fn make_outputs(fields: Vec<String>) -> Vec<DecisionTableOutputField> {
    let mut xs: Vec<DecisionTableOutputField> = vec![];
    for f in fields {
        let field = DecisionTableOutputField {
            id: f.clone(),
            name: f.clone(),
            field: f.clone()
        };
        xs.push(field);
    }
    xs
}

fn make_edges(flow: Vec<Decision>) -> Vec<DecisionEdge> {
    let mut edges: Vec<DecisionEdge> = vec![];
    for decision in flow {
        let d = decision.clone();

        let sources = d.clone().sources;
        let targets = d.clone().targets;
        for source in sources {
            let e = DecisionEdge { source_id: source, target_id: d.clone().id };
            edges.push(e);
        }

        for target in targets {
            let e = DecisionEdge { source_id: d.clone().id, target_id: target };
            edges.push(e);
        }

    }
    edges
}

fn make_decision_table_node(decision: Decision) -> DecisionNode {
    let Decision { id, rules, inputs, outputs, .. } = decision;

    let input_fields = make_inputs(inputs);
    let output_fields = make_outputs(outputs);
    let content = DecisionTableContent {
        hit_policy:  DecisionTableHitPolicy::First,
        rules: rules,
        inputs: input_fields,
        outputs: output_fields
    };
    let node = DecisionNode {
        id: id.clone(),
        name: id.clone(),
        kind: DecisionNodeKind::DecisionTableNode { content: content },
    };
    node
}

fn make_expression_node(decision: Decision) -> DecisionNode {
    let Decision { id, expression, inputs, .. } = decision;

    let key = inputs.first().unwrap().to_string();
    let expression = Expression {
        id: key.clone(),
        key: key,
        value: expression.to_string()
    };

    let content = ExpressionNodeContent {
        expressions: vec![expression]
    };

    let node = DecisionNode {
        id: id.clone(),
        name: id.clone(),
        kind: DecisionNodeKind::ExpressionNode { content: content },
    };
    node
}

fn make_function_node(decision: Decision) -> DecisionNode {
    let Decision { id, function, .. } = decision;
    let node = DecisionNode {
        id: id.clone(),
        name: id.clone(),
        kind: DecisionNodeKind::FunctionNode { content: function },
    };
    node
}

fn make_nodes(flow: Vec<Decision>) -> Vec<DecisionNode> {
    let mut nodes: Vec<DecisionNode> = vec![];

    let request = DecisionNode {
        id: "request".to_string(),
        name: "request".to_string(),
        kind: DecisionNodeKind::InputNode,
    };
    nodes.push(request);

    for n in flow {
        let Decision { ref kind, .. } = n;

        let node = match kind.as_ref() {
            "table"      => make_decision_table_node(n),
            "expression" => make_expression_node(n),
            "function"   => make_function_node(n),
            _            => panic!("decision kind not given")
        };
        nodes.push(node);
    };

    let response = DecisionNode {
        id: "response".to_string(),
        name: "response".to_string(),
        kind: DecisionNodeKind::OutputNode,
    };

    nodes.push(response);
    nodes
}

pub async fn build(flow: Vec<Decision>) -> DecisionContent {
    let nodes = make_nodes(flow.clone());
    let edges = make_edges(flow.clone());
    DecisionContent {
        nodes: nodes,
        edges: edges
    }
}

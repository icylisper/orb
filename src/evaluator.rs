use serde_json::Value;
use zen_expression::isolate::Isolate;

pub async fn eval(expr: &str, data: Value) -> Value {
    let isolate = Isolate::default();
    isolate.inject_env(&data);
    let result = isolate.run_standard(expr).unwrap();
    result
}

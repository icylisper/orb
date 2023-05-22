
A tool to build decision trees using human-friendly rules based on the awesome [zen engine](https://github.com/gorules/zen)

# Design

1. `orb` helps define a flow or acyclic tree of decisions. These decisions are meant to be data-driven and managed by domain experts.
2. Each node in the decision tree is a set of rules defined using tables, functions or expressions.
3. A table can be CSV or json that captures the constraints and rules. The values in the table could be expressions.
4. A function is a javascript lambda that runs in an embedded V8 engine.
5. An expression is based on a side-effect free, dynamically-typed [zen expression language](https://gorules.io/docs/rules-engine/expression-language)

## Decision trees

The following is an example of a decision tree to calculate shipping fees based on a set of rules in [fees.csv](examples/shipping/fees.csv).
The table contains simple rules based on country, cart total and tier. The cells in the table can contain expressions. When a shipping fee is found for the given input, it is then adjusted in the next decision node using `inline expressions` and sent to a verification function.

```json
[
  {
    "id": "fees",
    "kind": "table",
    "rules": "examples/shipping/fees.csv",
    "inputs": ["cart.total", "customer.country", "customer.tier"],
    "outputs": ["fees.flat", "fees.percent"],
    "sources": ["request"],
    "targets": ["adjust", "response"]
  },

  {
    "id": "adjust",
    "kind": "expression",
    "rules": "fees.flat + 5",
    "inputs": ["fees.flat"],
    "outputs": ["fees.flat"],
    "sources": ["fees"],
    "targets": ["verify"]
  },

  {
    "id": "verify",
    "kind": "function",
    "rules": "examples/shipping/verify.js",
    "inputs": ["fees.percent"],
    "sources": ["adjust"],
    "targets": ["response"]
  }

]
```

The above example is to showcase the different kinds of rule formats and the succinctness of decision tree description.

`orb` supports 3 kinds of rule formats - `table`, `function` and `expression`


1. Table - a CSV or JSON file containing rules in a sparse form. Cells in the table can have expressions (see examples)
2. Function - A python or Javascript file that can do arbitrary data transformation and filtering
3. Expression - Inline expressions (see examples)
4. Sexp - S-expressions that can do basic data transformations

See Examples section for more concrete examples.

## Usage:

To resolve `orb decisions` into `JDM` (JSON Decision Model is a standard for defining decision trees)

```sh
orb resolve --flow examples/shipping/decisions.json
#=> jdm
```

To run the decision tree with given input

```sh

orb run --flow examples/shipping/decisions.json --input examples/shipping/input.json

```

where input.json is a query

```json
{
  "cart": {
    "total": 800
  },
  "customer": {
    "country": "US",
    "tier": "gold"
  }
}

```

We can also specify the input as follows

```sh
orb run -f examples/shipping/decisions.json -e '{"cart": {"total": 800}, "customer": {"country": "US"}}'
=> {"fees":{"flat":26},"verified":true}

```

## Testing

### Expressions

```sh

orb eval -e "max(numbers)" -d '{"numbers": [1, 2, 3]}'

echo '{"numbers": [1, 2, 3, 45]}' | orb eval -e "max(numbers)"
```


### Install

`orb` is written in Rust. Use cargo to build this project

```sh

cargo build
cp target/debug/orb orb
./orb

```

Alternatively, download a compiled native binary:

| GNU/Linux x86                                                                   | MacOSX M1/M2                                                       | MacOSX x86                                                                      |
|---------------------------------------------------------------------------------|--------------------------------------------------------------------|---------------------------------------------------------------------------------|
| [0.1.1](https://github.com/icylisper/orb/releases/download/0.1.1/orb-x86_64-linux) | [0.1.1](https://github.com/icylisper/orb/releases/download/0.1.1/orb-mac) | [0.4.2](https://github.com/icylisper/orb/releases/download/0.1.1/orb-x86_64-apple) |

### Credits

Thanks to [GoRules team](https://github.com/gorules/zen) for the awesome zen engine.


Copyright 2023 icylisper

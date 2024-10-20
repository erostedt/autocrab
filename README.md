# About
autocrab is a rust pun on autograd (automatic differentiation). autocrab only supports forward mode autodifferentation

# Install
* Build using `cargo build`
* Run tests using `cargo test`
* Run examples using `cargo run --example <example>`

# How to use
See the examples and tests.
But generally the way to use autocrab is to define an "objective function", e.g.:
```
fn objective_function(variables: [Variable; 2]) -> Variable
{
    square(variables[0]) * variables[1] + sin(variables[1]) / variables[0]
}
```
Then use the `evaluate` function which has the signature:
```
pub fn evaluate<const VARIABLE_COUNT: usize>(function: ObjectiveFunction<VARIABLE_COUNT>, x: [f64; VARIABLE_COUNT]) -> (f64, [f64; VARIABLE_COUNT])
```
to evaluate the objective function and its gradient at a point `x`.

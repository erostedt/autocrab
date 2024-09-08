use super::variable::*;

fn make_variables_from_values<const VARIABLE_COUNT: usize>(x: [f64; VARIABLE_COUNT], seed_index: usize) -> [Variable; VARIABLE_COUNT]
{
    assert!(seed_index < VARIABLE_COUNT);
    let mut variables = [Variable::empty(); VARIABLE_COUNT];
    for (variable, value) in variables.iter_mut().zip(x.into_iter())
    {
        variable.value = value;
    }
    variables[seed_index].derivative = 1.0;
    variables
}

type ObjectiveFunction<const VARIABLE_COUNT: usize>  = fn(variables: [Variable; VARIABLE_COUNT]) -> Variable;
pub fn evaluate<const VARIABLE_COUNT: usize>(function: ObjectiveFunction<VARIABLE_COUNT>, x: [f64; VARIABLE_COUNT]) -> (f64, [f64; VARIABLE_COUNT])
{
    let mut value = 0.0;
    let mut gradient = [0.0; VARIABLE_COUNT];
    for (index, derivative) in gradient.iter_mut().enumerate()
    {
        let variables = make_variables_from_values(x, index);
        let res = function(variables);
        value = res.value;
        *derivative = res.derivative;
    }
    (value, gradient)
}

pub fn gradient_descent_step<const VARIABLE_COUNT: usize>(function: ObjectiveFunction<VARIABLE_COUNT>, x: [f64; VARIABLE_COUNT], step_size: f64) -> [f64; VARIABLE_COUNT]
{
    let (_, gradient) = evaluate(function, x);
    let mut next_position = [0.0; VARIABLE_COUNT];
    for (i, xi) in x.iter().zip(gradient).map(|(xi, gi)| -> f64 { xi - step_size * gi }).enumerate()
    {
        next_position[i] = xi;
    }
    next_position
}

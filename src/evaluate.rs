use super::variable::*;

fn make_variables_from_values<const VARIABLE_COUNT: usize>(
    x: [f64; VARIABLE_COUNT],
    seed_index: usize,
) -> [Variable; VARIABLE_COUNT]
{
    assert!(seed_index < VARIABLE_COUNT);
    let mut variables = [Variable::empty(); VARIABLE_COUNT];
    for (variable, value) in variables.iter_mut().zip(x.into_iter()) {
        variable.value = value;
    }
    variables[seed_index].derivative = 1.0;
    variables
}

pub type ObjectiveFunction<const VARIABLE_COUNT: usize, const OUTPUT_COUNT: usize> =
    fn(variables: [Variable; VARIABLE_COUNT]) -> [Variable; OUTPUT_COUNT];

pub fn evaluate<const VARIABLE_COUNT: usize, const OUTPUT_COUNT: usize>(
    function: ObjectiveFunction<VARIABLE_COUNT, OUTPUT_COUNT>,
    x: [f64; VARIABLE_COUNT],
) -> ([f64; OUTPUT_COUNT], [[f64; VARIABLE_COUNT]; OUTPUT_COUNT])
{
    let mut values = [0.0; OUTPUT_COUNT];
    let mut jacobian = [[0.0; VARIABLE_COUNT]; OUTPUT_COUNT];

    for (output_index, value) in values.iter_mut().enumerate() {
        for (input_index, derivative) in jacobian[output_index].iter_mut().enumerate() {
            let variables = make_variables_from_values(x, input_index);
            let res = function(variables);
            *value = res[output_index].value;
            *derivative = res[output_index].derivative;
        }
    }

    (values, jacobian)
}

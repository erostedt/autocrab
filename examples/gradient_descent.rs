use autocrab::evaluate::evaluate;
use autocrab::evaluate::ObjectiveFunction;
use autocrab::functions::*;
use autocrab::variable::Variable;

fn objective_function(variables: [Variable; 1]) -> [Variable; 1]
{
    [square(variables[0]) + 2.0 * variables[0] + 1.0]
}

pub fn gradient_descent_step<const VARIABLE_COUNT: usize>(
    function: ObjectiveFunction<VARIABLE_COUNT, 1>,
    x: [f64; VARIABLE_COUNT],
    step_size: f64,
) -> [f64; VARIABLE_COUNT]
{
    let (_, jacobian) = evaluate(function, x);
    let mut next_position = [0.0; VARIABLE_COUNT];
    for (i, xi) in x
        .iter()
        .zip(jacobian[0])
        .map(|(xi, gi)| -> f64 { xi - step_size * gi })
        .enumerate()
    {
        next_position[i] = xi;
    }
    next_position
}

fn main()
{
    let mut x = [5.0];
    for _ in 0..30 {
        x = gradient_descent_step(objective_function, x, 0.3);
    }

    println!("Predicted minimum: {:?}", x[0]);
    println!("Actual minimum: {:?}", -1.0);
}

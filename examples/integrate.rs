use autocrab::evaluate::evaluate;
use autocrab::evaluate::ObjectiveFunction;
use autocrab::functions::*;
use autocrab::variable::Variable;

fn objective_function(variables: [Variable; 1]) -> [Variable; 1]
{
    [square(variables[0])]
}

fn linspace(start: f64, end: f64, steps: usize) -> Vec<f64>
{
    assert!(start <= end);
    assert!(steps > 0);
    let delta = (end - start) / steps as f64;
    (0..steps).map(|i| start + (i as f64) * delta).collect()
}

fn integrate_trapizoidal(
    function: ObjectiveFunction<1, 1>,
    start: f64,
    end: f64,
    steps: usize,
) -> f64
{
    let delta = (end - start) / steps as f64;
    let range = linspace(start, end, steps);
    let mut integrand = 0.0;
    let (value, jacobian) = evaluate(function, [start]);
    let mut previous_value = value[0];
    let mut previous_derivative = jacobian[0][0];

    for x in range.into_iter().skip(1) {
        let (current_values, current_jacobian) = evaluate(function, [x]);
        let average_value = (previous_value + current_values[0]) / 2.0;

        let slope_adjustment = delta * (previous_derivative + current_jacobian[0][0]) / 2.0;
        integrand += (average_value + slope_adjustment) * delta;

        previous_value = current_values[0];
        previous_derivative = current_jacobian[0][0];
    }
    integrand
}

fn integrate_euler(function: ObjectiveFunction<1, 1>, start: f64, end: f64, steps: usize) -> f64
{
    let delta = (end - start) / steps as f64;
    let range = linspace(start, end, steps);
    let mut integrand = 0.0;

    for x in range.into_iter() {
        let (values, _) = evaluate(function, [x]);
        integrand += delta * values[0];
    }
    integrand
}

fn integrate_euler_with_gradients(
    function: ObjectiveFunction<1, 1>,
    start: f64,
    end: f64,
    steps: usize,
) -> f64
{
    let delta = (end - start) / steps as f64;
    let range = linspace(start, end, steps);
    let mut integrand = 0.0;

    for x in range.into_iter() {
        let (values, jacobian) = evaluate(function, [x]);
        integrand += delta * values[0] + 0.5 * delta * delta * jacobian[0][0];
    }
    integrand
}

fn main()
{
    let start = 0.0;
    let end = 5.0;
    let steps = 100;
    let ground_truth = 125.0 / 3.0;
    let integrand_trapizoidal = integrate_trapizoidal(objective_function, start, end, steps);
    let integrand_euler = integrate_euler(objective_function, start, end, steps);
    let integrand_euler_gradient =
        integrate_euler_with_gradients(objective_function, start, end, steps);

    println!("Integrand of x^2 from 0 to 5 = {}", ground_truth);
    println!(
        "Trapizoidal estimation using {} steps = {}",
        steps, integrand_trapizoidal
    );

    println!(
        "Euler estimation using {} steps = {}",
        steps, integrand_euler
    );
    println!(
        "Euler with gradient estimation using {} steps = {}",
        steps, integrand_euler_gradient
    );
}

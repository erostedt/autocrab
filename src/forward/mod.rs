pub mod evaluate;
pub mod functions;
pub mod variable;

pub fn almost_eq(left: f64, right: f64) -> bool
{
    let tolerance = 1.0e-8;
    (left - right).abs() < tolerance
}

pub fn almost_equals<const VARIABLE_COUNT: usize>(
    lhs: [f64; VARIABLE_COUNT],
    rhs: [f64; VARIABLE_COUNT],
) -> bool
{
    for (left, right) in lhs.into_iter().zip(rhs.into_iter()) {
        if !almost_eq(left, right) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test_variable
{
    use super::variable::*;
    use super::*;

    #[test]
    fn test_add_variables()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = Variable::with_derivative(2.0, 3.0);
        let z = x + y;
        assert!(almost_eq(z.value, 3.0));
        assert!(almost_eq(z.derivative, 5.0));
    }

    #[test]
    fn test_add_scalar()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = x + 2.0;
        assert!(almost_eq(y.value, 3.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_add_scalar_rev()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = 2.0 + x;
        assert!(almost_eq(y.value, 3.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_sub_variables()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = Variable::with_derivative(2.0, 3.0);
        let z = x - y;
        assert!(almost_eq(z.value, -1.0));
        assert!(almost_eq(z.derivative, -1.0));
    }

    #[test]
    fn test_sub_scalar()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = x - 2.0;
        assert!(almost_eq(y.value, -1.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_sub_scalar_rev()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = 2.0 - x;
        assert!(almost_eq(y.value, 1.0));
        assert!(almost_eq(y.derivative, -2.0));
    }

    #[test]
    fn test_multiply_variables()
    {
        let x = Variable::with_derivative(2.0, 4.0);
        let y = Variable::with_derivative(3.0, 5.0);
        let z = x * y;
        assert!(almost_eq(z.value, 6.0));
        assert!(almost_eq(z.derivative, 22.0));
    }

    #[test]
    fn test_multiply_scalar()
    {
        let x = Variable::with_derivative(2.0, 3.0);
        let y = x * 2.0;
        assert!(almost_eq(y.value, 4.0));
        assert!(almost_eq(y.derivative, 6.0));
    }

    #[test]
    fn test_multiply_scalar_rev()
    {
        let x = Variable::with_derivative(2.0, 3.0);
        let y = 2.0 * x;
        assert!(almost_eq(y.value, 4.0));
        assert!(almost_eq(y.derivative, 6.0));
    }

    #[test]
    fn test_div_variables()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = Variable::with_derivative(2.0, 3.0);
        let z = x / y;
        assert!(almost_eq(z.value, 0.5));
        assert!(almost_eq(z.derivative, 0.25));
    }

    #[test]
    fn test_div_scalar()
    {
        let x = Variable::with_derivative(1.0, 2.0);
        let y = x / 2.0;
        assert!(almost_eq(y.value, 0.5));
        assert!(almost_eq(y.derivative, 1.0));
    }

    #[test]
    fn test_div_scalar_rev()
    {
        let x = Variable::with_derivative(4.0, 2.0);
        let y = 2.0 / x;
        println!("Variable: {:?}", y);
        assert!(almost_eq(y.value, 0.5));
        assert!(almost_eq(y.derivative, -0.125));
    }
}

#[cfg(test)]
mod test_elementary_functions
{
    use super::functions::*;
    use super::variable::*;
    use super::*;

    #[test]
    fn test_sinus_90_deg()
    {
        let angle = std::f64::consts::PI / 2.0;
        let x = Variable::seeded(angle);
        let fx = sin(x);
        assert!(almost_eq(fx.value, 1.0));
        assert!(almost_eq(fx.derivative, 0.0));
    }

    #[test]
    fn test_sinus_30_deg()
    {
        let angle = std::f64::consts::PI / 6.0;
        let x = Variable::seeded(angle);
        let fx = sin(x);
        assert!(almost_eq(fx.value, 0.5));
        assert!(almost_eq(fx.derivative, f64::sqrt(3.0) / 2.0));
    }

    #[test]
    fn test_cosinus_90_deg()
    {
        let angle = std::f64::consts::PI / 2.0;
        let x = Variable::seeded(angle);
        let fx = cos(x);
        assert!(almost_eq(fx.value, 0.0));
        assert!(almost_eq(fx.derivative, -1.0));
    }

    #[test]
    fn test_cosinus_30_deg()
    {
        let angle = std::f64::consts::PI / 6.0;
        let x = Variable::seeded(angle);
        let fx = cos(x);
        assert!(almost_eq(fx.value, f64::sqrt(3.0) / 2.0));
        assert!(almost_eq(fx.derivative, -0.5));
    }

    #[test]
    fn test_tan_45_deg()
    {
        let angle = std::f64::consts::PI / 4.0;
        let x = Variable::seeded(angle);
        let fx = tan(x);
        assert!(almost_eq(fx.value, 1.0));
        assert!(almost_eq(fx.derivative, 2.0));
    }

    #[test]
    fn test_ln_2()
    {
        let x = Variable::seeded(2.0);
        let fx = ln(x);
        assert!(almost_eq(fx.value, std::f64::consts::LN_2));
        assert!(almost_eq(fx.derivative, 1.0 / 2.0));
    }

    #[test]
    fn test_square()
    {
        let x = Variable::seeded(3.0);
        let fx = square(x);
        assert!(almost_eq(fx.value, 9.0));
        assert!(almost_eq(fx.derivative, 6.0));
    }

    #[test]
    fn test_pow()
    {
        let x = Variable::seeded(2.0);
        let fx = pow(x, 3.0);
        assert!(almost_eq(fx.value, 8.0));
        assert!(almost_eq(fx.derivative, 12.0));
    }

    #[test]
    fn test_sqrt()
    {
        let x = Variable::seeded(2.0);
        let fx = sqrt(x);
        let target_value = f64::sqrt(2.0);
        let target_derivative = 0.5 / f64::sqrt(2.0);
        assert!(almost_eq(fx.value, target_value));
        assert!(almost_eq(fx.derivative, target_derivative));
    }

    #[test]
    fn test_hard_expression()
    {
        let x = Variable::seeded(2.0);
        let fx = tan(ln(x) + sin(x)) + x * cos(x);
        let target_value = -32.4190367069393;
        let target_derivative = 81.5112855513418;
        assert!(almost_eq(fx.value, target_value));
        assert!(almost_eq(fx.derivative, target_derivative));
    }
}

#[cfg(test)]
mod test_polynomials
{
    use super::variable::*;
    use super::*;

    #[test]
    fn test_linear_positive()
    {
        let x = Variable::seeded(2.0);
        let fx = 3.0 * x + 5.0;
        assert!(almost_eq(fx.value, 11.0));
        assert!(almost_eq(fx.derivative, 3.0));
    }

    #[test]
    fn test_linear_negative()
    {
        let x = Variable::seeded(2.0);
        let fx = -3.0 * x + 5.0;
        assert!(almost_eq(fx.value, -1.0));
        assert!(almost_eq(fx.derivative, -3.0));
    }

    #[test]
    fn test_quadratic()
    {
        let x = Variable::seeded(2.0);
        let fx = 2.0 * x.pow(2.0) + 3.0 * x + 4.0;
        assert!(almost_eq(fx.value, 18.0));
        assert!(almost_eq(fx.derivative, 11.0));
    }
}

#[cfg(test)]
mod test_evaluate
{
    use super::evaluate::*;
    use super::functions::*;
    use super::variable::*;
    use super::*;

    #[test]
    fn test_evaluate_multi_linear()
    {
        fn objective_function(variables: [Variable; 3]) -> Variable
        {
            variables[0] + 2.0 * variables[1] + 3.0 * variables[2]
        }

        let (fx, gradient) = evaluate(objective_function, [1.0, 2.0, 3.0]);
        println!("{:?}", fx);

        assert!(almost_eq(fx, 14.0));
        assert!(almost_equals(gradient, [1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_gradient_descent()
    {
        fn objective_function(variables: [Variable; 1]) -> Variable
        {
            square(variables[0]) + 2.0 * variables[0] + 1.0
        }

        let mut x = [0.0];
        for _ in 0..30 {
            x = gradient_descent_step(objective_function, x, 0.5);
        }
        println!("{:?}", x[0]);

        assert!(almost_eq(x[0], -1.0));
    }
}

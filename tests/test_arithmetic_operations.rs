extern crate autocrab;

use autocrab::variable::*;
use autocrab::compare::*;

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_add_variables()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = Variable::with_initial_derivative(2.0, 3.0);
        let z = x + y;
        assert!(almost_eq(z.value, 3.0));
        assert!(almost_eq(z.derivative, 5.0));
    }

    #[test]
    fn test_add_scalar()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = x + 2.0;
        assert!(almost_eq(y.value, 3.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_add_scalar_rev()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = 2.0 + x;
        assert!(almost_eq(y.value, 3.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_sub_variables()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = Variable::with_initial_derivative(2.0, 3.0);
        let z = x - y;
        assert!(almost_eq(z.value, -1.0));
        assert!(almost_eq(z.derivative, -1.0));
    }

    #[test]
    fn test_sub_scalar()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = x - 2.0;
        assert!(almost_eq(y.value, -1.0));
        assert!(almost_eq(y.derivative, 2.0));
    }

    #[test]
    fn test_sub_scalar_rev()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = 2.0 - x;
        assert!(almost_eq(y.value, 1.0));
        assert!(almost_eq(y.derivative, -2.0));
    }

    #[test]
    fn test_multiply_variables()
    {
        let x = Variable::with_initial_derivative(2.0, 4.0);
        let y = Variable::with_initial_derivative(3.0, 5.0);
        let z = x * y;
        assert!(almost_eq(z.value, 6.0));
        assert!(almost_eq(z.derivative, 22.0));
    }

    #[test]
    fn test_multiply_scalar()
    {
        let x = Variable::with_initial_derivative(2.0, 3.0);
        let y = x * 2.0;
        assert!(almost_eq(y.value, 4.0));
        assert!(almost_eq(y.derivative, 6.0));
    }

    #[test]
    fn test_multiply_scalar_rev()
    {
        let x = Variable::with_initial_derivative(2.0, 3.0);
        let y = 2.0 * x;
        assert!(almost_eq(y.value, 4.0));
        assert!(almost_eq(y.derivative, 6.0));
    }

    #[test]
    fn test_div_variables()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = Variable::with_initial_derivative(2.0, 3.0);
        let z = x / y;
        assert!(almost_eq(z.value, 0.5));
        assert!(almost_eq(z.derivative, 0.25));
    }

    #[test]
    fn test_div_scalar()
    {
        let x = Variable::with_initial_derivative(1.0, 2.0);
        let y = x / 2.0;
        assert!(almost_eq(y.value, 0.5));
        assert!(almost_eq(y.derivative, 1.0));
    }

    #[test]
    fn test_div_scalar_rev()
    {
        let x = Variable::with_initial_derivative(4.0, 2.0);
        let y = 2.0 / x;
        println!("Variable: {:?}", y);
        assert!(almost_eq(y.value, 0.5));
        assert!(almost_eq(y.derivative, -0.125));
    }

}

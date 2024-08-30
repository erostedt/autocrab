extern crate autocrab;

use autocrab::variable::*;
use autocrab::compare::*;
use autocrab::functions::*;


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_sinus_90_deg()
    {
        let angle = std::f64::consts::PI / 2.0;
        let x = Variable::new(angle);
        let fx = sin(x);
        assert!(almost_eq(fx.value, 1.0));
        assert!(almost_eq(fx.derivative, 0.0));
    }

    #[test]
    fn test_sinus_30_deg()
    {
        let angle = std::f64::consts::PI / 6.0;
        let x = Variable::new(angle);
        let fx = sin(x);
        assert!(almost_eq(fx.value, 0.5));
        assert!(almost_eq(fx.derivative, f64::sqrt(3.0) / 2.0));
    }

    #[test]
    fn test_cosinus_90_deg()
    {
        let angle = std::f64::consts::PI / 2.0;
        let x = Variable::new(angle);
        let fx = cos(x);
        assert!(almost_eq(fx.value, 0.0));
        assert!(almost_eq(fx.derivative, -1.0));
    }

    #[test]
    fn test_cosinus_30_deg()
    {
        let angle = std::f64::consts::PI / 6.0;
        let x = Variable::new(angle);
        let fx = cos(x);
        assert!(almost_eq(fx.value, f64::sqrt(3.0) / 2.0));
        assert!(almost_eq(fx.derivative, -0.5));
    }

    #[test]
    fn test_tan_45_deg()
    {
        let angle = std::f64::consts::PI / 4.0;
        let x = Variable::new(angle);
        let fx = tan(x);
        assert!(almost_eq(fx.value, 1.0));
        assert!(almost_eq(fx.derivative, 2.0));
    }

    #[test]
    fn test_ln_2()
    {
        let x = Variable::new(2.0);
        let fx = ln(x);
        assert!(almost_eq(fx.value, std::f64::consts::LN_2));
        assert!(almost_eq(fx.derivative, 1.0/2.0));
    }

    #[test]
    fn test_square()
    {
        let x = Variable::new(3.0);
        let fx = square(x);
        assert!(almost_eq(fx.value, 9.0));
        assert!(almost_eq(fx.derivative, 6.0));
    }

    #[test]
    fn test_pow()
    {
        let x = Variable::new(2.0);
        let fx = pow(x, 3.0);
        assert!(almost_eq(fx.value, 8.0));
        assert!(almost_eq(fx.derivative, 12.0));
    }

    #[test]
    fn test_sqrt()
    {
        let x = Variable::new(2.0);
        let fx = sqrt(x);
        let target_value = f64::sqrt(2.0);
        let target_derivative = 0.5/f64::sqrt(2.0);
        assert!(almost_eq(fx.value, target_value));
        assert!(almost_eq(fx.derivative, target_derivative));
    }

    #[test]
    fn test_hard_expression()
    {
        let x = Variable::new(2.0);
        let fx = tan(ln(x) + sin(x)) + x * cos(x);
        let target_value = -32.4190367069393;
        let target_derivative = 81.5112855513418;
        assert!(almost_eq(fx.value, target_value));
        assert!(almost_eq(fx.derivative, target_derivative));
    }

}

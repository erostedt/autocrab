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

}

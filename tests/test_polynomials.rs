extern crate autocrab;

use autocrab::variable::*;
use autocrab::compare::*;


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_linear_positive()
    {
        let x = Variable::new(2.0);
        let fx = 3.0 * x + 5.0;
        assert!(almost_eq(fx.value, 11.0));
        assert!(almost_eq(fx.derivative, 3.0));
    }

    #[test]
    fn test_linear_negative()
    {
        let x = Variable::new(2.0);
        let fx = -3.0 * x + 5.0;
        assert!(almost_eq(fx.value, -1.0));
        assert!(almost_eq(fx.derivative, -3.0));
    }

    #[test]
    fn test_quadratic()
    {
        let x = Variable::new(2.0);
        let fx = 2.0 * x.pow(2.0) + 3.0 * x + 4.0;
        assert!(almost_eq(fx.value, 18.0));
        assert!(almost_eq(fx.derivative, 11.0));
    }

}

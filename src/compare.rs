pub fn almost_eq(left: f64, right: f64) -> bool
{
    let tolerance = 1.0e-8;
    (left - right).abs() < tolerance
}

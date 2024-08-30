use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Variable
{
    pub value: f64,
    pub derivative: f64,
}

impl Variable
{
    pub fn new(value: f64) -> Self
    {
        Self { value, derivative: 1.0 }
    }

    pub fn with_initial_derivative(value: f64, derivative: f64) -> Self
    {
        Self { value, derivative }
    }

    pub fn pow(&self, exponent: f64) -> Self
    {
        Self { value: self.value.powf(exponent), derivative:  exponent* self.derivative * self.value.powf(exponent - 1.0) }
    }
}

impl Add for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable 
    {
        Variable { value: self.value + other.value, derivative: self.derivative + other.derivative }
    }
}

impl Add<f64> for Variable 
{
    type Output = Variable;

    fn add(self, scalar: f64) -> Variable
    {
        Variable { value: self.value + scalar, derivative: self.derivative }
    }
}

impl Add<Variable> for f64 {
    type Output = Variable;

    fn add(self, variable: Variable) -> Self::Output 
    {
        Variable { value: self + variable.value, derivative: variable.derivative }
    }
}

impl Sub for Variable {
    type Output = Variable;

    fn sub(self, other: Variable) -> Variable 
    {
        Variable { value: self.value - other.value, derivative: self.derivative - other.derivative }
    }
}

impl Sub<f64> for Variable 
{
    type Output = Variable;

    fn sub(self, scalar: f64) -> Variable
    {
        Variable { value: self.value - scalar, derivative: self.derivative }
    }
}

impl Sub<Variable> for f64 {
    type Output = Variable;

    fn sub(self, variable: Variable) -> Self::Output 
    {
        Variable { value: self - variable.value, derivative: -variable.derivative }
    }
}

impl Mul for Variable {
    type Output = Variable;

    fn mul(self, other: Variable) -> Variable 
    {
        Variable { value: self.value * other.value, derivative: self.derivative * other.value + self.value * other.derivative }
    }
}


impl Mul<f64> for Variable 
{
    type Output = Variable;

    fn mul(self, scalar: f64) -> Variable
    {
        Variable { value: self.value * scalar, derivative: self.derivative * scalar }
    }
}

impl Mul<Variable> for f64 {
    type Output = Variable;

    fn mul(self, variable: Variable) -> Self::Output 
    {
        Variable { value: self * variable.value, derivative: self * variable.derivative }
    }
}

impl Div for Variable {
    type Output = Variable;

    fn div(self, other: Variable) -> Variable 
    {
        Variable { value: self.value / other.value, derivative:  (self.derivative * other.value - self.value * other.derivative) / (other.value * other.value) }
    }
}

impl Div<f64> for Variable 
{
    type Output = Variable;

    fn div(self, scalar: f64) -> Variable
    {
        Variable { value: self.value / scalar, derivative: self.derivative / scalar }
    }
}

impl Div<Variable> for f64 {
    type Output = Variable;

    fn div(self, variable: Variable) -> Self::Output 
    {
        Variable { value: self / variable.value, derivative: -self / (variable.value * variable.value) }
    }
}

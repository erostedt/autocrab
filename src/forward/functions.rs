use crate::forward::variable::Variable;

pub fn sin(variable: Variable) -> Variable
{
    Variable {
        value: variable.value.sin(),
        derivative: variable.derivative * variable.value.cos(),
    }
}

pub fn cos(variable: Variable) -> Variable
{
    Variable {
        value: variable.value.cos(),
        derivative: -variable.derivative * variable.value.sin(),
    }
}

pub fn tan(variable: Variable) -> Variable
{
    let sec = 1.0 / variable.value.cos();
    Variable {
        value: variable.value.tan(),
        derivative: variable.derivative * sec * sec,
    }
}

pub fn ln(variable: Variable) -> Variable
{
    Variable {
        value: variable.value.ln(),
        derivative: variable.derivative / variable.value,
    }
}

pub fn square(variable: Variable) -> Variable
{
    variable * variable
}

pub fn pow(variable: Variable, exponent: f64) -> Variable
{
    variable.pow(exponent)
}

pub fn sqrt(variable: Variable) -> Variable
{
    let root = variable.value.sqrt();
    Variable {
        value: root,
        derivative: variable.derivative / (2.0 * root),
    }
}

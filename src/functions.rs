use crate::variable::Variable;

pub fn sin(variable: Variable) -> Variable
{
    Variable { value: variable.value.sin(), derivative: variable.derivative * variable.value.cos() }
}

pub fn cos(variable: Variable) -> Variable
{
    Variable { value: variable.value.cos(), derivative: -variable.derivative * variable.value.sin() }
}

pub fn tan(variable: Variable) -> Variable
{
    let sec = 1.0 / variable.value.cos();
    Variable { value: variable.value.tan(), derivative: variable.derivative * sec * sec }
}

pub fn ln(variable: Variable) -> Variable
{
    Variable { value: variable.value.ln(), derivative: variable.derivative / variable.value }
}

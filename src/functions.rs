use crate::variable::Variable;

pub fn sin(variable: Variable) -> Variable
{
    Variable { value: variable.value.sin(), derivative: variable.derivative * variable.value.cos() }
}

pub fn cos(variable: Variable) -> Variable
{
    Variable { value: variable.value.cos(), derivative: -variable.derivative * variable.value.sin() }
}

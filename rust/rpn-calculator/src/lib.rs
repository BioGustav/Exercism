#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack: Vec<i32> = vec![];
    for input in inputs {
        if let Value(v) = input {
            stack.push(v.clone());
            continue;
        }
        let rhs = stack.pop()?;
        let lhs = stack.pop()?;
        let result = match input {
            Add => lhs + rhs,
            Subtract => lhs - rhs,
            Multiply => lhs * rhs,
            Divide => lhs / rhs,
            _ => panic!("Invalid state!"),
        };
        stack.push(result);
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

use std::ops::Add;

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
    for i in inputs {
        match (i, stack.len()) {
            (Value(value), _) => stack.push(*value),
            (command, len) if len >= 2 => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    if let Some(result) = match command {
                        Add => Some(lhs + rhs),
                        Subtract => Some(lhs - rhs),
                        Multiply => Some(lhs * rhs),
                        Divide => Some(lhs / rhs),
                        _ => None,
                    } {
                        stack.push(result);
                    };
                }
            }
            (_, _) => return None,
        };
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

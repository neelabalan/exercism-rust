#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn perform_op(stack: &mut Vec::<i32>, input: &CalculatorInput) {
    let (a, b) = (stack.pop(), stack.pop());
    if None != a && None != b {
        let (x, y) = (a.unwrap(), b.unwrap());
        match input {
            CalculatorInput::Add =>  stack.push(x + y),
            CalculatorInput::Subtract=> stack.push(y - x),
            CalculatorInput::Multiply=> stack.push(x * y),
            CalculatorInput::Divide=> stack.push(y / x),
            _ => {},
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack  = Vec::<i32>::new();
    for input in inputs {
        if let CalculatorInput::Value(val) = input {
            stack.push(val.clone())
        }
        else {
            if stack.len() == 0 {
                break
            }
            perform_op(&mut stack, input)
        }
    }
    return if stack.len() == 1 {stack.pop()} else {None};
}

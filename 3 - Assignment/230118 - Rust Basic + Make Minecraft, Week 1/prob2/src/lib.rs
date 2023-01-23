#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for x in 0..inputs.len() {
      match &inputs[x] {
        CalculatorInput::Add => {
          if stack.len() < 2 {
            return None
          }

          let a = stack.pop().unwrap();
          let b = stack.pop().unwrap();
          stack.push(a + b)
        }
        CalculatorInput::Subtract => {
          if stack.len() < 2 {
            return None
          }

          let a = stack.pop().unwrap();
          let b = stack.pop().unwrap();
          stack.push(a - b)
        }
        CalculatorInput::Multiply => {
          if stack.len() < 2 {
            return None
          }

          let a = stack.pop().unwrap();
          let b = stack.pop().unwrap();
          stack.push(a * b)
        }
        CalculatorInput::Divide => {
          if stack.len() < 2 {
            return None
          }

          let a = stack.pop().unwrap();
          let b = stack.pop().unwrap();
          stack.push(a / b)
        }
        CalculatorInput::Value(value) => {
          stack.push(*value)
        }
      }
    }

    if stack.len() == 1 {
      return stack.pop()
    }

    None
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}

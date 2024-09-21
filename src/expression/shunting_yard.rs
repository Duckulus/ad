const RADIX: u32 = 10;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Operator(char),
    OpenParen,
}

fn get_precedence(operator: char) -> i32 {
    match operator {
        '-' => 1,
        '+' => 2,
        '*' => 3,
        '/' => 4,
        '^' => 5,
        _ => { panic!("Unknown Operator {}", operator) }
    }
}

fn convert_to_rpn(input: &str) -> Vec<Token> {
    let mut holding_stack = Vec::new();
    let mut output_stack = Vec::new();

    let mut number = 0;
    let mut is_number = false;

    for char in input.chars() {
        if char.is_digit(RADIX) {
            number = number * RADIX + char.to_digit(RADIX).expect("char to be valid digit");
            is_number = true;
            continue;
        } else if is_number {
            output_stack.push(Token::Number(number as i32));
            number = 0;
            is_number = false;
        }

        if char.is_whitespace() {
            continue;
        }
        if char == '(' {
            holding_stack.push(Token::OpenParen);
        } else if char == ')' {
            while !holding_stack.is_empty() && !matches!(holding_stack.last(), Some(Token::OpenParen)) {
                output_stack.push(holding_stack.pop().unwrap())
            }
            holding_stack.pop();
        } else {
            'inner: while !holding_stack.is_empty() && matches!(holding_stack.last(), Some(Token::Operator(_))) {
                if let Some(Token::Operator(c)) = holding_stack.iter().last() {
                    if get_precedence(char) > get_precedence(*c) {
                        break 'inner;
                    }
                    output_stack.push(holding_stack.pop().unwrap());
                } else {
                    panic!("unreachable");
                }
            }
            holding_stack.push(Token::Operator(char));
        }
    }

    if is_number {
        output_stack.push(Token::Number(number as i32))
    }
    while !holding_stack.is_empty() {
        output_stack.push(holding_stack.pop().unwrap())
    }

    output_stack
}

/// Evaluates an arbitrary expression in string form with respect to PEMDAS
/// Supports the +,-,*,/,^ operators and parentheses
pub fn shunting_yard(input: &str) -> i32 {
    let tokens = convert_to_rpn(input);
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => {
                stack.push(num);
            }
            Token::Operator(operator) => {
                let right_operand = stack.pop().expect("right operand to be on stack");
                let left_operand = stack.pop().expect("left operand to be on stack");
                stack.push(match operator {
                    '-' => left_operand - right_operand,
                    '+' => left_operand + right_operand,
                    '*' => left_operand * right_operand,
                    '/' => left_operand / right_operand,
                    '^' => left_operand.pow(right_operand as u32),
                    _ => panic!("Unknown Operator {}", operator)
                });
            }
            _ => panic!("Unexpected Token")
        }
    }

    stack.pop().expect("a single value on the stack")
}

#[test]
pub fn convert_to_rpn_test() {
    assert_eq!(convert_to_rpn("1+1"), vec![Token::Number(1), Token::Number(1), Token::Operator('+')]);
    assert_eq!(convert_to_rpn("(1+1)*(3+6)*0"), vec![
        Token::Number(1),
        Token::Number(1),
        Token::Operator('+'),
        Token::Number(3),
        Token::Number(6),
        Token::Operator('+'),
        Token::Operator('*'),
        Token::Number(0),
        Token::Operator('*'),
    ]);
    assert_eq!(convert_to_rpn("1234+5678"), vec![Token::Number(1234), Token::Number(5678), Token::Operator('+')])
}

#[test]
pub fn shunting_yard_test() {
    assert_eq!(shunting_yard("1+1"), 2);
    assert_eq!(shunting_yard("2 + 4 * 3"), 14);
    assert_eq!(shunting_yard("(2 + 4) * 3"), 18);
    assert_eq!(shunting_yard("1+2^5"), 33);
}

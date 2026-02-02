use std::{io};
use crate::stack::{Operator, Stack};

mod stack;

fn main() {
    let expression = get_expression();
    let stack = &mut Stack::new();
    
    eval_expressions(expression, stack);
}

fn get_expression() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("To get expression");

    let expressions: Vec<String> = buffer
        .split_whitespace()
        .map(|e| e.to_string())
        .collect();
    
    expressions
}

fn eval_expressions(expressions: Vec<String>, stack: &mut Stack) {
    let mut output: Vec<String> = Vec::new();
    
    for e in expressions {
        match e.as_str() {
            "+" => output.append(&mut eval_operator(stack, Operator::Add)),
            "-" => output.append(&mut eval_operator(stack, Operator::Sub)),
            "/" => output.append(&mut eval_operator(stack, Operator::Div)),
            "*" => output.append(&mut eval_operator(stack, Operator::Multi)),
            "(" => stack.push(Operator::LeftParent),
            ")" => output.append(&mut stack.close_parenthesis()),
            _ => {
                output.push(e);
            }
        }
    }
    
    output.append(&mut stack.dry());
    println!("{:?}", output);
    println!("{:?}", stack);
}

fn precedence(operator: &Operator) -> u8 {
    match operator {
        Operator::Add | Operator::Sub => 0,
        Operator::Multi | Operator::Div => 1,
        Operator::LeftParent => 2,
    }
}

fn eval_operator(stack: &mut Stack, incoming: Operator) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(incoming);
        return buffer
    };
    
    let stack_top = stack.top();
    
    if let Operator::LeftParent = stack_top {
        stack.push(incoming);
        return buffer;
    };
    
    if should_pop_operator(stack_top, &incoming) {
        let operator = stack.pop();
        buffer.push(Stack::operator_to_string(&operator));
        stack.push(incoming);
        
        return buffer;
    } else {
        stack.push(incoming);
        return buffer;
    };
}

fn should_pop_operator(stack_top: &Operator, incoming: &Operator) -> bool {
    precedence(incoming) <= precedence(stack_top)
}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_popped() {
        let sum = Operator::Add;
        let sub = Operator::Sub;
        let div = Operator::Div;
        let multi = Operator::Multi;
        
        assert_eq!(true, should_pop_operator(&sum, &sum));
        assert_eq!(true, should_pop_operator(&sum, &sub));
        assert_eq!(true, should_pop_operator(&sub, &sum));
        assert_eq!(true, should_pop_operator(&sub, &sub));
        assert_eq!(true, should_pop_operator(&div, &div));
        assert_eq!(true, should_pop_operator(&div, &multi));
        assert_eq!(true, should_pop_operator(&multi, &div));
        assert_eq!(true, should_pop_operator(&multi, &multi));
    }
    
    #[test]
    fn is_not_popped() {
        let sum = Operator::Add;
        let sub = Operator::Sub;
        let div = Operator::Div;
        let multi = Operator::Multi;
        
        assert_eq!(false, should_pop_operator(&sum, &div));
        assert_eq!(false, should_pop_operator(&sub, &div));
        assert_eq!(false, should_pop_operator(&sum, &multi));
        assert_eq!(false, should_pop_operator(&sub, &multi));
    }
    
    #[test]
    fn set_operators_precedence() {
        let sum = Operator::Add;
        let sub = Operator::Sub;
        let div = Operator::Div;
        let multi = Operator::Multi;
        let left_parent = Operator::LeftParent;
        
        assert_eq!(0, precedence(&sum));
        assert_eq!(0, precedence(&sub));
        assert_eq!(1, precedence(&div));
        assert_eq!(1, precedence(&multi));
        assert_eq!(2, precedence(&left_parent));
    }
}
use std::{io};
use std::process;

use crate::stack::{OperatorType, Stack};

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
            "+" => {
                let buffer = eval_add_op(stack);
                
                for b in buffer {
                    output.push(b)
                }
            },
            "-" => stack.push(OperatorType::Sub),
            "/" => stack.push(OperatorType::Div),
            "*" => stack.push(OperatorType::Multi),
            "(" => stack.push(OperatorType::LeftParent),
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

fn eval_add_op(stack: &mut Stack) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(OperatorType::Add);
        return buffer
    }
    
    match stack.top() {
        OperatorType::Add => {
            stack.pop_and_discard();
            stack.push(OperatorType::Add);
            buffer.push(String::from("+"));
            
            return buffer;
        },
        OperatorType::Sub => {
            stack.pop_and_discard();
            stack.push(OperatorType::Add);
            buffer.push(String::from("-"));
            
            return buffer;
        },
        OperatorType::Div => {
            stack.pop_and_discard();
            stack.push(OperatorType::Add);
            buffer.push(String::from("/"));
            
            return buffer;
        },
        OperatorType::Multi => {
            stack.pop_and_discard();
            stack.push(OperatorType::Add);
            buffer.push(String::from("*")); 
            
            return buffer;
        },
        OperatorType::LeftParent => {
            stack.push(OperatorType::Add);
            
            return buffer;
        },
        _ => {
            println!("This shouldn't be occurs!");
            process::exit(1);
        },
    }
}

use std::{io};
use std::process;

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
            "+" => output.append(&mut eval_add_op(stack)),
            "-" => output.append(&mut eval_sub_op(stack)),
            "/" => output.append(&mut eval_div_op(stack)),
            "*" => output.append(&mut eval_multi_op(stack)),
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

fn eval_add_op(stack: &mut Stack) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(Operator::Add);
        return buffer
    }
    
    match stack.top() {
        Operator::Add => {
            stack.pop_and_discard();
            stack.push(Operator::Add);
            buffer.push(String::from("+"));
            
            return buffer;
        },
        Operator::Sub => {
            stack.pop_and_discard();
            stack.push(Operator::Add);
            buffer.push(String::from("-"));
            
            return buffer;
        },
        Operator::Div => {
            stack.pop_and_discard();
            stack.push(Operator::Add);
            buffer.push(String::from("/"));
            
            return buffer;
        },
        Operator::Multi => {
            stack.pop_and_discard();
            stack.push(Operator::Add);
            buffer.push(String::from("*")); 
            
            return buffer;
        },
        Operator::LeftParent => {
            stack.push(Operator::Add);
            
            return buffer;
        },
        _ => {
            println!("This shouldn't be occurs!");
            process::exit(1);
        },
    }
}

fn eval_sub_op(stack: &mut Stack) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(Operator::Sub);
        return buffer
    }
    
    match stack.top() {
        Operator::Add => {
            stack.pop_and_discard();
            stack.push(Operator::Sub);
            buffer.push(String::from("+"));
            
            return buffer;
        },
        Operator::Sub => {
            stack.pop_and_discard();
            stack.push(Operator::Sub);
            buffer.push(String::from("-"));
            
            return buffer;
        },
        Operator::Div => {
            stack.pop_and_discard();
            stack.push(Operator::Sub);
            buffer.push(String::from("/"));
            
            return buffer;
        },
        Operator::Multi => {
            stack.pop_and_discard();
            stack.push(Operator::Sub);
            buffer.push(String::from("*")); 
            
            return buffer;
        },
        Operator::LeftParent => {
            stack.push(Operator::Sub);
            
            return buffer;
        },
        _ => {
            println!("This shouldn't be occurs!");
            process::exit(1);
        },
    }
}

fn eval_div_op(stack: &mut Stack) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(Operator::Div);
        return buffer
    }
    
    match stack.top() {
        Operator::Add => {
            stack.push(Operator::Div);
            return buffer;
        },
        Operator::Sub => {
            stack.push(Operator::Div);
            return buffer;
        },
        Operator::Div => {
            stack.pop_and_discard();
            stack.push(Operator::Div);
            buffer.push(String::from("/"));
            
            return buffer;
        },
        Operator::Multi => {
            stack.pop_and_discard();
            stack.push(Operator::Div);
            buffer.push(String::from("*")); 
            
            return buffer;
        },
        Operator::LeftParent => {
            stack.push(Operator::Div);
            
            return buffer;
        },
        _ => {
            println!("This shouldn't be occurs!");
            process::exit(1);
        },
    }
}

fn eval_multi_op(stack: &mut Stack) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();
    
    if stack.is_empty() {
        stack.push(Operator::Multi);
        return buffer
    }
    
    match stack.top() {
        Operator::Add => {
            stack.push(Operator::Multi);
            return buffer;
        },
        Operator::Sub => {
            stack.push(Operator::Multi);
            return buffer;
        },
        Operator::Div => {
            stack.pop_and_discard();
            stack.push(Operator::Multi);
            buffer.push(String::from("/"));
            
            return buffer;
        },
        Operator::Multi => {
            stack.pop_and_discard();
            stack.push(Operator::Multi);
            buffer.push(String::from("*")); 
            
            return buffer;
        },
        Operator::LeftParent => {
            stack.push(Operator::Multi);
            
            return buffer;
        },
        _ => {
            println!("This shouldn't be occurs!");
            process::exit(1);
        },
    }
}
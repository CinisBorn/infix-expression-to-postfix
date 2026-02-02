use std::process;

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    LeftParent,
    Multi,
    Add,
    Sub,
    Div,
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Operator>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new() 
        }
    }
    
    pub fn top(&self) -> &Operator {
        self.stack.last().expect("Return last item")
    }
    
    pub fn pop_and_discard(&mut self) {
        self.stack.pop().expect("Remove last item");
    }
    
    pub fn pop(&mut self) -> Operator {
        self.stack.pop().expect("Remove last item")
    }
    
    pub fn push(&mut self, value: Operator) {
        self.stack.push(value);
    }
    
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    
    pub fn dry(&mut self) -> Vec<String> {
        let mut remains: Vec<String> = Vec::new();
        
        for _ in 0..self.len() {
            match self.pop() {
                Operator::Add => remains.push(String::from("+")),
                Operator::Sub => remains.push(String::from("-")),
                Operator::Div => remains.push(String::from("/")),
                Operator::Multi => remains.push(String::from("*")),
                _ => {
                    continue 
                }
            }
        }
        
        remains
    }
    
    pub fn close_parenthesis(&mut self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        
        for _ in 0..self.len() {
            match self.pop() {
                Operator::Add => output.push(String::from("+")),
                Operator::Sub => output.push(String::from("-")),
                Operator::Div => output.push(String::from("/")),
                Operator::Multi => output.push(String::from("*")),
                Operator::LeftParent => break,
                _ => {
                    eprint!("It shouldn't be possible... report it as an issues");
                    process::exit(1);
                }
            }
        }
        
        output
    }
}

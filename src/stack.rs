#[derive(Copy, Clone, Debug)]
pub enum OperatorType {
    RightParent,
    LeftParent,
    Multi,
    Add,
    Sub,
    Div,
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<OperatorType>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new() 
        }
    }
    
    pub fn top(&self) -> &OperatorType {
        self.stack.last().expect("Return last item")
    }
    
    pub fn pop_and_discard(&mut self) {
        self.stack.pop().expect("Remove last item");
    }
    
    pub fn pop(&mut self) -> OperatorType {
        self.stack.pop().expect("Remove last item")
    }
    
    pub fn push(&mut self, value: OperatorType) {
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
        
        for _ in [0..self.len()] {
            match self.pop() {
                OperatorType::Add => remains.push(String::from("+")),
                OperatorType::Sub => remains.push(String::from("-")),
                OperatorType::Div => remains.push(String::from("/")),
                OperatorType::Multi => remains.push(String::from("*")),
                _ => {
                    continue 
                }
            }
        }
        
        remains
    }
}

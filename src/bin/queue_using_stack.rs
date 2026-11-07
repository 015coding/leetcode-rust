struct MyQueue {
    input : Vec<i32>,
    output: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            input : Vec::new(),
            output : Vec::new(),
        }
    }
    fn shift(&mut self) {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop(){
                self.output.push(x);
            }
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.shift();
        self.output.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self.shift();
        *self.output.last().unwrap()
    }
    
    fn empty(&mut self) -> bool {
        self.shift();
        if self.output.is_empty() {
            return true;
        }
        return false
    }
}

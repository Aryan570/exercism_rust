//** We should evaluate everywhere we have value as key_word in the definition of any other word in hashmap,
//**Since we have a new definition (updated value) for key_word
//**eg. if previously bar ------> foo
//**                  foo ------>  5
//** new definition   bar ------>  5 
//**                  foo ------>  2
use std::collections::HashMap;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
const ADD : &str = "+";
const SUBTRACT : &str = "-";
const MULTIPLY : &str = "*";
const DIVIDE : &str = "/";
const DROP : &str = "drop";
const DUP : &str = "dup";
const SWAP : &str = "swap";
const OVER : &str = "over";
const START: &str = ":";
const END: &str = ";";


#[derive(Default)]
pub struct Forth{
   stack : Vec<Value>,
   mpp : HashMap<String,String>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    fn arithmatic(&mut self,operation : &str) -> Result{
        let right = self.stack.pop();
        let left = self.stack.pop();
        let (left,right) = match (left,right) {
            (None,_) => return Err(Error::StackUnderflow),
            (_,None) => return Err(Error::StackUnderflow),
            (_,Some(0)) if operation == DIVIDE => return Err(Error::DivisionByZero),
            (Some(l),Some(r)) => (l,r),
        };
        match operation{
            ADD => self.stack.push(left + right),
            SUBTRACT => self.stack.push(left - right),
            MULTIPLY => self.stack.push(left * right),
            DIVIDE => self.stack.push(left/right),
            _ => return Err(Error::InvalidWord),
        }
        Ok(())
    }
    fn stack_operations(&mut self,operation : &str)->Result{
        let right = match self.stack.pop(){
            Some(val) => val,
            None => return Err(Error::StackUnderflow)
        };
        match operation{
            DROP => Ok(()),
            DUP => {
                self.stack.push(right);
                self.stack.push(right);
                Ok(())
            }
            SWAP => match self.stack.pop() {
                Some(left) => {
                    self.stack.push(right);
                    self.stack.push(left);
                    return Ok(());
                },
                None => Err(Error::StackUnderflow)
            }
            OVER => match self.stack.pop() {
                Some(left) => {
                    self.stack.push(left);
                    self.stack.push(right);
                    self.stack.push(left);
                    return Ok(());
                },
                None => Err(Error::StackUnderflow)
            }
            _ => Err(Error::InvalidWord)
        }
    }
    fn replace_key_word(&mut self,word : &str, old_val : &str){
        for (key,val) in self.mpp.clone().iter(){
            if val.split_whitespace().any(|k| k == word){
                let mut new_val : Vec<_> = val.split_whitespace().collect();
                for w in &mut new_val{
                    if *w == word {*w = old_val;}
                }
                let new_val_string = new_val.join(" ");
                self.mpp.insert(key.clone(), new_val_string);
            }
        }
    }


    pub fn eval(&mut self, input: &str) -> Result {
        let mut running_add_word = false;
        let mut key_word = String::new();
        let mut value_of_word = String::new();
        for new_input in input.split_whitespace(){
            let lower_new_input = &new_input.to_ascii_lowercase()[..];
            match lower_new_input {
                START if !running_add_word =>{
                    running_add_word = true;
                    key_word = String::new();
                    value_of_word = String::new();
                }
                END if running_add_word =>{
                    running_add_word = false;
                    //**Check on top */
                    if let Some(old_value) = self.mpp.insert(key_word.clone(), value_of_word.clone()){
                        self.replace_key_word(&key_word, &old_value);
                    }
                }
                is_new_word_num if running_add_word && key_word.len() == 0 && is_new_word_num.parse::<Value>().is_ok() => return Err(Error::InvalidWord),
                key if running_add_word && key_word.len() == 0 => key_word = key.to_string(),
                val_of_key if running_add_word => {
                    value_of_word.push(' ');
                    value_of_word.push_str(val_of_key);
                }
                custom_word if self.mpp.contains_key(custom_word) => {
                    let ip = self.mpp.get(custom_word).unwrap().to_owned();
                    //checking nested stuff
                    self.eval(&ip)?
                }
                num if num.parse::<Value>().is_ok() => self.stack.push(num.parse::<Value>().unwrap()),
                ADD | SUBTRACT | MULTIPLY | DIVIDE => self.arithmatic(lower_new_input)?,
                DROP | DUP | SWAP | OVER => self.stack_operations(lower_new_input)?,
                _ => return Err(Error::UnknownWord)
            }
        }
        if running_add_word {return Err(Error::InvalidWord);}
        Ok(())
    }
}

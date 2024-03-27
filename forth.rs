// 15 tests are failing right now. Not considering benchmark performance
// and yet to write the make_hashmap function
// this is to record the history of this code.
use std::collections::HashMap;
// now remains the worded ones.
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth{
   ans : Vec<i32>,
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
        let mut tmp = HashMap::new();
        tmp.insert("drop".to_string(),"drop".to_string());
        tmp.insert("over".to_string(),"over".to_string());
        tmp.insert("swap".to_string(),"swap".to_string());
        tmp.insert("dup".to_string(),"dup".to_string());
        tmp.insert("+".to_string(),"+".to_string());
        tmp.insert("*".to_string(),"*".to_string());
        tmp.insert("/".to_string(),"/".to_string());
        tmp.insert("-".to_string(),"-".to_string());
        Forth { ans: Vec::new(), mpp: tmp }
    }

    pub fn stack(&self) -> &[Value] {
        self.ans.as_slice()
    }
    
    fn make_hashmap(ip :&String,tmp_mpp : &mut HashMap<String,String>) -> (){
        

    }

    pub fn eval(&mut self, input: &str) -> Result {
        let ip = input.to_string();
        if ip.contains(':') && ip.contains(';'){
            Self::make_hashmap(&ip,&mut self.mpp);
        }
        let ip_vec : Vec<_> = ip.split(' ').map(|x| x.to_owned()).collect();
        let mut stack = Vec::new();
        for s in ip_vec{
            match s.to_ascii_lowercase().as_str() {
                f @ ("+" | "/" | "-" | "*") => {
                    if stack.len() == 0 {return Err(Error::StackUnderflow);}
                    let num1 : Value = stack.pop().unwrap();
                    if stack.len() == 0 {return Err(Error::StackUnderflow);}
                    let num2: Value = stack.pop().unwrap();
                    if num1 == 0 && f == "/" {return Err(Error::DivisionByZero);}
                    match self.mpp.get(f).unwrap().as_str() {
                        "+" => stack.push(num1 + num2),
                        "/" => stack.push(num2 / num1),
                        "-" => stack.push(num2 - num1),
                        "*" => stack.push(num1 * num2),
                        _ => println!("Nah")
                    }
                },
                p @ ("dup" | "drop" | "swap" | "over") => {
                    if stack.len() == 0 || (p == "over" && stack.len() < 2){ return Err(Error::StackUnderflow);}
                    let mut num1 = 0; let mut num2 = 0;
                    if p == "dup" {
                        num1 = *stack.last().clone().unwrap();
                    }
                    if p == "swap" {
                        num1 = stack.pop().unwrap(); 
                        if stack.len() == 0 {return Err(Error::StackUnderflow);}
                        num2 = stack.pop().unwrap();
                    }
                    match p {
                        "dup" =>  stack.push(num1),
                        "drop" => {stack.pop();},
                        "swap" => {stack.push(num1);stack.push(num2);}
                        "over" => stack.push(*stack.get(stack.len() - 2).unwrap()),
                        _ => println!("Nah2")
                    }

                }
                // "dup" => {
                //     if stack.len() == 0 {return Err(Error::StackUnderflow);}
                //     let num1 : Value = *stack.last().clone().unwrap();
                //     stack.push(num1);
                // },
                // "drop" => {
                //     if stack.len() == 0 {return Err(Error::StackUnderflow);}
                //     stack.pop();
                // },
                // "swap" => {
                //     if stack.len() == 0 {return Err(Error::StackUnderflow);}
                //     let num1 : Value = stack.pop().unwrap();
                //     if stack.len() == 0 {return Err(Error::StackUnderflow);}
                //     let num2: Value = stack.pop().unwrap();
                //     stack.push(num1);stack.push(num2);
                // },
                // "over" => {
                //     if (stack.len()) < 2 {return Err(Error::StackUnderflow);}
                //     stack.push(*stack.get(stack.len() - 2).unwrap());
                // },
                num_or_word @ _ => {
                    // but this can have numbers as well as random string
                    if s.parse::<i32>().is_ok() {
                        stack.push(s.parse().unwrap());
                    }else {
                        stack.push(self.mpp.get(num_or_word).unwrap().parse().unwrap());
                    }
                },
            }
        }
        self.ans = stack;
        Ok(())
    }
}

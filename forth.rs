//So hear me out, although my code passed all the tests.
// but it is vulnerable to OOM attack
// and on the benchmark, it should have taken 1 MB only, but it did 993 MB. How the hell did that happen?
// so its no good
use std::collections::{HashMap, VecDeque};
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
        let mut vec : VecDeque<_> = ip.split(' ').collect();
        vec.pop_back();vec.pop_front();
        if vec.len() > 2 {
            let mut s = String::new();
            for v in 1..vec.len(){
                if tmp_mpp.contains_key(vec[v].to_ascii_lowercase().as_str()){
                    // *v = tmp_mpp.get(v.to_ascii_lowercase().as_str())
                    vec[v] = tmp_mpp.get(vec[v].to_ascii_lowercase().as_str()).unwrap();
                }
                s.push_str(vec[v].to_ascii_lowercase().as_str());
                s.push(' ');
            }
            s.pop();
            tmp_mpp.insert(vec[0].to_ascii_lowercase().to_string(), s);
        }else if vec.len() == 2{
            let key = *vec.get(0).unwrap();
            let val = *vec.get(1).unwrap();
            let mut value = val.to_string();
            if tmp_mpp.contains_key(val.to_ascii_lowercase().as_str()){
                value = tmp_mpp.get(val.to_ascii_lowercase().as_str()).unwrap().to_string();
            }
            tmp_mpp.insert(key.to_string().to_ascii_lowercase(), value);
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut ip = input.to_string();
        let mut for_eval = String::new();
        let mut for_map = String::new();
        let mut flag = false;
        let mut tmp_vec = Vec::new();
        if ip.contains(':') && ip.contains(';'){
            for sam in ip.chars(){
                if sam == ':'{flag = true;continue;}
                if sam == ';'{flag = false; tmp_vec.push(for_map); for_map = String::new(); continue;}
                if flag{
                    for_map.push(sam);
                }else {
                    for_eval.push(sam);
                }
            }
        }
        for v in tmp_vec{
            Self::make_hashmap(&v, &mut self.mpp);
        }
        for k in 0..=9{
            if self.mpp.contains_key(&k.to_string()){
                return Err(Error::InvalidWord);
            }
        }
        if for_eval.len() > 0 {ip = for_eval;}
        else if ip.contains(':'){
            if ip.contains(';'){
                return Ok(());
            }else {
                return Err(Error::InvalidWord);
            }
        }
        for (key, value) in &self.mpp{
            if key != value && !key.parse::<i32>().is_ok(){
                ip = ip.replace(key, value);
            }
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

                },
                "" => continue,
                num_or_word @ _ => {
                    if s.parse::<i32>().is_ok() {
                        if self.mpp.contains_key(num_or_word){
                                return Err(Error::InvalidWord);
                        }
                        stack.push(s.parse().unwrap());
                    }else {
                        let val = self.mpp.get(num_or_word);
                        if val.is_some(){
                            match val.unwrap().as_str() {
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
                
                                },
                                _ => return Err(Error::InvalidWord)
                            }
                        }else{return Err(Error::UnknownWord);}
                    }
                },
            }
        }
        self.ans = stack;
        Ok(())
    }
}

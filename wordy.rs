fn get_answer(input : &str) -> Option<i32>{
    let mut c = input.to_string();c.pop();
    let v : Vec<&str> = c.as_str().split_whitespace().collect();
    let mut stack: Vec<i32> = Vec::new();
    let mut operator = None;
    for s in v{
        match s {
            "multiplied" | "plus" | "minus" | "divided" => {
                if operator.is_some() || stack.is_empty(){return None;}
                operator = Some(s);
            }
            _ if s.parse::<i32>().is_ok() => {
                if !stack.is_empty() && operator.is_some(){
                    match operator.unwrap() {
                        "multiplied" =>{
                            let tmp = stack[0] * s.parse::<i32>().unwrap();
                            stack.clear();
                            stack.push(tmp);
                        }
                        "plus" => {
                            let tmp = stack[0] + s.parse::<i32>().unwrap();
                            stack.clear();
                            stack.push(tmp);
                        }
                        "minus" => {
                            let tmp = stack[0] - s.parse::<i32>().unwrap();
                            stack.clear();
                            stack.push(tmp);
                        }
                        "divided" => {
                            let tmp = stack[0] / s.parse::<i32>().unwrap();
                            stack.clear();
                            stack.push(tmp);
                        }
                        _ => println!("not supposed to be here")
                    }
                    operator = None;
                }else if stack.is_empty(){
                    stack.push(s.parse().unwrap());
                }else{
                    return None;
                }
            }
            _ => {
                if s == "by" && (operator.unwrap() == "multiplied" || operator.unwrap() == "divided"){
                    continue;
                }
                return None;
            }
        }
    }
    if operator.is_some() {return None;}
    Some(stack[0])
}
pub fn answer(command: &str) -> Option<i32> {
    if command.contains("raised"){
        let v : Vec<&str> = command.split("raised to the ").collect();
        let c : Vec<&str> = v[0].split(" is ").collect();
        let left_side = get_answer(c[1]);
        let mut tmp = String::new();
        for i in 0..v[1].len(){
            match v[1].chars().nth(i).unwrap() {
                t @ '0'..='9' =>{
                    tmp.push(t);
                }
                _ => {
                    break;
                }
            }
        }
        return Some(left_side.unwrap().pow(tmp.parse::<i32>().unwrap() as u32));
    }
    let v : Vec<&str> = command.split(" is ").collect();
    if v.len() <= 1 {return None;}
    let ans = get_answer(v[1]);
    ans
}

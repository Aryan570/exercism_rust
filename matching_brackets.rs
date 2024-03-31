pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for s in string.chars(){
        match s {
            '{' | '(' | '[' => stack.push(s),
            b @ ('}' | ')' | ']') => {
                let top = stack.last();
                if top.is_some(){
                    let brac = top.unwrap();
                    if (b == '}' && *brac == '{') || (b == ')' && *brac == '(') || (b == ']' && *brac == '['){
                        stack.pop();
                    }else {return false;}
                }else {return false;}
            }
            _ => continue,
        }
    }
    stack.is_empty()
}

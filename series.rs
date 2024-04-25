pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {return vec![];}
    let mut v: Vec<String> = Vec::new();
    for i in 0..(digits.len() - len + 1){
        v.push((&digits[i..(i+len)]).to_string());
    }
    v
}

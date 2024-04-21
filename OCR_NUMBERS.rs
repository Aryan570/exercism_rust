use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}
fn get_string(collecting : &HashMap<u8,String>,patt_to_num : &HashMap<&str,u8>) -> String{
    let mut to_return = String::new();
    for i in 0..collecting.len(){
        let key = collecting.get(&(i as u8)).unwrap();
        let sub_str = patt_to_num.get(key.as_str());
        if sub_str.is_none(){
            to_return.push('?');
            continue;
        }
        to_return.push_str(sub_str.unwrap().to_string().as_str());
    }
    to_return
}
pub fn convert(input: &str) -> Result<String, Error> {
    let mut ans = String::new();
    let patt_to_num: HashMap<&str, u8> = HashMap::from([
        (" _ | ||_|   ", 0),
        ("     |  |   ", 1),
        (" _  _||_    ", 2),
        (" _  _| _|   ", 3),
        ("   |_|  |   ", 4),
        (" _ |_  _|   ", 5),
        (" _ |_ |_|   ", 6),
        (" _   |  |   ", 7),
        (" _ |_||_|   ", 8),
        (" _ |_| _|   ", 9),
    ]);
    let mut processed = 0;
    let mut collecting: HashMap<u8, String> = HashMap::new();
    let mut num_lines = 0;
    let mut is_multiline = false;
    for line in input.lines() {
        if processed > 3 {
            if is_multiline {ans.push(',');}
            ans.push_str(&get_string(&collecting,&patt_to_num));
            collecting.clear();
            processed = 0;
            is_multiline = true;
        }
        if line.len() % 3 !=0 {return Err(Error::InvalidColumnCount(line.len()));}
        for (i,ch) in line.char_indices(){
            collecting.entry(i as u8/3).and_modify(|s| s.push(ch)).or_insert(ch.to_string());
        }
        processed += 1;
        num_lines += 1;
    }
    if num_lines % 4 !=0 {return Err(Error::InvalidRowCount(num_lines));}
    if num_lines > 4 {ans.push(',');}
    ans.push_str(&get_string(&collecting,&patt_to_num));
    Ok(ans)
}

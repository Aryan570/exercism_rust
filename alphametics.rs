// This one timed out.
use std::collections::{HashMap, HashSet};
fn check_recur(idx : usize,letter : &Vec<char>,char_to_int : &mut HashMap<char,u8>,num_visited : &mut HashSet<u8>,char_visited :&mut HashSet<char>,left : &String,right :&String, ans : &mut HashMap<char,u8>,is_first_letter : &HashSet<char>){
    if idx >= letter.len() {
        let mut left_dup = left.clone();
        left_dup = left_dup.chars().map(|c| {
            if char_to_int.contains_key(&c){
                let n = char_to_int.get(&c).unwrap();
                return (*n + 48) as char
            }
            c
        }).collect();
        let mut left_sum = 0;
        let v : Vec<_> = left_dup.trim().split("+").collect();
        for num in v{
            if num.trim().parse::<i64>().is_ok(){
                left_sum += num.trim().parse::<i64>().unwrap();
            }
        }
        let mut right_dup = right.clone();
        right_dup = right_dup.chars().map(|c| {
            if char_to_int.contains_key(&c){
                let n = char_to_int.get(&c).unwrap();
                return (*n + 48) as char
            }
            c
        }).collect();
        let mut right_sum = 0;
        let v = right_dup.trim();
        if v.parse::<i64>().is_ok(){
            right_sum += v.parse::<i64>().unwrap();
        }
        if left_sum == right_sum && left_sum !=0{
            *ans = char_to_int.clone();
        }
        return;
    }
    if char_visited.contains(&letter[idx]){return;}
    for i in 0..=9 as u8{
        if is_first_letter.contains(&letter[idx]) && i == 0{continue;}
        if !num_visited.contains(&i){
            char_to_int.insert(letter[idx], i);
            num_visited.insert(i);
            char_visited.insert(letter[idx]);
            check_recur(idx + 1, letter, char_to_int, num_visited, char_visited,left,right,ans,is_first_letter);
            char_to_int.remove(&letter[idx]);
            num_visited.remove(&i);
            char_visited.remove(&letter[idx]);
        }
    }
    return;
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut is_first_letter = HashSet::new(); 
    for l in input.split_whitespace(){
        is_first_letter.insert(l.chars().next().unwrap());
    }
    let mut letter = Vec::new();
    for ch in input.chars(){
        if ch.is_alphabetic(){letter.push(ch);}
    }
    let set: HashSet<_> = letter.drain(..).collect();
    letter.extend(set.into_iter());
    let mut char_to_int :HashMap<char, u8> = HashMap::new();
    let mut num_visited :HashSet<u8> = HashSet::new();
    let mut char_visited : HashSet<char> = HashSet::new();
    let v : Vec<&str> = input.split("==").collect();
    let left = v[0].to_string();
    let right = v[1].to_string();
    let mut ans: HashMap<char, u8> = HashMap::new();
    check_recur(0,&letter,&mut char_to_int,&mut num_visited,&mut char_visited,&left,&right,&mut ans, &is_first_letter);
    if ans.len() == 0 {return None;}
    Some(ans)
}

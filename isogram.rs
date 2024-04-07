// checkout the bit solution on exercism
use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut vis : HashSet<char> = HashSet::new();
    for ch in candidate.chars(){
        match ch.to_ascii_lowercase() {
            c @ 'a'..='z' =>{
                if vis.contains(&c){
                    return false;
                }else{vis.insert(c);}
            }
            _ => continue
        }
    }
    true
}

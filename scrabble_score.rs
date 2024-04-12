use std::collections::HashMap;
/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score = 0;
    let mpp = HashMap::from([('a',1),('e',1),('i',1),('o',1),('u',1),('l',1),('n',1),('r',1),('s',1),('t',1),('d',2),('g',2),('b',3),('c',3),('m',3),('p',3),('f',4),('h',4),('v',4),('w',4),('y',4),('k',5),('j',8),('x',8),('q',10),('z',10)]);
    for ch in word.chars(){
        let c = mpp.get(&ch.to_ascii_lowercase());
        if c.is_some(){
            score += c.unwrap();}
    }
    score
}

// optimized
const MPP: &'static [u64; 26] = &[1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,];
pub fn score(word: &str) -> u64 {
    word.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| MPP[c.to_ascii_lowercase() as usize - 97]).sum()
}

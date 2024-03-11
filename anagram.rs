use std::collections::{HashSet , BTreeMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut answer : HashSet<&str> = HashSet::new();
    let mut hash_for_word = BTreeMap::new();
    for ch in word.to_lowercase().chars() {
        hash_for_word.entry(ch).and_modify(|val| *val+=1).or_insert(1);
    }
    for x in possible_anagrams {
        let mut check = BTreeMap::new();
        for ch in x.to_lowercase().chars(){
            check.entry(ch).and_modify(|val| *val+=1).or_insert(1);
        }
        if check == hash_for_word && word.to_lowercase()!=*x.to_lowercase(){
            answer.insert(x);
        }
    }
    answer
}
// Subject to review. We'll get here later.

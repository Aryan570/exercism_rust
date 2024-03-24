// hardest till date
use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input_string : Vec<String> = input.iter().map(|&s| s.to_owned()).collect();
    let mpp = Arc::new(Mutex::new(HashMap::new()));
    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    if chunk_size == 0 {return HashMap::new();}
    let mut handles = vec![];
    for chunk in input_string.chunks(chunk_size){
        // to_owned() owns the data that it borrowed usually by cloning
        let curr_input_string :Vec<String> = chunk.iter().map(|s| s.to_owned()).collect();
        let mpp_clone = Arc::clone(&mpp);
        let handle = thread::spawn(move||{
            let mut tmp_map = HashMap::new();
            for s in curr_input_string{
                for ch in s.chars(){
                    if ch.is_alphabetic(){
                        tmp_map.entry(ch.to_ascii_lowercase()).and_modify(|cnt| *cnt+=1).or_insert(1);
                    }
                }
            }
            let mut mpp = mpp_clone.lock().unwrap();
            for (key, val) in tmp_map{
                *mpp.entry(key).or_insert(0) += val;
            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    let ans_map = mpp.lock().unwrap();
    ans_map.clone()
}



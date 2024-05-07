use std::collections::HashMap;
/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut mpp: HashMap<String, u32> = HashMap::new();
    let mut ap_cnt = 0;
    let mut tmp_str = String::new();
    for byte in words.to_ascii_lowercase().as_bytes(){
        match byte {
            b'a'..=b'z' | b'0'..=b'9' =>{
                if ap_cnt == 1 {
                    tmp_str.push('\'');
                    ap_cnt += 1;
                }
                tmp_str.push(*byte as char);
            }
            b' ' => {
                if tmp_str.len() > 0 {
                    mpp.entry(tmp_str).and_modify(|cnt| *cnt += 1).or_insert(1);
                    tmp_str = String::new();
                }
                ap_cnt = 0;
            }
            b'\'' =>{
                if tmp_str.len() > 0 {
                    ap_cnt += 1;
                }
            }
            _ => {
                if tmp_str.len() > 0 {
                    mpp.entry(tmp_str).and_modify(|cnt| *cnt += 1).or_insert(1);
                    tmp_str = String::new();
                }
                ap_cnt = 0;
            }
        }
    }
    if tmp_str.len() > 0 {
        mpp.entry(tmp_str).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    mpp
}

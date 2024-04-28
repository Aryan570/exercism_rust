// will optimize later..
fn get_normal(input: &str) -> String{
    let mut ans = String::new();
    for ch in input.to_ascii_lowercase().chars(){
        match ch {
            c @ ('a'..='z' | '0'..='9') =>{
                ans.push(c);
            }
            _ => continue,
        }
    }
    ans
}
fn get_r_c(length : usize) -> (usize,usize){
    let mut r = 0;
    let mut c = 0;
    for i in 1..{
        let sq = i * i;
        if sq >= length{
            if sq == length {
                r = i; c = i; break;
            }
            if sq > length{
                if (i * (i-1)) >= length{
                    r = i - 1; c = i; break;
                }else {
                    r = i; c = i; break;
                }
            }
        }
    }
    (r,c)
}
pub fn encrypt(input: &str) -> String {
    let s = get_normal(input);
    let (r , c) = get_r_c(s.len());
    let mut v = vec!["".to_string();r];
    for i in s.as_bytes().chunks(c){
        let tmp_s : String = i.into_iter().map(|c| *c as char).collect();
        v.push(format!("{:<c$}",tmp_s));
    }
    let mut ans_v = vec!["".to_string();c];
    for i in 0..v.len(){
        for (idx,ch) in v[i].char_indices(){
            ans_v[idx].push(ch);
        }
    }
    ans_v.join(" ")
}

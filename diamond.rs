// need review, too much cloning going on here.
pub fn get_diamond(c: char) -> Vec<String> {
    let num = c as u8 - b'A';
    let mut ans = String::new();
    let mut tmp = num  as i32;
    let mut ch = b'A';
    let mut ans_v = Vec::new();
    loop {
        for _ in 0..tmp{
            ans.push(' ');
        }
        ans.push(ch as char);
        ch = ch + 1 as u8;
        for _ in tmp..num as i32{
            ans.push(' ');
        }
        tmp -= 1;
        ans_v.push(ans.clone());
        ans.clear();
        if tmp <0 {break;}
    }
    for i in 0..ans_v.len(){
        let mut dup = ans_v[i].clone();
        dup.pop();
        ans_v[i].push_str(&dup.chars().rev().collect::<String>());
    }
    let c = ans_v.len();
    for i in (0..(c - 1)).rev(){
        ans_v.push(ans_v[i].clone());
    }
    ans_v
}

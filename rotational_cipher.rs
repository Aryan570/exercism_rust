pub fn rotate(input: &str, key: u8) -> String {
    let mut ans = String::new();
    for ch in input.as_bytes(){
        match ch {
            c @ b'a'..=b'z' => ans.push((b'a'+ ((c - b'a' + key)%26 )) as char),
            c @ b'A'..=b'Z' => ans.push((b'A'+ ((c - b'A' + key)%26 )) as char),
            _ => ans.push(*ch as char),
        }
    }
    ans
}

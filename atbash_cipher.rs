/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut ans = String::new();
    let mut cnt = 0;
    for ch in plain.to_ascii_lowercase().as_bytes(){
        if cnt == 5 {
            ans.push(' ');
            cnt = 0;
        }
        match ch {
            n @ b'0'..=b'9' =>{
                ans.push(*n as char);
                cnt += 1;
            }
            c @ b'a'..=b'z' => {
                ans.push((122 - c + b'a') as char);
                cnt += 1;
            }
            _ => continue,
        }
    }
    if cnt == 0 {ans.pop();}
    ans
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut ans = String::new();
    for ch in cipher.as_bytes(){
        match ch {
            n @ b'0'..=b'9' =>{
                ans.push(*n as char);
            }
            c @ b'a'..=b'z' => {
                ans.push((122 - c + b'a') as char);
            }
            _ => continue,
        }
    }
    ans
}

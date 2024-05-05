use rand::Rng;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut ans = String::new();
    let v = key.as_bytes();
    let mut idx = 0;
    for ch in s.as_bytes() {
        if idx >= v.len() {
            ans.push(*ch as char);
            continue;
        }
        match v[idx] {
            b'a'..=b'z' => {
                ans.push((b'a' + (ch - b'a' + v[idx] - b'a') % 26) as char);
                idx += 1;
            }
            _ => return None,
        }
    }
    Some(ans)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut ans = String::new();
    let v = key.as_bytes();
    let mut idx = 0;
    for ch in s.as_bytes() {
        if idx >= v.len() {
            ans.push(*ch as char);
            continue;
        }
        match v[idx] {
            b'a'..=b'z' => {
                ans.push(( b'a' + (*ch as i32 - v[idx] as i32).rem_euclid(26) as u8) as char);
                idx += 1;
            }
            _ => return None,
        }
    }
    Some(ans)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key : String = (0..=100).map(|_|{
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();
    let s = encode(&key, s).unwrap();
    (key,s)
}

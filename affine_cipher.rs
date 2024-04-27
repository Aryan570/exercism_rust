/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a % 13 == 0 || a % 2 == 0 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut encoded = String::new();
    let mut cnt = 0;
    for ch in plaintext.to_ascii_lowercase().as_bytes() {
        match ch {
            c if !c.is_ascii_alphanumeric() => continue,
            _ => {
                if ch.is_ascii_digit(){
                    encoded.push(*ch as char);
                    cnt += 1;
                    if cnt > 0 && cnt % 5 == 0 {encoded.push(' ');}
                    continue;
                }
                let e = (( a * (ch - b'a') as i32) + b) % 26;
                encoded.push((e as u8 + b'a') as char);
                cnt += 1;
                if cnt > 0 && cnt % 5 == 0 {encoded.push(' ');}
            }
        }
    }
    if cnt > 0 && cnt % 5 == 0 {encoded.pop();}
    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if a % 13 == 0 || a % 2 == 0 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut inv = 0;
    for i in 1..=25{
        if (a * i) % 26 == 1 {
            inv = i; break;
        }
    }
    let mut decoded = String::new();
    for ch in ciphertext.as_bytes(){
        match ch {
            b' ' => continue,
            _ => {
                if ch.is_ascii_digit(){
                    decoded.push(*ch as char);
                    continue;
                }
                let e = (inv * ((ch - b'a') as i32 - b)).rem_euclid(26);
                decoded.push((e as u8 + b'a') as char);
            }
        }
    }
    Ok(decoded)
}

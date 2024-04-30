use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {return Err(Error::SpanTooLong);}
    let mut result = 1;
    let mut ans = 0;
    let v = string_digits.as_bytes();
    let mut left = 0; let mut right = 0;
    while right < span {
        match v[right] {
            num @ b'0'..=b'9' => {
                result *= (num - b'0') as u64;
            }
            _ => return Err(Error::InvalidDigit(v[right] as char))
        }
        right += 1;
    }
    ans = cmp::max(result, ans);
    while right < v.len(){
        match v[left] {
            b'0' => {
                // something more optimized need to be done here
                result = 1;
                for i in (left + 1)..=(right - 1){
                    result *= (v[i] - b'0') as u64;
                }
            }
            num @ b'1'..=b'9' =>{
                result /= (num - b'0') as u64;
            }
            _ => return Err(Error::InvalidDigit(v[left] as char))
        }
        match v[right] {
            num @ b'0'..=b'9' => {
                result *= (num - b'0') as u64;
            }
            _ => return Err(Error::InvalidDigit(v[right] as char))
        }
        ans = cmp::max(result, ans);
        left += 1;
        right += 1;
    }
    Ok(ans as u64)
}

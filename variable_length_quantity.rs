#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut ans = Vec::new();
    for val in values.iter().rev(){
        let mut number = *val;
        ans.push((number & 0x7F) as u8);
        number = number >> 7;
        while number > 0{
            ans.push(((number & 0x7F) as u8) | 0x80);
            number = number >> 7;
        }
    }
    ans.reverse();
    ans
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut ans = Vec::new();
    let mut has_next = false;
    let mut c = 0;
    for byte in bytes{
        c = ((c << 7) | (*byte as u32 & 0x7F)) as u32;
        if byte & 0x80 != 0 {
            has_next = true;
        }else{
            ans.push(c);
            has_next = false;
            c = 0;
        }
    }
    if has_next {return Err(Error::IncompleteNumber);}
    Ok(ans)
}

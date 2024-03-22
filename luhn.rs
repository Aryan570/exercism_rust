/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    if code.len() <=1 {return false;}
    for byte in code.as_bytes(){
        if *byte != 32 && (*byte <48 || *byte >57){
            return false;
        }
    }
    let real : Vec<&str> = code.trim().split(' ').collect();
    let mut real_str = String::new();
    for s in real{
        real_str.push_str(s);
    }
    if real_str.len() <=1 {return false;}
    // by now, we must have our clean string, so now the algo part
    // rule 1
    let mut flag = false;
    let mut sum = 0;
    for num in 0..real_str.len(){
        sum += real_str.chars().nth(num).unwrap() as u32 - '0' as u32;
    }
    for i in (0..real_str.len() as i32).rev(){
        if flag{
           let mut r = real_str.chars().nth(i.try_into().unwrap()).unwrap() as u32 - '0' as u32;
           sum -= r;
           if r*2 > 9{
              r = (r*2) - 9;
           }else{
              r = r*2;
           }
           sum += r;
        }
        flag = !flag;
    }
    if sum%10 == 0{
        return true;
    }
    false
}
// you actually don't need to do anything with string. Improve the solution

//will circle back
pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    for s in phrase.split_whitespace(){
        let mut flag = false;
        let mut lower_case_taken = false;
        for c in s.chars(){
            if (c as u8 >= 65 && c as u8 <=90) && !flag{
                acronym.push(c); flag = true;
            }
            if (c as u8 >= 97 && c as u8 <=122) && !flag && !lower_case_taken{
                acronym.push(c); 
                if s.contains("-"){
                    lower_case_taken = true;
                    flag = false;
                }else {break;}
            }
            if (c as u8 >= 97 && c as u8 <=122) && flag{
                flag = false; lower_case_taken = true;
            }
            if c == '-' && lower_case_taken{
                lower_case_taken = false;
                flag = false;
            }
        }
    }
    acronym.to_ascii_uppercase()
}

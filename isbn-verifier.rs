pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut coeff = 10;
    let mut sum = 0;
    let mut cnt_digits = 0;
    let mut x = false;
    for ch in isbn.chars(){
        match ch {
            '0'..='9'=>{
                sum += coeff * (ch as u32 - '0' as u32);
                if coeff == 0 || x {return false;}
                coeff -= 1;
                cnt_digits += 1;
            }
            'X' => {
                sum += 10 * coeff;
                if coeff == 0 {return false;}
                coeff -= 1;
                x = true;
            }
            _ => continue
        }
    }
    if x {return sum % 11 == 0 && cnt_digits == 9;}
    sum % 11 == 0 && cnt_digits == 10
}

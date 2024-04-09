
use std::collections::HashSet;
pub fn is_pangram(sentence: &str) -> bool {
    sentence.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>().len() >= 26
}

// Brilliant Bit solution on exercism
const A_LCASE: u8 = 97;
const A_UCASE: u8 = 65;
// 11111111...26 times
const ALL_26_BITS_SET: u32 = 67108863;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letter_flags = 0;
    for letter in sentence.chars() {
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << (letter as u8 - A_LCASE);
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << (letter as u8 - A_UCASE);
        }
    }
    letter_flags == ALL_26_BITS_SET
}

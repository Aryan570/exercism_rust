// it passed, but need to refactor
use std::u64::MAX;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut reversed = 0;
        let mut tmp = value;
        while tmp > 0 {
            reversed = (reversed * 10)  +  (tmp%10);
            tmp = tmp /10;
        }
        if reversed == value {return Some(Palindrome (value));}
        None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // case 1 ---> first smallest
    // case 2 ---> first largest
    let mut small = MAX;
    for i in min..=max{
        for j in min..=max{
            if i*j < small && Palindrome::new(i*j).is_some(){
                small = i * j;
                break;
            }
        }
    }
    if small == 0 {return None;}
    let mut large = 0;
    for i in (min..=max).rev(){
        for j in (min..=max).rev(){
            if i*j > large && Palindrome::new(i*j).is_some(){
                large = i * j;
                break;
            }
        }
    }
    if large == 0 {return None}
    Some((Palindrome (small) , Palindrome (large)))
}

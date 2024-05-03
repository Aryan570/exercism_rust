pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T : ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut flag = false;
        let mut cnt = 0;
        let mut sum = 0;
        for byte in self.to_string().bytes().rev() {
            if byte != 32 && (byte < 48 || byte > 57) {
                return false;
            }
            if byte == 32 {
                continue;
            }
            let tmp = byte - '0' as u8;
            sum += tmp;
            if flag {
                sum -= tmp;
                if 2 * tmp > 9 {
                    sum += (2 * tmp) - 9;
                } else {
                    sum += 2 * tmp;
                }
            }
            flag = !flag;
            cnt += 1;
        }
        sum % 10 == 0 && cnt > 1
    }
}

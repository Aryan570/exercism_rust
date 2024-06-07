use std::collections::BTreeMap;
pub trait LBOUND {
    fn get_el(&self, num: u64) -> Option<u64>;
}
impl LBOUND for BTreeMap<u64, &str> {
    fn get_el(&self, num: u64) -> Option<u64> {
        let mut prev = None;
        for (key, _val) in self.iter() {
            if *key > num {
                return prev;
            } else if *key == num {
                return Some(*key);
            } else {
                prev = Some(*key);
            }
        }
        prev
    }
}
pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let mpp: BTreeMap<u64, &str> = BTreeMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
        (1000, "thousand"),
        (1000000, "million"),
        (1000000000, "billion"),
        (1000000000000, "trillion"),
        (1000000000000000, "quadrillion"),
        (1000000000000000000, "quintillion"),
    ]);
    let mut ans = String::new();
    let mut nn = n;
    while nn > 0 {
        let div = mpp.get_el(nn);
        match div {
            Some(x) => {
                if nn > 99 {
                    let pref = mpp.get_el(nn / x);
                    if nn / x > 10 {
                        let s = encode(nn / x);
                        ans.push_str(&s);
                        ans.push(' ');
                    } else {
                        ans.push_str(mpp.get(&pref.unwrap()).unwrap());
                        ans.push(' ');
                    }
                }
                ans.push_str(mpp.get(&x).unwrap());
                if nn > 20 && nn < 100 {
                    ans.push('-');
                    nn = nn % x;
                    continue;
                }
                ans.push(' ');
                nn = nn % x;
            }
            None => {
                unreachable!()
            }
        }
    }
    let _ = ans.pop();
    ans
}

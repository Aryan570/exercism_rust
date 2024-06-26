use std::fmt::{Display, Formatter, Result};

pub struct Roman{
    roman : String
}
const VEC : [(u32,&str);13] = [(1000,"M"),(900,"CM"),(500,"D"),(400,"CD"),(100,"C"),(90,"XC"),(50,"L"),(40,"XL"),(10,"X"),(9,"IX"),(5,"V"),(4,"IV"),(1,"I")];
impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{}",self.roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut num = num;
        let mut ans = String::new();
        for (i,ch) in VEC{
            while num >= i {
                ans.push_str(ch);
                num -= i;
            }
        }
        Roman { roman: ans }
    }
}

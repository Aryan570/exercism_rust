const VEC : [&str; 4] = ["wink","double blink","close your eyes","jump"];
pub fn actions(n: u8) -> Vec<&'static str> {
    let mut ans = Vec::new();
    let mut tmp = 1;
    for i in 0..=4{
        match n & tmp {
            0 => {
                tmp = tmp << 1;
            }
            1.. => {
                if i == 4 {
                    ans.reverse();
                    break;
                }
                ans.push(VEC[i]);
                tmp = tmp << 1;
            }
        }
    }
    ans
}
/// there's also a pretty neat solution from babahop on exercism

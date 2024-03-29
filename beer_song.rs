pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }else if n == 1{
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }else if n == 2{return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string();}
    let c = n - 1;
    return format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {c} bottles of beer on the wall.\n");
}
pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for n in (end..=start).rev(){
        s.push_str(&verse(n));
        s.push_str("\n");
    }
    s.pop();
    s
}

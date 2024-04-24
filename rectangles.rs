fn get_cnt(line_number : usize,first_idx : usize,second_idx : usize,lines: &[&str]) -> u32{
    let mut ans = 0;
    for line_idx in (line_number + 1)..lines.len(){
        match (lines[line_idx].as_bytes()[first_idx],lines[line_idx].as_bytes()[second_idx]) {
            (b'+',b'+') => {
                for check_idx in first_idx..=second_idx{
                    if lines[line_idx].as_bytes()[check_idx] == b' '{return ans;}
                }
                ans += 1;
            }
            (b'|',a) | (a,b'|') if a != b'+' && a != b'|' => return ans,
            (b' ',_) | (_,b' ') => return ans,
            _ => continue
        }
    }
    ans
}
pub fn count(lines: &[&str]) -> u32 {
    let mut ans = 0;
    let mut line_number = 0;
    // iterate over the lines
    for i in lines{
        for (idx,ch) in i.char_indices(){
            // if we get a + , we look for another one
            match ch {
                '+' => {
                    // finding the next +
                    for next_plus_idx in (idx + 1)..i.len(){
                        let c = i.as_bytes()[next_plus_idx];
                        if c != b'-' && c != b'+' {break;}
                        match c {
                            b'+' => {
                                ans += get_cnt(line_number,idx,next_plus_idx,lines);
                            }
                            _ => continue
                        }
                    }
                }
                _ => continue
            }
        }
        line_number += 1;
    }
    ans
}

fn num_neighbour(i :&i32,s_idx: &i32, minefield: &[&str],directions : &[[i32;2];8]) -> char{
    let mut cnt :u8 = 0;
    for dir in directions{
        let row = dir[0] + i;
        let col = dir[1] + s_idx;
        if row >=0 && col >=0 && col<minefield[0].len().try_into().unwrap() && row<minefield.len().try_into().unwrap(){
            let check = minefield[row.wrapping_abs() as usize].to_string();
             if check.chars().nth(col.wrapping_abs() as usize).unwrap() == '*'{
                cnt += 1;
             }
        }
    }
    if cnt > 0 {
        return (cnt+ b'0') as char;
    }
    ' '
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given minefield with the 
    // number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    // string slices -- traverse over len() and read as bytes or chars
    let mut ans = Vec::new();
    let directions = [[-1,-1],[-1,0],[-1,1],[0,1],[1,1],[1,0],[1,-1],[0,-1]];
    for i in 0..minefield.len() as i32{
        let mut s = String::new();
        let mut s_idx = 0;
        for byte in minefield[i.wrapping_abs() as usize].as_bytes(){
            if *byte == 32{
                s.push(num_neighbour(&i,&s_idx,&minefield,&directions));
            }else{
                s.push('*');
            }
            s_idx += 1;
        }
        ans.push(s);
    }
    ans
}

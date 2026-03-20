pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut tmp = vec![Vec::new(); garden.len()];
    let mut answer = Vec::new();
    let mut i = 0;
    for &s in garden {
        for byte in s.as_bytes() {
            tmp[i].push(*byte);
        }
        i += 1;
    }
    for (i, row) in tmp.iter().enumerate() {
        let mut tmp_str = String::new();
        for (j, byte) in row.iter().enumerate() {
            match byte {
                b' ' => {
                    let mut neighbours = 0;
                    for ii in -1..=1 {
                        for jj in -1..=1 {
                            let r = i as i32 + ii;
                            let c = j as i32 + jj;
                            if r >= 0
                                && r < tmp.len() as i32
                                && c >= 0
                                && c < row.len() as i32
                                && tmp[r as usize][c as usize] == b'*'
                            {
                                neighbours += 1;
                            }
                        }
                    }
                    if neighbours > 0 {
                        tmp_str.push((b'0' + neighbours) as char);
                    } else {
                        tmp_str.push(' ');
                    }
                }
                _ => {
                    tmp_str.push('*');
                }
            }
        }
        answer.push(tmp_str);
    }
    answer
}

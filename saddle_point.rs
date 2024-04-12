use std::cmp::{max,min};

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_max = vec![0; input.len()];
    let mut col_min = vec![u64::MAX; input[0].len()];
    let mut ans: Vec<(usize, usize)> = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            row_max[row] = max(row_max[row], input[row][col]);
            col_min[col] = min(col_min[col], input[row][col]);
        }
    }
    for row in 0..input.len(){
        for col in 0..input[0].len() {
            if row_max[row] == input[row][col] && col_min[col] == input[row][col]{
                ans.push((row,col));
            }
        }
    }
    ans
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut left: i32 = 0;
    let mut right: i32 = size as i32 - 1;
    let mut top: i32 = 0;
    let mut bottom: i32 = size as i32 - 1;
    let mut cnt = 1;
    let mut v = vec![vec![0; size as usize]; size as usize];
    while left <= right && top <= bottom {
        for i in left..=right {
            v[left as usize][i as usize] = cnt;
            cnt += 1;
        }
        top += 1;
        for i in top..=bottom {
            v[i as usize][right as usize] = cnt;
            cnt += 1;
        }
        right -= 1;
        for i in (left..=right).rev() {
            v[bottom as usize][i as usize] = cnt;
            cnt += 1;
        }
        bottom -= 1;
        for i in (top..=bottom).rev() {
            v[i as usize][left as usize] = cnt;
            cnt += 1;
        }
        left += 1;
    }
    v
}

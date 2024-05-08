// BRUTE FORCE
pub struct Matrix {
    v : Vec<String>
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let mut v = Vec::new();
        for l in input.lines(){
            v.push(l.to_string());
        }
        Matrix { v }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.v.len() {return None;}
        let mut ans = Vec::new();
        for row in self.v[row_no - 1].split_whitespace(){
            ans.push(row.parse().unwrap());
        }
        Some(ans)
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let mut ans = Vec::new();
        for l in &self.v{
            let mut tmp = 1;
            for num in l.as_str().split_whitespace(){
                if tmp == col_no {
                    ans.push(num.parse().unwrap());
                    break;
                }
                tmp += 1;
            }
        }
        if ans.len() == 0 {return None;}
        Some(ans)
    }
}

pub struct PascalsTriangle{
    rows : u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }
    fn factorial(num : u32)-> u32 {
        if num == 1 || num == 0 {return 1;}
        num * Self::factorial(num - 1)
    }
    fn n_c_r(row : u32)-> Vec<u32>{
        let mut v = Vec::new();
        for i in 0..=row{
            v.push(Self::factorial(row) / (Self::factorial(row - i)  * Self::factorial(i)));
        }
        v
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut ans = Vec::new();
        for row in 0..self.rows{
            ans.push(Self::n_c_r(row));
        }
        ans
    }
}

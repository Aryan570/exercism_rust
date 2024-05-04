// we'll make in linear in space , second time
pub struct RailFence{
    rails : u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }
    pub fn encode(&self, text: &str) -> String {
        let mut v: Vec<String> = vec![String::new();self.rails as usize];
        let mut idx = 0; let mut down = 1;
        for c in text.chars() {
            v[idx as usize].push(c);
            if idx == 0 {down = 1;}
            if idx == (self.rails - 1) as i32 {down = -1;}
            idx += down;
        }
        let mut ans = String::new();
        for s in v{
            ans.push_str(&s);
        }
        ans
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut v = vec![vec![' ';cipher.len()];self.rails as usize];
        let mut row = 0; let mut down = 1;
        for i in 0..cipher.len(){
            v[row as usize][i] = '*';
            if row == 0 {down = 1;}
            if row == (self.rails - 1) as i32 {down = -1;}
            row += down;
        }
        let cip : Vec<char> = cipher.chars().collect();
        let mut idx = 0;
        for i in 0..self.rails{
            for j in i as usize..cip.len(){
                if v[i as usize][j] == '*'{
                    v[i as usize][j] = cip[idx];
                    idx += 1;
                }
            }
        }
        let mut ans = String::new();
        row = 0; down = 1;
        for i in 0..cipher.len(){
            ans.push(v[row as usize][i]);
            if row == 0 {down = 1;}
            if row == (self.rails - 1) as i32 {down = -1;}
            row += down;
        }
        ans
    }
}

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x : i32,
    y : i32,
    direction : Direction
}
// const DIRECTION : [Direction;4] = [Direction::North,Direction::East,Direction::South,Direction::West];

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction{
            Direction::North =>{
                Robot {x : self.x , y : self.y , direction : Direction::East}
            }
            Direction::East =>{
                Robot {x : self.x , y : self.y , direction : Direction::South}
            }
            Direction::South =>{
                Robot {x : self.x , y : self.y , direction : Direction::West}
            }
            Direction::West =>{
                Robot {x : self.x , y : self.y , direction : Direction::North}
            }
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.direction{
            Direction::North =>{
                Robot {x : self.x , y : self.y , direction : Direction::West}
            }
            Direction::East =>{
                Robot {x : self.x , y : self.y , direction : Direction::North}
            }
            Direction::South =>{
                Robot {x : self.x , y : self.y , direction : Direction::East}
            }
            Direction::West =>{
                Robot {x : self.x , y : self.y , direction : Direction::South}
            }
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction{
            Direction::North =>{
                Robot {x : self.x , y : self.y + 1 , direction : self.direction}
            }
            Direction::East =>{
                Robot {x : self.x + 1 , y : self.y , direction : self.direction}
            }
            Direction::South =>{
                Robot {x : self.x , y : self.y - 1 , direction : self.direction}
            }
            Direction::West =>{
                Robot {x : self.x - 1 , y : self.y , direction : self.direction}
            }
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut ans = self;
        for c in instructions.chars(){
            match c {
                'R' => ans = ans.turn_right(),
                'A' => ans = ans.advance(),
                'L' => ans = ans.turn_left(),
                _ => println!("Bye")
            }
        }
        ans
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x,self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
// there's also a brilliant solution from bobahop on exercism

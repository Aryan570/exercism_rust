// first iteration
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score : Vec<u16>,
    frame_number: u8,
    prev : u8,// how many pins in prev turn
    cnt : u8,
    fill_pins : u8,
    tenth_frame_index : usize
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{
            score : Vec::new(),
            frame_number: 0,
            prev : 0,
            cnt : 0,
            fill_pins : 0,
            tenth_frame_index : 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match pins {
            pin if (self.prev + pin as u8 == 10 || pin == 10) && self.fill_pins == 0 =>{
                self.frame_number = self.frame_number + 1;
                if self.frame_number == 10 {
                    self.tenth_frame_index = self.score.len();
                    if pin == 10 {
                        self.fill_pins = 2;
                    }else{self.fill_pins = 1;}
                }
                self.prev = 0; self.cnt = 0;
            } 
            pin if self.prev + pin as u8 > 10 || pin > 10 => return Err(Error::NotEnoughPinsLeft),
            _pin if self.frame_number >= 10 && self.fill_pins == 0 => return Err(Error::GameComplete),
            _ => {
                if pins != 10 {self.prev = pins as u8;}
                if self.fill_pins == 0 {self.cnt = self.cnt + 1;}
                if self.cnt == 2 {
                    self.frame_number = self.frame_number + 1;
                    if self.frame_number >=10 {self.tenth_frame_index = self.score.len();}
                    self.prev = 0; self.cnt = 0;
                }
                if self.fill_pins != 0 {
                    self.fill_pins -=1;
                }
            }
        }
        self.score.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.fill_pins != 0 || self.score.len() == 0 || self.frame_number < 10 {return None;}
        let mut points = 0;
        let mut prev= 0;
        for i in 0..self.score.len(){
            let pt = self.score[i];
            match pt {
                10 if i == self.tenth_frame_index =>{
                    points += 10 + self.score[i+1] + self.score[i+2];
                    break;
                }
                pt if i == self.tenth_frame_index && pt + prev == 10 =>{
                    points += pt + self.score[i+1]; break;
                }
                10 => {
                    points += 10 + self.score[i+1] + self.score[i+2];
                    prev = 0;
                }
                pt if pt + prev == 10 =>{
                    points += pt + self.score[i+1];
                    prev = 0;
                }
                _ => {
                    prev = pt;
                    points += pt;
                }
            }
        }
        Some(points)
    }
}

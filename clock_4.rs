use std::fmt;
#[derive(Debug,PartialEq, Eq)]
pub struct Clock{
    hours : i32,
    minutes: i32
}
// if you implemented display triat, to_string() method is automatically applied
impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        if self.minutes < 10 && self.hours < 10 {
            return write!(f,"0{}:0{}",self.hours,self.minutes);
        }else if self.minutes < 10 {
            return write!(f,"{}:0{}",self.hours,self.minutes);
        }else if self.hours <10 {
            return write!(f,"0{}:{}",self.hours,self.minutes);
        }
        write!(f,"{}:{}",(self.hours),self.minutes)
    }
}
// apparently modulus isn't the one you think it was.
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let mut hours = hours.clone();
        let mut minutes = minutes.clone();
        hours += minutes/60;
        // work around for modulus
        hours = hours.rem_euclid(24);
        if minutes < 0 && minutes%60 != 0 {
            hours-=1;
            hours = hours.rem_euclid(24);
        }
        minutes = minutes.rem_euclid(60);
        Clock {
            hours,
            minutes
        }
    }
    pub fn add_minutes(&self, minute: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        let mut hours = self.hours.clone();
        let mut minutes = self.minutes.clone();
        minutes += minute;
        hours+= minutes / 60;
        if minutes <0 && minutes%60 != 0 {
            hours -=1;
        }
        minutes = minutes.rem_euclid(60);
        hours = hours.rem_euclid(24);
        Clock{
            hours,
            minutes
        }
    }
}

// you can also use thread_local, by that way, you don't need to have unsafe blocks
use rand::prelude::*;
use std::collections::HashSet;
use std::sync::Mutex;
pub struct Robot {
    name: String,
}
const CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
static mut VIS: Mutex<Option<HashSet<String>>> = Mutex::new(None);
unsafe fn is_there(a: &String) -> bool {
    VIS.lock().as_ref().unwrap().as_ref().unwrap().contains(a)
}
impl Robot {
    fn gen_random() -> String {
        format!(
            "{0}{1}{2}",
            CHARS[rand::thread_rng().gen_range(0..=25)],
            CHARS[rand::thread_rng().gen_range(0..=25)],
            rand::thread_rng().gen_range(100..=999)
        )
    }
    pub fn new() -> Self {
        let mut a = Self::gen_random();
        unsafe {
            if VIS.lock().unwrap().is_some() {
                loop {
                    if is_there(&a) {
                        a = Self::gen_random();
                    } else {
                        VIS.lock().as_mut().unwrap().as_mut().unwrap().insert(a.clone());
                        break;
                    }
                }
            } else {
                *VIS.lock().unwrap() = Some(HashSet::from([a.clone()]));
            }
        }
        Robot { name: a }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut a = Self::gen_random();
        loop {
            unsafe {
                if is_there(&a) {
                    a = Self::gen_random();
                } else {
                    VIS.lock().as_mut().unwrap().as_mut().unwrap().remove(self.name());
                    VIS.lock().as_mut().unwrap().as_mut().unwrap().insert(a.clone());
                    break;
                }
            }
        }
        self.name = a;
    }
}

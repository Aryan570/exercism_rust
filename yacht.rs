#[derive(Debug, Clone, Copy)]
pub enum Category {
    Ones = 1,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
use std::collections::{HashMap, HashSet};

use Category::*;
fn f(start: u8, end: u8, dice: Dice) -> u8 {
    for i in start..=end {
        if !dice.contains(&i) {
            return 0;
        }
    }
    return 30;
}
pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        var @ (Ones | Twos | Threes | Fours | Fives | Sixes) => {
            let mut cnt = 0;
            for val in dice {
                if var as u8 == val {
                    cnt += 1;
                }
            }
            return (var as u8) * cnt;
        }
        Choice => return dice.iter().sum(),
        LittleStraight => f(1, 5, dice),
        BigStraight => f(2, 6, dice),
        Yacht => {
            let st = dice.iter().collect::<HashSet<&u8>>();
            match st.len() {
                1 => return 50,
                _ => return 0,
            }
        }
        FullHouse => {
            let mut mpp = HashMap::new();
            let mut sum = 0;
            for d in dice {
                sum += d;
                mpp.entry(d).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
            if mpp.len() != 2 {return 0;}
            let mut flag2 = false; let mut flag3 = false;
            for val in mpp.values(){
                if *val == 2 {flag2 = true;}
                if *val == 3 {flag3 = true;}
            }
            if flag2 && flag3 {return sum;}
            return 0;
        }
        FourOfAKind => {
            let mut mpp = HashMap::new();
            let mut sum = 0;
            for d in dice {
                mpp.entry(d).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
            let mut flag4 = false;
            for (key,val) in mpp{
                if val >= 4 {sum = 4 * key;flag4 = true;}
            }
            if flag4 {return sum;}
            return 0;
        }
    }
}

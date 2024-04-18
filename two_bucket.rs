use std::{cmp::min, collections::HashSet, u8::MAX};
// Depth First Search
#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}
fn can_fill_water(tmp_1: i8,tmp_2: i8,buc1: u8,buc2: u8,total: u8,vis : &mut HashSet<(u8,u8)>,curr_stats : &mut BucketStats,moves : u8,one : bool) -> bool {
    if tmp_1 < 0 || tmp_2 < 0 || tmp_1 > buc1 as i8 || tmp_2 > buc2 as i8 || (one && tmp_1 == 0 && tmp_2 == buc2 as i8) || (!one && tmp_2 == 0 && tmp_1 == buc1 as i8) || vis.contains(&(tmp_1 as u8,tmp_2 as u8)) {return false;}
    if tmp_1 == total as i8 || tmp_2 == total as i8 {
        match tmp_1 == total as i8 {
            true =>{
                if curr_stats.moves > moves {
                    curr_stats.moves = moves;
                    curr_stats.goal_bucket = Bucket::One;
                    curr_stats.other_bucket = tmp_2 as u8;
                }
            }
            _ =>{
                if curr_stats.moves > moves {
                    curr_stats.moves = moves;
                    curr_stats.goal_bucket = Bucket::Two;
                    curr_stats.other_bucket = tmp_1 as u8;
                }
            }
        }
        return true;
    }
    vis.insert((tmp_1 as u8,tmp_2 as u8));
    let s_to_fi = min(tmp_1 + tmp_2, buc1 as i8);
    let c = s_to_fi - tmp_1;
    let fi_to_c = min(tmp_1 + tmp_2, buc2 as i8);
    let d = fi_to_c - tmp_2;
    can_fill_water(0, tmp_2, buc1, buc2, total, vis, curr_stats, moves + 1, one) 
    || can_fill_water(tmp_1, 0, buc1, buc2, total, vis, curr_stats, moves + 1, one)
    || can_fill_water(buc1 as i8, tmp_2, buc1, buc2, total, vis, curr_stats, moves + 1, one)
    || can_fill_water(tmp_1, buc2 as i8, buc1, buc2, total, vis, curr_stats, moves + 1, one)
    || can_fill_water(s_to_fi, tmp_2 - c, buc1, buc2, total, vis, curr_stats, moves + 1, one)
    || can_fill_water(tmp_1 - d, fi_to_c, buc1, buc2, total, vis, curr_stats, moves + 1, one)
}
/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut vis: HashSet<(u8, u8)> = HashSet::new();
    let mut curr_stats = BucketStats {moves : MAX,goal_bucket : Bucket::One , other_bucket : MAX};
    let ans = match start_bucket {
        Bucket::One => can_fill_water(capacity_1 as i8, 0, capacity_1, capacity_2, goal, &mut vis,&mut curr_stats,1,true),
        _ => can_fill_water(0, capacity_2 as i8, capacity_1, capacity_2, goal, &mut vis,&mut curr_stats,1,false)
    };
    match ans {
        false => return None,
        _ =>{
            Some(curr_stats)
        }
    }
}

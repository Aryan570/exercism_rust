fn get_str(number: u32) -> String {
    match number {
        0 => String::from("no"),
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        _ => unreachable!("Not reachable"),
    }
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut answer = String::new();
    let mut bottles = start_bottles;
    for _i in 1..=take_down {
        let str_bottle = get_str(bottles);
        let plural = match bottles > 1 {
            true => "bottles",
            false => "bottle",
        };
        let str_rem = get_str(bottles - 1).to_lowercase();
        let rem_plural = match bottles == 2 {
            true => "bottle",
            false => "bottles",
        };
        answer.push_str(format!("{str_bottle} green {plural} hanging on the wall,\n").as_str());
        answer.push_str(format!("{str_bottle} green {plural} hanging on the wall,\n").as_str());
        answer.push_str(format!("And if one green bottle should accidentally fall,\n").as_str());
        answer.push_str(format!("There'll be {str_rem} green {rem_plural} hanging on the wall.\n").as_str());
        bottles -= 1;
        if bottles > 0 {
            answer.push_str("\n");
        }
    }
    answer
}

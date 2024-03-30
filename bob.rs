//obsession with match
pub fn reply(message: &str) -> &str {
    match message {
        s if s.trim().len() > 0 && s.trim().chars().last().unwrap() == '?' => {
            let mut flag = false;
            let mut any_alpha = false;
            for ch in s.chars() {
                if ch.is_alphabetic() {
                    any_alpha = true;
                    if ch.to_ascii_uppercase() != ch {
                        flag = true;
                    }
                }
            }
            if flag == true || !any_alpha {
                return "Sure.";
            }
            return "Calm down, I know what I'm doing!";
        }
        m if m.trim() == "" => "Fine. Be that way!",
        e if e
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .to_ascii_uppercase()
            == e.trim().split_whitespace().collect::<Vec<&str>>().join("") =>
        {
            let mut flag = false;
            for ch in e.trim().chars() {
                if ch.is_alphabetic() {
                    flag = true;
                }
            }
            if flag == false {
                return "Whatever.";
            }
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}


// CLEAN CODE from Exercism expert
pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}

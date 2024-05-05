pub fn answer(command: &str) -> Option<i32> {
    let command = command.trim_end_matches('?');
    let mut words = command.split_whitespace();
    if words.next() != Some("What") || words.next() != Some("is") {
        return None;
    }

    let mut result = words.next()?.parse::<i32>().ok()?;
    while let Some(x) = words.next() {
        match x {
            "plus" => result += words.next()?.parse::<i32>().ok()?,
            "minus" => result -= words.next()?.parse::<i32>().ok()?,
            "multiplied" => {
                if words.next() != Some("by") {
                    return None;
                }
                result *= words.next()?.parse::<i32>().ok()?;
            },
            "divided" => {
                if words.next() != Some("by") {
                    return None;
                }
                result /= words.next()?.parse::<i32>().ok()?;
            },
            _ => return None,
        }
    }

    Some(result)
}


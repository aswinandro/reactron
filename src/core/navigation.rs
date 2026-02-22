pub fn step_clamped(current: Option<usize>, delta: isize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let base = current.unwrap_or(0) as isize;
    let next = (base + delta).clamp(0, len as isize - 1) as usize;
    Some(next)
}

pub fn step_wrapped(current: Option<usize>, delta: isize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let mut index = current.unwrap_or(0) as isize + delta;
    while index < 0 {
        index += len as isize;
    }
    Some((index as usize) % len)
}

pub fn find_next_contains(items: &[String], query: &str, start_after: Option<usize>) -> Option<usize> {
    if items.is_empty() || query.is_empty() {
        return None;
    }

    let needle = query.to_lowercase();
    let start = start_after.map(|index| index + 1).unwrap_or(0);

    for offset in 0..items.len() {
        let index = (start + offset) % items.len();
        if items[index].to_lowercase().contains(&needle) {
            return Some(index);
        }
    }

    None
}

pub fn find_next_prefix(items: &[String], query: &str, start_after: Option<usize>) -> Option<usize> {
    if items.is_empty() || query.is_empty() {
        return None;
    }

    let needle = query.to_lowercase();
    let start = start_after.map(|index| index + 1).unwrap_or(0);

    for offset in 0..items.len() {
        let index = (start + offset) % items.len();
        if items[index].to_lowercase().starts_with(&needle) {
            return Some(index);
        }
    }

    None
}


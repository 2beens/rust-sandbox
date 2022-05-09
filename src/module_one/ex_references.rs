pub fn get_str_len(s: &str) -> usize {
    s.len()
}

pub fn change_str(s: &mut String) {
    s.push_str(", and something more");
}

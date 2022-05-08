pub fn get_str_len(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(", and something more");
}

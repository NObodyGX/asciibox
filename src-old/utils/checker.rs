pub fn check_is_color<S: AsRef<str>>(color: S) -> bool {
    let color = color.as_ref();
    if !color.starts_with('#') {
        return false;
    }
    if !(color.len() == 7 || color.len() == 4) {
        return false;
    }
    let hex_part = &color[1..];
    for ch in hex_part.chars() {
        if !ch.is_digit(16) {
            return false;
        }
    }

    true
}

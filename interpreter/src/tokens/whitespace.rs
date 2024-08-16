use std::char;

pub const SPACE: char = ' ';
pub const TAB: char = '\u{09}';
pub const LINE_FEED: char = '\u{0A}';
pub const CARRIAGE_RETURN: char = '\u{0D}';
pub const WHITESPACE_CHARS: [char; 4] = [SPACE, TAB, LINE_FEED, CARRIAGE_RETURN];

pub fn is_whitespace(current: &char) -> bool {
    WHITESPACE_CHARS.contains(current)
}

pub fn open_line_comment(current: &char, next: Option<&char>) -> bool {
    if let Some(n) = next {
        return current == &';' && n == &';';
    }
    false
}

pub fn open_block_comment(current: &char, next: Option<&char>) -> bool {
    if let Some(n) = next {
        let is_open = current == &'(' && n == &';';
        println!("is open block {} {} {}", current, n, is_open);
        return is_open;
    }
    false
}

pub fn close_block_comment(current: &char, next: Option<&char>) -> bool {
    if let Some(n) = next {
        return current == &';' && n == &')';
    }
    false
}

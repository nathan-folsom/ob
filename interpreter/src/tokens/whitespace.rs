use std::char;

const SPACE: char = ' ';
const TAB: char = '\u{09}';
const LINE_FEED: char = '\u{0A}';
const CARRIAGE_RETURN: char = '\u{0D}';
const WHITESPACE_CHARS: [char; 4] = [SPACE, TAB, LINE_FEED, CARRIAGE_RETURN];

pub struct WhitespaceParser {
    internal: Option<CommentParser>,
}

impl WhitespaceParser {
    pub fn new() -> Self {
        Self { internal: None }
    }

    pub fn is_whitespace(&mut self, current: char, next: Option<char>) -> bool {
        if self.internal.is_none() {
            if WHITESPACE_CHARS.contains(&current) {
                return true;
            } else if let Some(p) = CommentParser::comment_started(current, next) {
                self.internal = Some(p);
                return true;
            }
        } else if let Some(p) = &mut self.internal {
            if p.comment_ended(current, next) {
                self.internal = None;
                return false;
            } else {
                return true;
            }
        }
        false
    }
}

enum CommentParser {
    Line,
    Block(i32),
}

impl CommentParser {
    fn comment_started(current: char, next: Option<char>) -> Option<Self> {
        if open_block_comment(current, next) {
            Some(Self::Block(0))
        } else if open_line_comment(current, next) {
            Some(Self::Line)
        } else {
            None
        }
    }

    fn comment_ended(&mut self, current: char, next: Option<char>) -> bool {
        match self {
            Self::Block(depth) => {
                // handle nested comments
                if open_block_comment(current, next) {
                    *depth += 1;
                    true
                } else if close_block_comment(current, next) {
                    if *depth == 0 {
                        false
                    } else {
                        *depth -= 1;
                        true
                    }
                } else {
                    true
                }
            }
            Self::Line => current != CARRIAGE_RETURN && current != LINE_FEED,
        }
    }
}

fn open_line_comment(current: char, next: Option<char>) -> bool {
    if let Some(n) = next {
        return current == ';' && n == ';';
    }
    false
}

fn open_block_comment(current: char, next: Option<char>) -> bool {
    if let Some(n) = next {
        return current == '(' && n == ';';
    }
    false
}

fn close_block_comment(current: char, next: Option<char>) -> bool {
    if let Some(n) = next {
        return current == ';' && n == ')';
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_detect_whitespace_chars() {
        let mut parser = WhitespaceParser::new();
        WHITESPACE_CHARS.iter().for_each(|c| {
            assert!(parser.is_whitespace(*c, None));
        });
    }
    #[test]
    fn should_detect_line_comment_start() {
        let mut parser = WhitespaceParser::new();
        assert!(parser.is_whitespace(';', Some(';')));
        assert_commment_started(&mut parser);
    }
    #[test]
    fn should_detect_line_comment_end() {
        let mut parser = WhitespaceParser::new();
        parser.is_whitespace(';', Some(';'));
        assert!(parser.is_whitespace(LINE_FEED, None));
        assert_commment_ended(&mut parser);
        parser.is_whitespace(';', Some(';'));
        assert!(parser.is_whitespace(CARRIAGE_RETURN, None));
        assert_commment_ended(&mut parser);
    }
    #[test]
    fn should_detect_block_comment_start() {
        let mut parser = WhitespaceParser::new();
        assert!(parser.is_whitespace('(', Some(';')));
        assert_commment_started(&mut parser);
    }
    #[test]
    fn should_detect_block_comment_end() {
        let mut parser = WhitespaceParser::new();
        parser.is_whitespace('(', Some(';'));
        assert!(parser.is_whitespace(';', Some(')')));
        assert_commment_ended(&mut parser);
    }
    #[test]
    fn should_handle_nested_block_comments() {
        let mut parser = WhitespaceParser::new();
        parser.is_whitespace('(', Some(';'));
        parser.is_whitespace('(', Some(';'));
        parser.is_whitespace(';', Some(')'));
        assert_commment_ended(&mut parser);
        parser.is_whitespace(';', Some(')'));
        assert_commment_ended(&mut parser);
    }

    fn assert_commment_started(parser: &mut WhitespaceParser) {
        assert!(parser.is_whitespace('1', None));
    }
    fn assert_commment_ended(parser: &mut WhitespaceParser) {
        assert!(!parser.is_whitespace('1', None));
    }
}

use super::{
    token::Token,
    whitespace::{
        close_block_comment, is_whitespace, open_block_comment, open_line_comment, CARRIAGE_RETURN,
        LINE_FEED,
    },
};

pub fn from_text(input: String) -> Vec<Token> {
    let mut output = vec![];
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let current = chars.get(i).unwrap();
        let next = chars.get(i + 1);
        if is_whitespace(current) {
            continue;
        }
        if open_block_comment(current, next) {
            i += 2;
            let mut depth = 0;
            while i < chars.len() {
                let current = chars.get(i).unwrap();
                let next = chars.get(i + 1);
                if open_block_comment(current, next) {
                    depth += 1;
                } else if close_block_comment(current, next) {
                    if depth == 0 {
                        break;
                    } else {
                        depth -= 1;
                    }
                }
                i += 1;
            }
            i += 2;
            continue;
        } else if open_line_comment(current, next) {
            i += 2;
            while i < chars.len() {
                let current = chars.get(i).unwrap();
                if current == &CARRIAGE_RETURN || current == &LINE_FEED {
                    break;
                }
                i += 1;
            }
            i += 1;
            continue;
        } else if current == &'(' {
            output.push(Token::OpenParenthesis);
        } else if current == &')' {
            output.push(Token::CloseParenthesis);
        }
        i += 1;
    }
    output
}

pub fn from_binary(input: Vec<u8>) -> Vec<Token> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_line_comment() {
        let output = from_text(";; line comment \n".to_string());
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_read_block_comment() {
        let output = from_text("(; block comment ;)".to_string());
        println!("{:?}", output);
        assert_eq!(output.len(), 0);
    }
}

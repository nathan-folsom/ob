use super::{token::Token, whitespace::WhitespaceParser};

pub fn from_text(input: String) -> Vec<Token> {
    let mut output = vec![];
    let mut whitespace_parser = WhitespaceParser::new();
    let mut prev = None;
    input.chars().for_each(|c| {
        if whitespace_parser.is_whitespace(c, prev) {
            return;
        }

        if c == '(' {
            output.push(Token::OpenParenthesis);
        } else if c == ')' {
            output.push(Token::CloseParenthesis);
        }
        prev = Some(c);
    });
    output
}

pub fn from_binary(input: Vec<u8>) -> Vec<Token> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_keyword() {
        todo!();
    }
}

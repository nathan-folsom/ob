use super::token::Token;

const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const HEX_CHARS: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];

pub fn is_number(current: &char) -> bool {
    ['-', '+'].contains(current) || DIGIT_CHARS.contains(current)
}

pub fn tokenize_number(i: &mut usize, chars: &[char]) -> Token {
    let mut value = vec![];
    match chars.get(*i) {
        Some('+') => {
            *i += 1;
            value.push('+');
        }
        Some('-') => {
            *i += 1;
            value.push('-');
        }
        Some(_) => (),
        None => panic!("Failed to parse number"),
    };
    let mut is_hex = false;
    if let (Some('0'), Some('x')) = (chars.get(*i), chars.get(*i + 1)) {
        *i += 2;
        value.push('0');
        value.push('x');
        is_hex = true;
    }
    let mut is_float = false;
    while *i < chars.len() {
        let current = chars.get(*i).unwrap();
        println!("current {}", current);
        if current == &'_' {
            *i += 1;
            continue;
        } else if DIGIT_CHARS.contains(current) || is_hex && HEX_CHARS.contains(current) {
            value.push(*current);
            *i += 1;
            continue;
        } else {
            break;
        }
    }
    if is_float {
        return Token::Float(value.iter().collect());
    }
    Token::Integer(value.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tokenize_decimal_integer() {
        let input = "10".to_string();
        let mut i = 0;
        let output = tokenize_number(&mut i, &input.chars().collect::<Vec<char>>());
        assert_eq!(i, 2);
        assert_eq!(output, Token::Integer("10".into()));
    }

    #[test]
    fn should_tokenize_hex_integer() {
        let input = "0x1f".to_string();
        let mut i = 0;
        let output = tokenize_number(&mut i, &input.chars().collect::<Vec<char>>());
        assert_eq!(i, 4);
        assert_eq!(output, Token::Integer("0x1f".into()));
    }

    #[test]
    fn should_tokenize_decimal_float() {
        todo!();
    }

    #[test]
    fn should_tokenize_hex_float() {
        todo!();
    }
}

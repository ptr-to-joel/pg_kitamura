use crate::token::Token;

fn tokeniser_input(input: &str) -> Vec<Token> {
    input
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| Token {
            value: s.into(),
            position: i as u32,
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn whitespace_and_commas_removed() {
        let input = "hello, I liKe uchi mata";

        let expected = vec![
            Token {
                value: "hello".into(),
                position: 0,
            },
            Token {
                value: "I".into(),
                position: 1,
            },
            Token {
                value: "liKe".into(),
                position: 2,
            },
            Token {
                value: "uchi".into(),
                position: 3,
            },
            Token {
                value: "mata".into(),
                position: 4,
            },
        ];

        assert_eq!(tokeniser_input(input), expected);
    }
}

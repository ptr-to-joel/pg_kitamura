use crate::token::Token;

fn normalise_tokens(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .iter()
        .map(|t| Token {
            value: t.value.to_lowercase(),
            position: t.position,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_become_lowercase() {
        let input = vec![
            Token {
                value: "HELLO".into(),
                position: 0,
            },
            Token {
                value: "tHIs".into(),
                position: 1,
            },
            Token {
                value: "iS".into(),
                position: 2,
            },
            Token {
                value: "A".into(),
                position: 3,
            },
            Token {
                value: "mix".into(),
                position: 4,
            },
        ];

        let expected = vec![
            Token {
                value: "hello".into(),
                position: 0,
            },
            Token {
                value: "this".into(),
                position: 1,
            },
            Token {
                value: "is".into(),
                position: 2,
            },
            Token {
                value: "a".into(),
                position: 3,
            },
            Token {
                value: "mix".into(),
                position: 4,
            },
        ];

        assert_eq!(normalise_tokens(input), expected);
    }
}

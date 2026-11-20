#[derive(PartialEq)]
pub(crate) enum Token {
    Term(String),
    And,
    Or,
    LeftParen,
    RightParen,
}

fn parse(query: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut term = String::new();

    query.chars().for_each(|c| match c.is_whitespace() {
        true => push_term(&mut tokens, &mut term),
        _ => match_token(&mut tokens, &mut term, c),
    });

    if !term.is_empty() {
        tokens.push(Token::Term(term));
    }

    tokens
}

fn push_term(tokens: &mut Vec<Token>, term: &mut String) {
    if !term.is_empty() {
        tokens.push(Token::Term(term.to_owned()));
        term.clear();
    }
}

fn match_token(tokens: &mut Vec<Token>, term: &mut String, c: char) {
    match c {
        '(' => add_token(tokens, term, Token::LeftParen),
        ')' => add_token(tokens, term, Token::RightParen),
        '&' => add_token(tokens, term, Token::And),
        '|' => add_token(tokens, term, Token::Or),
        _ => term.push(c),
    };
}

fn add_token(tokens: &mut Vec<Token>, term: &mut String, token: Token) {
    push_term(tokens, term);
    tokens.push(token);
}

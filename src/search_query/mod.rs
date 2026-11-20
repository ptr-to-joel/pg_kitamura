use crate::search_query::parser::Token;

mod parser;

#[derive(Debug, PartialEq)]
enum QueryAST {
    Term(String),
    And(Box<QueryAST>, Box<QueryAST>),
    Or(Box<QueryAST>, Box<QueryAST>),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);

        self.pos += 1;
        token
    }

    fn parse_primary(&mut self) -> QueryAST {
        match self.advance() {
            Some(Token::Term(s)) => QueryAST::Term(s.into()),
            _ => todo!(),
        }
    }
}

fn parse_init(tokens: Vec<Token>) -> QueryAST {
    let mut parser = Parser { tokens, pos: 0 };

    parse_or(&mut parser)
}

fn parse_or(parser: &mut Parser) -> QueryAST {
    let mut left = parse_and(parser);

    while matches!(parser.peek(), Some(Token::Or)) {
        parser.advance();
        let right = parse_and(parser);
        left = QueryAST::Or(Box::new(left), Box::new(right));
    }

    left
}

fn parse_and(parser: &mut Parser) -> QueryAST {
    let mut left = parser.parse_primary();

    while matches!(parser.peek(), Some(Token::And)) {
        parser.advance();
        let right = parser.parse_primary();
        left = QueryAST::And(Box::new(left), Box::new(right));
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_term() {
        let input = vec![Token::Term("postgres".into())];

        let expected = QueryAST::Term("postgres".into());

        assert_eq!(expected, parse_init(input));
    }

    #[test]
    fn two_terms_with_and() {
        let input = vec![
            Token::Term("postgres".into()),
            Token::And,
            Token::Term("rust".into()),
        ];

        let expected = QueryAST::And(
            Box::new(QueryAST::Term("postgres".into())),
            Box::new(QueryAST::Term("rust".into())),
        );

        assert_eq!(expected, parse_init(input));
    }

    #[test]
    fn two_terms_with_or() {
        let input = vec![
            Token::Term("postgres".into()),
            Token::Or,
            Token::Term("rust".into()),
        ];

        let expected = QueryAST::Or(
            Box::new(QueryAST::Term("postgres".into())),
            Box::new(QueryAST::Term("rust".into())),
        );

        assert_eq!(expected, parse_init(input));
    }

    #[test]
    fn three_terms_with_and_plus_and() {
        let input = vec![
            Token::Term("postgres".into()),
            Token::And,
            Token::Term("rust".into()),
            Token::And,
            Token::Term("java".into()),
        ];

        let expected = QueryAST::And(
            Box::new(QueryAST::And(
                Box::new(QueryAST::Term("postgres".into())),
                Box::new(QueryAST::Term("rust".into())),
            )),
            Box::new(QueryAST::Term("java".into())),
        );

        assert_eq!(expected, parse_init(input));
    }

    #[test]
    fn three_terms_with_and_plus_or() {
        let input = vec![
            Token::Term("postgres".into()),
            Token::And,
            Token::Term("rust".into()),
            Token::Or,
            Token::Term("java".into()),
        ];

        let expected = QueryAST::Or(
            Box::new(QueryAST::And(
                Box::new(QueryAST::Term("postgres".into())),
                Box::new(QueryAST::Term("rust".into())),
            )),
            Box::new(QueryAST::Term("java".into())),
        );

        assert_eq!(expected, parse_init(input));
    }

    #[test]
    fn three_terms_with_or_plus_and() {
        let input = vec![
            Token::Term("postgres".into()),
            Token::Or,
            Token::Term("rust".into()),
            Token::And,
            Token::Term("java".into()),
        ];

        let expected = QueryAST::Or(
            Box::new(QueryAST::Term("postgres".into())),
            Box::new(QueryAST::And(
                Box::new(QueryAST::Term("rust".into())),
                Box::new(QueryAST::Term("java".into())),
            )),
        );

        assert_eq!(expected, parse_init(input));
    }
}

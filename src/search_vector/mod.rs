use crate::token::Token;

#[derive(Debug, PartialEq)]
struct SearchVector {
    terms: Vec<Term>,
}

impl SearchVector {
    fn new() -> SearchVector {
        SearchVector { terms: Vec::new() }
    }

    fn add(&mut self, input: &Token) {
        match self.terms.iter_mut().find(|t| t.value == *input.value) {
            Some(token) => token.positions.push(input.position),
            None => self.terms.push(Term {
                value: input.value.to_owned(),
                positions: vec![input.position],
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Term {
    value: String,
    positions: Vec<u32>,
}

fn generate_terms(input: Vec<Token>) -> SearchVector {
    let mut sv = SearchVector::new();

    input.iter().for_each(|t| sv.add(t));

    sv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_became_search_vector() {
        let input = vec![
            Token {
                value: "do".into(),
                position: 0,
            },
            Token {
                value: "you".into(),
                position: 1,
            },
            Token {
                value: "like".into(),
                position: 2,
            },
            Token {
                value: "mango".into(),
                position: 3,
            },
            Token {
                value: "because".into(),
                position: 4,
            },
            Token {
                value: "i".into(),
                position: 5,
            },
            Token {
                value: "like".into(),
                position: 6,
            },
            Token {
                value: "mango".into(),
                position: 7,
            },
        ];

        let expected = SearchVector {
            terms: vec![
                Term {
                    value: "do".into(),
                    positions: vec![0],
                },
                Term {
                    value: "you".into(),
                    positions: vec![1],
                },
                Term {
                    value: "like".into(),
                    positions: vec![2, 6],
                },
                Term {
                    value: "mango".into(),
                    positions: vec![3, 7],
                },
                Term {
                    value: "because".into(),
                    positions: vec![4],
                },
                Term {
                    value: "i".into(),
                    positions: vec![5],
                },
            ],
        };

        assert_eq!(generate_terms(input), expected);
    }
}

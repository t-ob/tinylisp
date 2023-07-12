// AST nodes are either numbers, symbols, or lists of subexpressions
#[derive(Debug, PartialEq, Eq)]
pub enum AST {
    Number(i32),
    Symbol(String),
    List(Vec<AST>),
}

// TODO: Implement Value
pub enum Value {

}

pub fn tokenize(sexpr: &str) -> Vec<String> {
    let mut tokens = vec![];

    let mut current_token = String::new();

    // Go through each char in input string. Parentheses get their own token,
    // whitespace is ignored, and everything else is accumulated into a string.
    for c in sexpr.chars() {
        match c {
            '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(c.to_string());
            }
            c if c.is_whitespace() => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    tokens
}

fn _parse(tokens: &Vec<String>, i: usize) -> Result<(usize, AST), String> {
    // Parses tokens starting at index i, returning the AST node that was parsed, and the index to continue parsing from.
    if tokens.is_empty() {
        return Err("Cannot parse empty token list".to_string());
    }

    match &tokens[i][..] {
        // An lparen indicates the start of a subexpression, and will get parsed into a List node.
        "(" => {
            let mut subexpr = vec![];
            let mut j = i + 1;
            // Recursively consume tokens until we encounter a matching rparen.
            while &tokens[j][..] != ")" {
                let (next_j, ast) = _parse(tokens, j)?;
                subexpr.push(ast);
                j = next_j;
            }
            // Index j now points to an rparen so continue parsing from the next token.
            Ok((j + 1, AST::List(subexpr)))
        }
        // Try parsing the token as a number, otherwise parse it as a symbol.
        _ => {
            if let Ok(number) = tokens[i].parse::<i32>() {
                Ok((i + 1, AST::Number(number)))
            } else {
                Ok((i + 1, AST::Symbol(tokens[i].clone())))
            }
        }
    }
}

pub fn parse(tokens: &Vec<String>) -> Result<AST, String> {
    _parse(tokens, 0).map(|(_, ast)| ast)
}

// TODO: Implement eval
pub fn eval(ast: &AST) -> Result<Value, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("(+ 1 2)"),
            vec!["(", "+", "1", "2", ")"]
        );

        assert_eq!(
            tokenize("(first (list 1 (+ 2 3) 9))"),
            vec!["(", "first", "(", "list", "1", "(", "+", "2", "3", ")", "9", ")", ")"]
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&tokenize("(+ 1 2)")).unwrap(),
            AST::List(vec![
                AST::Symbol("+".to_string()),
                AST::Number(1),
                AST::Number(2),
            ])
        );

        assert_eq!(
            parse(&tokenize("(first (list 1 (+ 2 3) 9))")).unwrap(),
            AST::List(vec![
                AST::Symbol("first".to_string()),
                AST::List(vec![
                    AST::Symbol("list".to_string()),
                    AST::Number(1),
                    AST::List(vec![
                        AST::Symbol("+".to_string()),
                        AST::Number(2),
                        AST::Number(3),
                    ]),
                    AST::Number(9),
                ]),
            ])
        )
    }
}
use crate::lexing::token::Token;
use regex::Regex;

// //TODO: can't be global matching has to be anchored to index
// pub fn lex_program(program: &str) -> Vec<Token> {
//     let current_input = program;

//     //highest to lowest priority
//     //ADD NEW TOKENS
//     let tokens = [
//         "Print",
//         "If",
//         "Else",
//         "While",
//         "For",
//         "Fun",
//         "Class",
//         "Super",
//         "Return",
//         "This",
//         "And",
//         "Or",
//         "Null",
//         "Plus",
//         "Minus",
//         "Star",
//         "Slash",
//         "Dot",
//         "Assign",
//         "Semicolon",
//         "LeftParen",
//         "RightParen",
//         "LeftBrace",
//         "RightBrace",
//         "Bang",
//         "BangEqual",
//         "GreaterEqual",
//         "LessEqual",
//         "EqualEqual",
//         "GreaterThan",
//         "LessThan",
//         "IntegerLiteral",
//         "StringLiteral",
//         "True",
//         "False",
//         "Identifier",
//     ];

//     let mut match_vec: Vec<(&str, usize, usize)> = Vec::new();

//     for token in tokens.iter() {
//         let token_regex = Token::get_token_regex(token);
//         match token_regex {
//             Ok(t) => {
//                 let re = Regex::new(t.as_str()).unwrap();
//                 let matched = re.find_iter(current_input);

//                 let all_matches = matched.collect::<Vec<_>>();

//                 if all_matches.len() == 0 {
//                     continue;
//                 }

//                 for m in all_matches.iter() {
//                     match_vec.push((token, m.start(), m.end()));
//                 }
//             }
//             Err(e) => eprintln!("{}", e),
//         };
//     }

//     match_vec.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| (b.2 - b.1).cmp(&(a.2 - a.1))));

//     let mut token_vec: Vec<Token> = Vec::new();
//     for m in match_vec.iter() {
//         let token = Token::get_token(m.0, Some(&current_input[m.1..m.2]));
//         match token {
//             Ok(t) => token_vec.push(t),
//             Err(e) => eprintln!("{}", e),
//         }
//         //token_vec.push(Token::get_token(m.0, Some(&current_input[m.1..m.2])));
//     }

//     return token_vec;
// }

struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus),
            '/' => self.add_token(Token::Slash),
            '*' => self.add_token(Token::Star),
        }
    }
}

pub fn lex_program(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);

    while !lexer.is_at_end() {
        lexer.start = lexer.current;
        lexer.scan_token();
    }

    lexer.tokens
}

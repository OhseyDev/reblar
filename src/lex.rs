
pub enum Token {
    Identifior(String),
    Integer(i64),
    Floater(f64),
}

pub struct LexerTokens {
    tokens: Vec<Token>
}

impl LexerTokens {
    pub fn tokens(&self) -> &Vec<Token> { &self.tokens }
}

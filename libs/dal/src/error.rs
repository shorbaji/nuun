pub enum DalError {
    LexerError,
    ParserError(String),
    EvalError(String),
}


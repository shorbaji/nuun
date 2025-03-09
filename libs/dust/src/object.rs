pub enum Object {
    Boolean(bool),
    Bytevector(Vec<u8>),
    Char(char),
    Eof,
    Null,
    Number(f64),
    Pair(Box<Object>, Box<Object>),
    String(String),
    Symbol(String),
    Vector(Vec<Object>),
}


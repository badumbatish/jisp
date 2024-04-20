pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Operator(Operator),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    Not,
}

pub fn pretty_print(module: &Module) {
    println!("AST: {:#?}", module);
}

#[derive(Debug)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    FunDecl,
}

#[derive(Debug)]
pub enum Expr {
    FunCall{name: String, args: Vec<Expr>},
    Integer(i64),
    String(String),
    Unary{ op: UnaryOp, expr: Box<Expr>},
    Binary { lhs: Box<Expr>, op: BinOp, rhs: Box<Expr> },
}

#[derive(Debug)]
pub enum UnaryOp {
    Negative,
    BitNot,
    Not,
}

#[derive(Debug)]
pub enum BinOp {
    Exp,
    Mul,
    MatMul,
    Div,
    Rem,
    Add,
    Sub,
    LShift,
    RShift,
    BitAnd,
    BitXor,
    BitOr,
    In,
    NotIn,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    And,
    Or,
}

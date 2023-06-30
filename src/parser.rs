use std::fs;

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/njar.pest"]
pub struct Grammar;

use pest::pratt_parser::PrattParser;


lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(or, Left))
            .op(Op::infix(and, Left))
            .op(Op::infix(in_, Left) | Op::infix(not_in, Left) | Op::infix(eq, Left) | Op::infix(ne, Left) | Op::infix(le, Left) | Op::infix(ge, Left) | Op::infix(lt, Left) | Op::infix(gt, Left))
            .op(Op::infix(bit_or, Left))
            .op(Op::infix(bit_xor, Left))
            .op(Op::infix(bit_and, Left))
            .op(Op::infix(lshift, Left) | Op::infix(rshift, Left))
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(matmul, Left) | Op::infix(rem, Left))
            .op(Op::prefix(negative) | Op::prefix(bit_not))
            .op(Op::infix(exp, Left))
    };
}

pub mod ast {
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
}


fn parse_expr(pair: Pair<Rule>) -> ast::Expr {
    assert_eq!(Rule::expr, pair.as_rule());
    let pairs = pair.into_inner();
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::fun_call => parse_fun_call(primary),
            Rule::type_init => unimplemented!(),
            Rule::var_call => unimplemented!(),
            Rule::integer => ast::Expr::Integer(primary.as_str().trim().parse().unwrap()),
            Rule::float => unimplemented!(),
            Rule::string => ast::Expr::String(primary.as_str().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string()),
            Rule::expr => parse_expr(primary),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::exp => ast::BinOp::Exp,
                Rule::mul => ast::BinOp::Mul,
                Rule::matmul => ast::BinOp::MatMul,
                Rule::div => ast::BinOp::Div,
                Rule::rem => ast::BinOp::Rem,
                Rule::add => ast::BinOp::Add,
                Rule::sub => ast::BinOp::Sub,
                Rule::lshift => ast::BinOp::LShift,
                Rule::rshift => ast::BinOp::RShift,
                Rule::bit_and => ast::BinOp::BitAnd,
                Rule::bit_xor => ast::BinOp::BitXor,
                Rule::bit_or => ast::BinOp::BitOr,
                Rule::in_ => ast::BinOp::In,
                Rule::not_in => ast::BinOp::NotIn,
                Rule::eq => ast::BinOp::Eq,
                Rule::ne => ast::BinOp::Ne,
                Rule::le => ast::BinOp::Le,
                Rule::ge => ast::BinOp::Ge,
                Rule::lt => ast::BinOp::Lt,
                Rule::gt => ast::BinOp::Gt,
                Rule::and => ast::BinOp::And,
                Rule::or => ast::BinOp::Or,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            ast::Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::negative => ast::Expr::Unary{ op: ast::UnaryOp::Negative, expr: Box::new(rhs) },
            Rule::bit_not => ast::Expr::Unary{ op: ast::UnaryOp::BitNot, expr: Box::new(rhs)},
            Rule::not => ast::Expr::Unary{op: ast::UnaryOp::Not, expr: Box::new(rhs)},
            _ => unreachable!(),
        })
        .parse(pairs)
}


fn parse_fun_call(pair: Pair<Rule>) -> ast::Expr {
    assert_eq!(pair.as_rule(), Rule::fun_call);

    let mut rules = pair.into_inner();
    let ident = rules.next().unwrap();
    let arglist = rules.next().unwrap();

    ast::Expr::FunCall {
        name: ident.as_str().to_string(),
        args: parse_arglist(arglist)
    }
}

fn parse_arglist(pair: Pair<Rule>) -> Vec<ast::Expr> {
    assert_eq!(pair.as_rule(), Rule::arg_list);
    pair.into_inner().map(parse_expr).collect()
}


fn parse_stmt(pair: Pair<Rule>) -> ast::Stmt {
    match pair.as_rule() {
        // Rule::fun_decl => parse_fun_decl(stmt),
        Rule::expr => ast::Stmt::Expr(parse_expr(pair)),
        _ => unimplemented!(),
    }
}

fn parse_stmts(pair: Pair<Rule>) -> Vec<ast::Stmt> {
    assert_eq!(pair.as_rule(), Rule::stmts);
    pair.into_inner().map(parse_stmt).collect()
}


fn parse_module(pair: Pair<Rule>) -> ast::Module {
    let mut children = pair.into_inner();

    let stmts = children.next().unwrap();
    let _eoi = children.next().unwrap();
    assert_eq!(children.next(), None);

    ast::Module {
        stmts: parse_stmts(stmts)
    }
}


pub fn parse_file(path: &str) -> ast::Module {
    let source = fs::read_to_string(path).unwrap();
    let parse_tree = Grammar::parse(Rule::module, &source).unwrap().next().unwrap();
    let ast = parse_module(parse_tree);
    println!("{:#?}", ast);
    ast
}

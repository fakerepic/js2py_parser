pub mod ast;
pub mod expr;
pub mod func;
pub mod obj;
pub mod op;
pub mod parser;
pub mod stmt;
pub mod syntax;

pub use js_lexer::token::*;
pub use parser::*;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatementContext {
    If,
    Do,
    While,
    With,
    For,
    StatementList,
}

// impl StatementContext {
//     pub(crate) fn is_single_statement(self) -> bool {
//         self != Self::StatementList
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct PasrseContext {
//     pub r#return: bool,
// }

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// pub enum FunctionKind {
//     Declaration,
//     Expression,
//     DefaultExport,
// }
//
// impl FunctionKind {
//     pub(crate) fn is_id_required(self) -> bool {
//         matches!(self, Self::Declaration)
//     }
//
//     pub(crate) fn is_expression(self) -> bool {
//         self == Self::Expression
//     }
// }

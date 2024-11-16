use serde::Serialize;

// #![allow(non_snake_case)]
use crate::syntax::precedence::*;

/// Operators that may be used in assignment epxressions.
///
/// ## References
/// - [13.15 Assignment Operators](https://tc39.es/ecma262/#sec-assignment-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AssignmentOperator {
    Assign = 0,
    Addition = 1,
    Subtraction = 2,
    Multiplication = 3,
    Division = 4,
    Remainder = 5,
    ShiftLeft = 6,
    ShiftRight = 7,
    ShiftRightZeroFill = 8,
    BitwiseOR = 9,
    BitwiseXOR = 10,
    BitwiseAnd = 11,
    LogicalAnd = 12,
    LogicalOr = 13,
    LogicalNullish = 14,
    Exponential = 15,
}

impl AssignmentOperator {
    /// Returns `true` for '||=`, `&&=`, and `??=`.
    #[rustfmt::skip]
    pub fn is_logical(self) -> bool {
        matches!(self, Self::LogicalAnd | Self::LogicalOr | Self::LogicalNullish)
    }

    /// Returns `true` for `+=`, `-=`, `*=`, `/=`, `%=`, and `**=`.
    #[rustfmt::skip]
    pub fn is_arithmetic(self) -> bool {
        matches!(self, Self::Addition | Self::Subtraction | Self::Multiplication
                | Self::Division | Self::Remainder | Self::Exponential
        )
    }

    /// Returns `true` for `|=`, `^=`, `&=`, `<<=`, `>>=`, and `>>>=`.
    #[rustfmt::skip]
    pub fn is_bitwise(self) -> bool {
        matches!(self, Self::BitwiseOR | Self::BitwiseXOR | Self::BitwiseAnd
                | Self::ShiftLeft | Self::ShiftRight | Self::ShiftRightZeroFill
        )
    }

    /// Get the string representation of this operator.
    ///
    /// This is the same as how the operator appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assign => "=",
            Self::Addition => "+=",
            Self::Subtraction => "-=",
            Self::Multiplication => "*=",
            Self::Division => "/=",
            Self::Remainder => "%=",
            Self::ShiftLeft => "<<=",
            Self::ShiftRight => ">>=",
            Self::ShiftRightZeroFill => ">>>=",
            Self::BitwiseOR => "|=",
            Self::BitwiseXOR => "^=",
            Self::BitwiseAnd => "&=",
            Self::LogicalAnd => "&&=",
            Self::LogicalOr => "||=",
            Self::LogicalNullish => "??=",
            Self::Exponential => "**=",
        }
    }
}

/// Operators used in binary expressions. Does not include logical binary
/// operators, which are in [`LogicalOperator`].
///
/// ## References
/// - [12.10 Binary Logical Operators](https://tc39.es/ecma262/#sec-binary-logical-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum BinaryOperator {
    /// `==`
    Equality = 0,
    /// `!=`
    Inequality = 1,
    /// `===`
    StrictEquality = 2,
    /// `!==`
    StrictInequality = 3,
    /// `<`
    LessThan = 4,
    /// `<=`
    LessEqualThan = 5,
    /// `>`
    GreaterThan = 6,
    /// `>=`
    GreaterEqualThan = 7,
    /// `<<`
    ShiftLeft = 8,
    /// `>>`
    ShiftRight = 9,
    /// `>>>`
    ShiftRightZeroFill = 10,
    /// `+`
    Addition = 11,
    /// `-`
    Subtraction = 12,
    /// `*`
    Multiplication = 13,
    /// `/`
    Division = 14,
    /// `%`
    Remainder = 15,
    /// `|`
    BitwiseOR = 16,
    /// `^`
    BitwiseXOR = 17,
    /// `&`
    BitwiseAnd = 18,
    /// `in`
    In = 19,
    /// `instanceof`
    Instanceof = 20,
    /// `**`
    Exponential = 21,
}

impl BinaryOperator {
    /// Returns `true` for inequality or inequality operarors
    #[rustfmt::skip]
    pub fn is_equality(self) -> bool {
        matches!(self, Self::Equality | Self::Inequality | Self::StrictEquality | Self::StrictInequality)
    }

    /// Returns `true` for logical comparison operators
    #[rustfmt::skip]
    pub fn is_compare(self) -> bool {
        matches!(self, Self::LessThan | Self::LessEqualThan | Self::GreaterThan | Self::GreaterEqualThan)
    }

    /// Returns `true` for arithmetic operators
    #[rustfmt::skip]
    pub fn is_arithmetic(self) -> bool {
        matches!(self, Self::Addition | Self::Subtraction | Self::Multiplication
                | Self::Division | Self::Remainder | Self::Exponential)
    }

    /// Returns `true` for multiplication (`*`), division (`/`), and remainder
    /// (`%`) operators
    pub fn is_multiplicative(self) -> bool {
        matches!(
            self,
            Self::Multiplication | Self::Division | Self::Remainder
        )
    }

    /// Returns `true` for object relation operators
    pub fn is_relational(self) -> bool {
        matches!(self, Self::In | Self::Instanceof)
    }

    /// Returns `true` if this is an [`In`](BinaryOperator::In) operator.
    pub fn is_in(self) -> bool {
        matches!(self, Self::In)
    }

    /// Returns `true` for any bitwise operator
    #[rustfmt::skip]
    pub fn is_bitwise(self) -> bool {
        self.is_bitshift() || matches!(self, Self::BitwiseOR | Self::BitwiseXOR | Self::BitwiseAnd)
    }

    /// Returns `true` for any bitshift operator
    pub fn is_bitshift(self) -> bool {
        matches!(
            self,
            Self::ShiftLeft | Self::ShiftRight | Self::ShiftRightZeroFill
        )
    }

    /// Returns `true` for any numeric or string binary operator
    pub fn is_numeric_or_string_binary_operator(self) -> bool {
        self.is_arithmetic() || self.is_bitwise()
    }

    /// Returns `true` if this operator is a keyword instead of punctuation.
    pub fn is_keyword(self) -> bool {
        matches!(self, Self::In | Self::Instanceof)
    }

    /// Try to get the operator that performs the inverse comparison operation.
    /// [`None`] if this is not a comparison operator.
    pub fn compare_inverse_operator(self) -> Option<Self> {
        match self {
            Self::LessThan => Some(Self::GreaterThan),
            Self::LessEqualThan => Some(Self::GreaterEqualThan),
            Self::GreaterThan => Some(Self::LessThan),
            Self::GreaterEqualThan => Some(Self::LessEqualThan),
            _ => None,
        }
    }

    /// Try to get the operator that performs the inverse equality operation.
    /// [`None`] if this is not an equality operator.
    pub fn equality_inverse_operator(self) -> Option<Self> {
        match self {
            Self::Equality => Some(Self::Inequality),
            Self::Inequality => Some(Self::Equality),
            Self::StrictEquality => Some(Self::StrictInequality),
            Self::StrictInequality => Some(Self::StrictEquality),
            _ => None,
        }
    }

    /// The string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equality => "==",
            Self::Inequality => "!=",
            Self::StrictEquality => "===",
            Self::StrictInequality => "!==",
            Self::LessThan => "<",
            Self::LessEqualThan => "<=",
            Self::GreaterThan => ">",
            Self::GreaterEqualThan => ">=",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::ShiftRightZeroFill => ">>>",
            Self::Addition => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
            Self::Remainder => "%",
            Self::BitwiseOR => "|",
            Self::BitwiseXOR => "^",
            Self::BitwiseAnd => "&",
            Self::In => "in",
            Self::Instanceof => "instanceof",
            Self::Exponential => "**",
        }
    }

    /// Get the operator that has a lower precedence than this operator by a
    /// single level. Use [`BinaryOperator::precedence`] to get the operator
    /// with a higher precedence.
    pub fn lower_precedence(&self) -> Precedence {
        match self {
            Self::BitwiseOR => Precedence::LogicalAnd,
            Self::BitwiseXOR => Precedence::BitwiseOr,
            Self::BitwiseAnd => Precedence::BitwiseXor,
            Self::Equality | Self::Inequality | Self::StrictEquality | Self::StrictInequality => {
                Precedence::BitwiseAnd
            }
            Self::LessThan
            | Self::LessEqualThan
            | Self::GreaterThan
            | Self::GreaterEqualThan
            | Self::Instanceof
            | Self::In => Precedence::Equals,
            Self::ShiftLeft | Self::ShiftRight | Self::ShiftRightZeroFill => Precedence::Compare,
            Self::Addition | Self::Subtraction => Precedence::Shift,
            Self::Multiplication | Self::Remainder | Self::Division => Precedence::Add,
            Self::Exponential => Precedence::Multiply,
        }
    }
}

/// Logical binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LogicalOperator {
    /// `||`
    Or = 0,
    /// `&&`
    And = 1,
    /// `??`
    Coalesce = 2,
}

impl LogicalOperator {
    /// Get the string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Or => "||",
            Self::And => "&&",
            Self::Coalesce => "??",
        }
    }

    /// Get the operator that has a lower precedence than this operator by a
    /// single level. Use [`BinaryOperator::precedence`] to get the operator
    /// with a higher precedence.
    pub fn lower_precedence(&self) -> Precedence {
        match self {
            Self::Or => Precedence::NullishCoalescing,
            Self::And => Precedence::LogicalOr,
            Self::Coalesce => Precedence::Conditional,
        }
    }
}

/// Operators used in unary operators.
///
/// Does not include self-modifying operators, which are in [`UpdateOperator`].
///
/// ## References
/// - [12.5 Unary Operators](https://tc39.es/ecma262/#sec-unary-operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum UnaryOperator {
    /// `-`
    UnaryNegation = 0,
    /// `+`
    UnaryPlus = 1,
    /// `!`
    LogicalNot = 2,
    /// `~`
    BitwiseNot = 3,
    /// `typeof`
    Typeof = 4,
    /// `void`
    Void = 5,
    /// `delete`
    Delete = 6,
}

impl UnaryOperator {
    /// Returns `true` if this operator is a unary arithmetic operator.
    pub fn is_arithmetic(self) -> bool {
        matches!(self, Self::UnaryNegation | Self::UnaryPlus)
    }

    /// Returns `true` if this operator is a [`LogicalNot`].
    ///
    /// [`LogicalNot`]: UnaryOperator::LogicalNot
    pub fn is_not(self) -> bool {
        matches!(self, Self::LogicalNot)
    }

    /// Returns `true` if this operator is a bitwise operator.
    pub fn is_bitwise(self) -> bool {
        matches!(self, Self::BitwiseNot)
    }

    /// Returns `true` if this is the [`void`](UnaryOperator::Void) operator.
    pub fn is_void(self) -> bool {
        matches!(self, Self::Void)
    }

    /// Returns `true` if this operator is a keyword instead of punctuation.
    pub fn is_keyword(self) -> bool {
        matches!(self, Self::Typeof | Self::Void | Self::Delete)
    }

    /// Get the string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnaryNegation => "-",
            Self::UnaryPlus => "+",
            Self::LogicalNot => "!",
            Self::BitwiseNot => "~",
            Self::Typeof => "typeof",
            Self::Void => "void",
            Self::Delete => "delete",
        }
    }
}

/// Unary update operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum UpdateOperator {
    /// `++`
    Increment = 0,
    /// `--`
    Decrement = 1,
}

impl UpdateOperator {
    /// Get the string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Increment => "++",
            Self::Decrement => "--",
        }
    }
}

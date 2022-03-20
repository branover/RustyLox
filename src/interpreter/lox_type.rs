use super::token::TokenType;

use std::cmp::Ordering;

#[derive(Debug)]
pub enum LoxTypeError {
    IllegalOperationError,
    IllegalComparisonError(LoxType, LoxType),
}

impl std::fmt::Display for LoxTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxTypeError::IllegalOperationError => {
                write!(f,"IllegalOperationError")
            }
            LoxTypeError::IllegalComparisonError(left, right) => {
                write!(f,"IllegalComparisonError: between {:?} and {:?}", left, right)
            }
        }
    }
}

impl std::error::Error for LoxTypeError {
    fn description(&self) -> &str {
        match *self {
            LoxTypeError::IllegalOperationError => "IllegalOperationError",
            LoxTypeError::IllegalComparisonError(_,_) => "IllegalComparisonError",
        }
    }
}

#[derive(Debug,Clone)]
pub enum LoxType {
    Nil,
    Bool(bool),
    Num(f64),
    String(String)
}

impl std::fmt::Display for LoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxType::Nil => write!(f, "nil"),
            LoxType::Bool(b) => write!(f, "{}", b),
            LoxType::Num(n) => write!(f, "{}", n),
            LoxType::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::ops::Neg for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn neg(self) -> Self::Output {
        match self {
            LoxType::Num(n) => Ok(LoxType::Num(-n)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::ops::Not for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn not(self) -> Self::Output {
        match self {
            LoxType::Bool(b) => Ok(LoxType::Bool(!b)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::ops::Sub for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (LoxType::Num(n1), LoxType::Num(n2)) => Ok(LoxType::Num(n1 - n2)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::ops::Div for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (LoxType::Num(n1), LoxType::Num(n2)) => Ok(LoxType::Num(n1 / n2)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::ops::Mul for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (LoxType::Num(n1), LoxType::Num(n2)) => Ok(LoxType::Num(n1 * n2)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::ops::Add for LoxType {
    type Output = Result<Self, LoxTypeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (LoxType::Num(left), LoxType::Num(right)) => Ok(LoxType::Num(left + right)),
            (LoxType::String(left), LoxType::String(right)) => Ok(LoxType::String(left + &right)),
            _ => Err(LoxTypeError::IllegalOperationError)
        }
    }
}

impl std::cmp::PartialEq for LoxType {
    fn eq(&self, rhs: &LoxType) -> bool {
        match (self, rhs) {
            (LoxType::Num(left),LoxType::Num(right)) => left == right,
            (LoxType::String(left),LoxType::String(right)) => left == right,
            (LoxType::Bool(left),LoxType::Bool(right)) => left == right,
            (LoxType::Nil,LoxType::Nil) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for LoxType {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (LoxType::Num(left),LoxType::Num(right)) => left.partial_cmp(right),
            (LoxType::String(left),LoxType::String(right)) => left.partial_cmp(right),
            (LoxType::Bool(left),LoxType::Bool(right)) => left.partial_cmp(right),
            _ => None
        }
    }
}

impl LoxType {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxType::Nil => false,
            LoxType::Bool( b) => *b,
            _ => true
        }
    }

    pub fn determine_ordering(&self, right: &LoxType, comparison: TokenType) -> Result<LoxType, LoxTypeError> {
        if comparison == TokenType::EqualEqual {
            Ok(LoxType::Bool(self.eq(right)))
        } else if comparison == TokenType::BangEqual {
            Ok(LoxType::Bool(!self.eq(right)))
        } else if let Some(ordering) = self.partial_cmp(right) {
            let result = match comparison {
                TokenType::Greater if ordering == Ordering::Greater => true,
                TokenType::GreaterEqual if ordering == Ordering::Greater || ordering == Ordering::Equal => true,
                TokenType::Less if ordering == Ordering::Less => true,
                TokenType::LessEqual if ordering == Ordering::Less || ordering == Ordering::Equal => true,
                TokenType::BangEqual if ordering != Ordering::Equal => true,
                TokenType::EqualEqual if ordering == Ordering::Equal => true,
                _ => false
            };
            return Ok(LoxType::Bool(result))
        } else {
            Err(LoxTypeError::IllegalComparisonError(self.clone(), right.clone()))
        }
    }
}
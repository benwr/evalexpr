use crate::{Node, Operator};
use std::fmt::{Display, Error, Formatter};

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self.operator {
            Operator::RootNode => {
                for c in self.children.iter() {
                    write!(f, "{}", c)?
                }
            }
            Operator::Add => {
                write!(f, "({} + {})", self.children[0], self.children[1])?;
            }
            Operator::Sub => {
                write!(f, "({} - {})", self.children[0], self.children[1])?;
            }
            Operator::Neg => {
                write!(f, "-{}", self.children[0])?;
            }
            Operator::Mul => {
                write!(f, "({} * {})", self.children[0], self.children[1])?;
            }
            Operator::Div => {
                write!(f, "({} / {})", self.children[0], self.children[1])?;
            }
            Operator::Mod => {
                write!(f, "({} % {})", self.children[0], self.children[1])?;
            }
            Operator::Exp => {
                write!(f, "({} ^ {})", self.children[0], self.children[1])?;
            }
            Operator::Eq => {
                write!(f, "({} == {})", self.children[0], self.children[1])?;
            }
            Operator::Neq => {
                write!(f, "({} != {})", self.children[0], self.children[1])?;
            }
            Operator::Gt => {
                write!(f, "({} > {})", self.children[0], self.children[1])?;
            }
            Operator::Lt => {
                write!(f, "({} < {})", self.children[0], self.children[1])?;
            }
            Operator::Geq => {
                write!(f, "({} >= {})", self.children[0], self.children[1])?;
            }
            Operator::Leq => {
                write!(f, "({} <= {})", self.children[0], self.children[1])?;
            }
            Operator::And => {
                write!(f, "({} && {})", self.children[0], self.children[1])?;
            }
            Operator::Or => {
                write!(f, "({} || {})", self.children[0], self.children[1])?;
            }
            Operator::Not => {
                write!(f, "!{}", self.children[0])?;
            }
            Operator::Assign => {
                write!(f, "({} = {})", self.children[0], self.children[1])?;
            }
            Operator::AddAssign => {
                write!(f, "({} += {})", self.children[0], self.children[1])?;
            }
            Operator::SubAssign => {
                write!(f, "({} -= {})", self.children[0], self.children[1])?;
            }
            Operator::MulAssign => {
                write!(f, "({} *= {})", self.children[0], self.children[1])?;
            }
            Operator::DivAssign => {
                write!(f, "({} /= {})", self.children[0], self.children[1])?;
            }
            Operator::ModAssign => {
                write!(f, "({} %= {})", self.children[0], self.children[1])?;
            }
            Operator::ExpAssign => {
                write!(f, "({} ^= {})", self.children[0], self.children[1])?;
            }
            Operator::AndAssign => {
                write!(f, "({} &&= {})", self.children[0], self.children[1])?;
            }
            Operator::OrAssign => {
                write!(f, "({} ||= {})", self.children[0], self.children[1])?;
            }
            Operator::Tuple => {
                write!(f, "(")?;
                for (i, c) in self.children.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i + 1 < self.children.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
            }
            Operator::Chain => {
                write!(f, "(")?;
                for c in self.children.iter() {
                    write!(f, "{}; ", c)?;
                }
                write!(f, ")")?;
            }
            Operator::Const{value} => {
                write!(f, "{}", value)?;
            }
            Operator::VariableIdentifierWrite{identifier} => {
                write!(f, "{}", identifier)?;
            }
            Operator::VariableIdentifierRead{identifier} => {
                write!(f, "{}", identifier)?;
            }
            Operator::FunctionIdentifier{identifier} => {
                write!(f, "{} ", identifier)?;
                for (i, c) in self.children.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i + 1 < self.children.len() {
                        write!(f, " ")?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

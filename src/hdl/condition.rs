use crate::hdl::{Operand, Signal};
use std::fmt;

thread_local! {
    pub static mut LAST_CONDITION: Option<Condition> = None;
}

pub enum Conditional {
    AlwaysComb,
    Posedge(Signal),
    When(Condition),
    ElseWhen(Condition),
    Otherwise,
}

pub struct Condition {
    pub a: Box<dyn Operand>,
    pub b: Option<Box<dyn Operand>>,
    pub op: String,
}

impl Condition {
    pub fn push_last(cond: Condition) {
        unsafe {
            LAST_CONDITION = Some(cond);
        }
    }

    pub fn pop_last() -> Option<Condition> {
        unsafe {
            let last = std::mem::replace(&mut LAST_CONDITION, None);
            last
        }
    }

    pub fn signal_based(a: Signal, b: Box<dyn Operand>, op: &str) -> Condition {
        Condition {
            a: Box::new(a),
            b: Some(b),
            op: String::from(op),
        }
    }

    pub fn signal_based_unary(a: Signal, op: &str) -> Condition {
        Condition {
            a: Box::new(a),
            b: None,
            op: String::from(op),
        }
    }

    pub fn fake() -> Condition {
        let fakesig = Signal::new("NOT_DEFINED", 0);
        let fake_op = Condition::signal_based_unary(fakesig, "NOP_");
        fake_op
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl Operand for Condition {
    fn repr(&self) -> String {
        let s = match &self.b {
            Some(val) => format!("({} {} {})", &self.a.repr(), &self.op, &val.repr()),
            None => format!("({}{})", &self.op, &self.a.repr()),
        };
        return s;
    }
}

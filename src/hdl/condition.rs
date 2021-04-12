use crate::hdl::{Operand, Signal};

static mut LAST_CONDITION: Option<String> = None;

pub enum Conditional {
    AlwaysComb,
    Posedge(Signal),
    When(String),
    ElseWhen(String),
    Otherwise,
}

pub struct Condition {
    pub a: Box<dyn Operand>,
    pub b: Option<Box<dyn Operand>>,
    pub op: String,
}

impl Condition {
    pub fn push_last(cond: String) {
        unsafe {
            LAST_CONDITION = Some(cond);
        }
    }

    pub fn pop_last() -> Option<String> {
        unsafe {
            if let Some(s) = &LAST_CONDITION {
                let last = Some(s.clone());
                LAST_CONDITION = None;
                last
            }
            else {
                None
            }
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

impl Operand for Condition {
    fn repr(&self) -> String {
        let s = match &self.b {
            Some(val) => format!("({} {} {})", &self.a.repr(), &self.op, &val.repr()),
            None => format!("({}{})", &self.op, &self.a.repr()),
        };
        return s;
    }
}

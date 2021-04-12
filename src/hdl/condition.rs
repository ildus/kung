use crate::hdl::{Synth, Operand, Signal};

pub struct Condition {
    pub a: Box<dyn Operand>,
    pub b: Option<Box<dyn Operand>>,
    pub op: String,
}

impl Condition {
    pub fn signal_based<T:'static + Operand>(a: Signal, b: T, op: &str) -> Condition {
        Condition {
            a: Box::new(a),
            b: Some(Box::new(b)),
            op: String::from(op),
        }
    }

    pub fn fake() -> Condition {
        let fakesig = Signal::new("NOT_DEFINED", 0);
        let fake_op = Condition::new_unary(fakesig, "NOP_");
        fake_op
    }
}

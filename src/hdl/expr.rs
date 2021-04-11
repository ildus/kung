use crate::hdl::{Synth, Operand, Signal};
use std::ops::{Add, Sub, Shl, Shr};

pub struct Op {
    pub a: Box<dyn Operand>,
    pub b: Option<Box<dyn Operand>>,
    pub op: String,
}

pub struct Assign {
    pub op: Op,
    pub dest: Signal,
}

impl Op {
    pub fn new_unary(a: Signal, op: &str) -> Op {
        Op {
            a: Box::new(a.clone()),
            b: None,
            op: String::from(op),
        }
    }
    pub fn new<T:'static + Operand>(a: Signal, b: T, op: &str) -> Op {
        Op {
            a: Box::new(a),
            b: Some(Box::new(b)),
            op: String::from(op),
        }
    }

    pub fn new_op<T:'static + Operand>(a: Op, b: T, op: &str) -> Op {
        Op {
            a: Box::new(a),
            b: Some(Box::new(b)),
            op: String::from(op),
        }
    }

    pub fn fake() -> Op {
        let fakesig = Signal::new("NOT_DEFINED", 0);
        let fake_op = Op::new_unary(fakesig, "NOP_");
        fake_op
    }
}

impl Operand for Op {
    fn repr(&self) -> String {
        let s = match &self.b {
            Some(val) => format!("({} {} {})", &self.a.repr(), &self.op, &val.repr()),
            None => format!("({}{})", &self.op, &self.a.repr()),
        };
        return s;
    }
}

impl Assign {
    pub fn new(dest: Signal, op: Op) -> Assign {
        Assign {
            op: op,
            dest,
        }
    }
}

impl Synth for Assign {
    fn synth(&self) -> String {
        format!("assign {} = {};", &self.dest.repr(), &self.op.repr())
    }
}

impl Add<Op> for Op {
    type Output = Op;

    fn add(self, other: Op) -> Self::Output {
        return Op::new_op(self, other, "+");
    }
}

impl Add<Signal> for Op {
    type Output = Op;

    fn add(self, other: Signal) -> Self::Output {
        return Op::new_op(self, other, "+");
    }
}

impl Sub<Op> for Op {
    type Output = Op;

    fn sub(self, other: Op) -> Self::Output {
        return Op::new_op(self, other, "-");
    }
}

impl Sub<Signal> for Op {
    type Output = Op;

    fn sub(self, other: Signal) -> Self::Output {
        return Op::new_op(self, other, "-");
    }
}

impl Shl<Op> for Op {
    type Output = Op;

    fn shl(self, other: Op) -> Self::Output {
        return Op::new_op(self, other, "<<");
    }
}

impl Shr<Op> for Op {
    type Output = Op;

    fn shr(self, other: Op) -> Self::Output {
        return Op::new_op(self, other, ">>");
    }
}

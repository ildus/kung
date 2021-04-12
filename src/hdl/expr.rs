use crate::hdl::{Synth, Operand, Signal};
use std::ops::{Add, Sub, Shl, Shr, Mul};
use duplicate::duplicate;

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

    pub fn op_based<T:'static + Operand>(a: Op, b: T, op: &str) -> Op {
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

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Add<tt> for Op {
    type Output = Op;

    fn add(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "+");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Sub<tt> for Op {
    type Output = Op;

    fn sub(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "-");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Mul<tt> for Op {
    type Output = Op;

    fn mul(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "*");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Shl<tt> for Op {
    type Output = Op;

    fn shl(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "<<");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Shr<tt> for Op {
    type Output = Op;

    fn shr(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, ">>");
    }
}

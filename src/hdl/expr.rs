use crate::hdl::{Operand, Signal, condition::Condition};
use std::ops::{Add, Sub, Shl, Shr, Mul, Div, BitAnd, BitOr, BitXor, Not};
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
    pub fn new_unary(a: Signal, op: &str) -> Self {
        Self {
            a: Box::new(a.clone()),
            b: None,
            op: String::from(op),
        }
    }
    pub fn new<T:'static + Operand>(a: Signal, b: T, op: &str) -> Self {
        Self {
            a: Box::new(a),
            b: Some(Box::new(b)),
            op: String::from(op),
        }
    }

    pub fn op_based<T:'static + Operand>(a: Self, b: T, op: &str) -> Self {
        Self {
            a: Box::new(a),
            b: Some(Box::new(b)),
            op: String::from(op),
        }
    }

    pub fn op_based_unary(a: Self, op: &str) -> Self {
        Self {
            a: Box::new(a),
            b: None,
            op: String::from(op),
        }
    }

    pub fn fake() -> Self {
        let fakesig = Signal::new("NOT_DEFINED", 0);
        let fake_op = Self::new_unary(fakesig, "NOP_");
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

impl Assign {
    pub fn synth(&self, nonblocking: bool) -> String {
        let assign_op = if nonblocking { "<=" } else { "=" };
        format!("{} {} {};", &self.dest.repr(), assign_op, &self.op.repr())
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
impl Div<tt> for Op {
    type Output = Op;

    fn div(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "/");
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

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitAnd<tt> for Op {
    type Output = Op;

    fn bitand(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "&");
    }
}

impl Not for Op {
    type Output = Op;

    fn not(self) -> Self::Output {
        return Op::op_based_unary(self, "~");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitOr<tt> for Op {
    type Output = Op;

    fn bitor(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "|");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitXor<tt> for Op {
    type Output = Op;

    fn bitxor(self, other: tt) -> Self::Output {
        return Op::op_based(self, other, "^");
    }
}

impl From<bool> for Op {
    fn from(item: bool) -> Self {
        let last_condition = Condition::pop_last();
        if let Some(cond) = last_condition {
            // do nothing
            Op {
                a: Box::new(cond),
                b: None,
                op: String::from(""),
            }
        } else {
            Op {
                a: Box::new(item as u32),
                b: None,
                op: String::from(""),
            }
        }
    }
}

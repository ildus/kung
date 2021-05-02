use crate::hdl::{Operand, Signal};
//use std::ops::{Add, Sub, Shl, Shr, Mul, Div, BitAnd, BitOr, BitXor, Not};
//use duplicate::duplicate;

pub struct Op<'module> {
    pub a: &'module dyn Operand,
    pub b: Option<&'module dyn Operand>,
    pub op: String,
}

pub struct Assign<'module> {
    pub op: Op<'module>,
    pub dest: &'module Signal<'module>,
}

impl<'module> Op<'module> {
    pub fn new_unary(a: &'module Signal<'module>, op: &str) -> Self {
        Self {
            a: a,
            b: None,
            op: String::from(op),
        }
    }
    pub fn new(a: &'module dyn Operand, b: &'module dyn Operand, op: &str) -> Self {
        Self {
            a: a,
            b: Some(b),
            op: String::from(op),
        }
    }
}

impl Operand for Op<'_> {
    fn repr(&self) -> String {
        let s = match &self.b {
            Some(val) => format!("({} {} {})", &self.a.repr(), &self.op, &val.repr()),
            None => format!("({}{})", &self.op, &self.a.repr()),
        };
        return s;
    }
}

impl<'module> Assign<'module> {
    pub fn new(dest: &'module Signal, op: Op<'module>) -> Self {
        Assign {
            op: op,
            dest,
        }
    }

    pub fn synth(&self, nonblocking: bool) -> String {
        let assign_op = if nonblocking { "<=" } else { "=" };
        format!("{} {} {};", &self.dest.repr(), assign_op, &self.op.repr())
    }
}

/*
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
*/

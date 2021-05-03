use std::ops::{Add, Sub, Shl, Shr, Mul, Div, BitAnd, BitOr, BitXor, Not};
use arraystring::{ArrayString, typenum::U64};
use super::{Operand, Module};
use super::expr::{Op};
//use super::condition::{Condition};
use duplicate::duplicate;
//use std::cmp::Ordering;

type SignalName = ArrayString<U64>;

#[derive(Copy, Clone)]
pub struct Signal<'module> {
    name: SignalName,
    width: u32,

    pub module: Option<&'module Module<'module>>,
}

impl Signal<'_> {
    pub fn new(name: &str, width: u32) -> Self {
        let name = SignalName::try_from_str(name).expect("expected valid name");

        return Signal{
            name: name,
            width,
            module: None,
        }
    }

    pub fn bool(name: &str) -> Self {
        return Signal::new(name, 1);
    }

    pub fn def(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("logic [{}:0] {}", &(self.width - 1).to_string(), &self.name));
        return s;
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Operand for Signal<'_> {
    fn repr(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{}", &self.name));
        return s;
    }
}

#[duplicate(tt; [Signal<'module>]; [Op<'module>]; [u32]; [i32])]
impl<'module> Add<tt> for Signal<'module> {
    type Output = Op<'module>;

    fn add(self, other: tt) -> Self::Output {
        return Op::new(&self.clone(), other, "+");
    }
}

/*
#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitAnd<tt> for Signal {
    type Output = Op;

    fn bitand(self, other: tt) -> Self::Output {
        return Op::new(self, other, "&");
    }
}

impl Not for Signal {
    type Output = Op;

    fn not(self) -> Self::Output {
        return Op::new_unary(self, "~");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitOr<tt> for Signal {
    type Output = Op;

    fn bitor(self, other: tt) -> Self::Output {
        return Op::new(self, other, "|");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl BitXor<tt> for Signal {
    type Output = Op;

    fn bitxor(self, other: tt) -> Self::Output {
        return Op::new(self, other, "^");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Sub<tt> for Signal {
    type Output = Op;

    fn sub(self, other: tt) -> Self::Output {
        return Op::new(self, other, "-");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Mul<tt> for Signal {
    type Output = Op;

    fn mul(self, other: tt) -> Self::Output {
        return Op::new(self, other, "*");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Div<tt> for Signal {
    type Output = Op;

    fn div(self, other: tt) -> Self::Output {
        return Op::new(self, other, "/");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Shl<tt> for Signal {
    type Output = Op;

    fn shl(self, other: tt) -> Self::Output {
        return Op::new(self, other, "<<");
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Shr<tt> for Signal {
    type Output = Op;

    fn shr(self, other: tt) -> Self::Output {
        return Op::new(self, other, ">>");
    }
}

impl Signal {
    pub fn cmp(&self, other: Box<dyn Operand>, op: &str) -> bool {
        let cond = Condition::signal_based(*self, other, op);
        Condition::push_last(cond);
        return false
    }
}

#[duplicate(tt; [Signal]; [u32]; [i32])]
impl PartialEq<tt> for Signal {
    fn eq(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), "==")
    }

    fn ne(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), "!=")
    }
}

#[duplicate(tt; [Signal]; [u32]; [i32])]
impl PartialOrd<tt> for Signal {
    fn partial_cmp(&self, _other: &tt) -> Option<Ordering> {
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), "<")
    }
    fn le(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), "<=")
    }
    fn gt(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), ">")
    }
    fn ge(&self, other: &tt) -> bool {
        return Signal::cmp(self, Box::new(*other), ">=")
    }
}
*/

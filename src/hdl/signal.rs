use std::ops::{Add, Sub, Shl, Shr, Mul};
use arraystring::{ArrayString, typenum::U64};
use super::Operand;
use super::expr::{Op};
use duplicate::duplicate;

type SignalName = ArrayString<U64>;

#[derive(Copy, Clone)]
pub struct Signal {
    name: SignalName,
    width: u32,
}

impl Signal {
    pub fn new(name: &str, width: u32) -> Self {
        let name = SignalName::try_from_str(name).expect("expected valid name");

        return Signal{
            name: name,
            width,
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

impl Operand for Signal {
    fn repr(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{}", &self.name));
        return s;
    }
}

#[duplicate(tt; [Signal]; [Op]; [u32]; [i32])]
impl Add<tt> for Signal {
    type Output = Op;

    fn add(self, other: tt) -> Self::Output {
        return Op::new(self, other, "+");
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

impl PartialEq<u32> for Signal {
    fn eq(&self, _other: &u32) -> bool {
        return false
    }
}

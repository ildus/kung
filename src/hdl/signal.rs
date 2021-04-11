use std::ops::{Add, Sub, Shl, Shr};
use arraystring::{ArrayString, typenum::U64};
use super::Operand;
use super::expr::{Op};

type SignalName = ArrayString<U64>;

#[derive(Copy, Clone)]
pub struct Signal {
    name: SignalName,
    width: u32,
}

impl Signal {
    pub fn new(name: &str, width: u32) -> Signal {
        let name = SignalName::try_from_str(name).expect("expected valid name");

        return Signal{
            name: name,
            width,
        }
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

impl Add<Signal> for Signal {
    type Output = Op;

    fn add(self, other: Self) -> Self::Output {
        return Op::new(self, other, "+");
    }
}

impl Add<Op> for Signal {
    type Output = Op;

    fn add(self, other: Op) -> Self::Output {
        return Op::new(self, other, "+");
    }
}

impl Add<u32> for Signal {
    type Output = Op;

    fn add(self, other: u32) -> Self::Output {
        return Op::new(self, other, "+");
    }
}

impl Add<i32> for Signal {
    type Output = Op;

    fn add(self, other: i32) -> Self::Output {
        return Op::new(self, other, "+");
    }
}

impl Sub<Signal> for Signal {
    type Output = Op;

    fn sub(self, other: Self) -> Self::Output {
        return Op::new(self, other, "-");
    }
}

impl Sub<u32> for Signal {
    type Output = Op;

    fn sub(self, other: u32) -> Self::Output {
        return Op::new(self, other, "-");
    }
}

impl Sub<i32> for Signal {
    type Output = Op;

    fn sub(self, other: i32) -> Self::Output {
        return Op::new(self, other, "-");
    }
}

impl Shl<u32> for Signal {
    type Output = Op;

    fn shl(self, other: u32) -> Self::Output {
        return Op::new(self, other, "<<");
    }
}

impl Shl<Signal> for Signal {
    type Output = Op;

    fn shl(self, other: Signal) -> Self::Output {
        return Op::new(self, other, "<<");
    }
}

impl Shr<u32> for Signal {
    type Output = Op;

    fn shr(self, other: u32) -> Self::Output {
        return Op::new(self, other, ">>");
    }
}

impl Shr<Signal> for Signal {
    type Output = Op;

    fn shr(self, other: Signal) -> Self::Output {
        return Op::new(self, other, ">>");
    }
}

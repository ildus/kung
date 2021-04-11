use std::ops::Add;
use std::ops::Shl;
use arraystring::{ArrayString, typenum::U64};
use super::Operand;
use super::expr::{Op, Assign};

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

impl Add for Signal {
    type Output = Op;

    fn add(self, other: Self) -> Self::Output {
        return Op::new(self, other, "+");
    }
}

impl Shl<Op> for Signal {
    type Output = Assign;

    fn shl(self, other: Op) -> Self::Output {
        return Assign::new(self, other);
    }
}

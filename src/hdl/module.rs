use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Index;
use std::ops::IndexMut;

use super::{Synth, Signal};
use super::expr::{Assign, Op};

pub struct Module {
    name: String,
    items: Vec<Box<dyn Synth>>,
    assigns: Vec<Assign>,
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,
}

impl Synth for Module {
    fn synth(&self) -> String {
        let mut s = String::new();

        s.push_str("module ");
        s.push_str(&self.name);
        s.push_str("();\n");

        for item in self.inputs.iter() {
            s.push_str("input ");
            s.push_str(&item.def());
            s.push_str(";\n");
        }

        for item in self.outputs.iter() {
            s.push_str("output ");
            s.push_str(&item.def());
            s.push_str(";\n");
        }

        for assign in self.assigns.iter() {
            s.push_str(&assign.synth());
            s.push_str("\n");
        }

        for item in self.items.iter() {
            s.push_str(&item.synth());
            s.push_str("\n");
        }
        s.push_str("endmodule");
        s
    }
}

impl Module {
    pub fn new(name: &str) -> Module {
        return Module {
            name: String::from(name),
            items: vec![],
            inputs: vec![],
            outputs: vec![],
            assigns: vec![],
        }
    }
}


impl AddAssign<Assign> for Module {
    fn add_assign(&mut self, other: Assign) {
        self.assigns.push(other);
    }
}

impl AddAssign<Signal> for Module {
    fn add_assign(&mut self, other: Signal) {
        self.inputs.push(other.clone());
    }
}

impl SubAssign<Signal> for Module {
    fn sub_assign(&mut self, other: Signal) {
        self.outputs.push(other.clone());
    }
}

impl Index<Signal> for Module {
    type Output = Op;

    fn index(&self, index: Signal) -> &Self::Output {
        for assign in self.assigns.iter() {
            if assign.dest.name() == index.name() {
                return &assign.op
            }
        }
        return &self.assigns[0].op
    }
}

impl IndexMut<Signal> for Module {
    fn index_mut(&mut self, signal: Signal) -> &mut Self::Output {
        let sigfake = Signal::new("fake", 0);
        let mut assign = Assign::new(signal, Op{a: Box::new(sigfake), b: Box::new(sigfake), op: String::from("NOP")});
        self.items.push(Box::new(assign));
        &mut assign.op
    }
}

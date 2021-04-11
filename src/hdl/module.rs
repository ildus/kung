use std::ops::{AddAssign, SubAssign, Index, IndexMut};
use super::{Synth, Signal, Operand};
use super::expr::{Assign, Op};

pub struct Scope {
    sync_on: Option<Signal>,
    cond: Option<Op>,
}

pub struct Module {
    name: String,
    items: Vec<Box<dyn Synth>>,
    assigns: Vec<Assign>,
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,

    assign_signal: Option<Signal>,
    assign_op: Op,
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

        if let Some(sig) = self.assign_signal {
            s.push_str(&format!("assign {} = {};", &sig.repr(), &self.assign_op.repr()));
            s.push_str("\n");
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

            assign_signal: None,
            assign_op: Op::fake(),
        }
    }

    pub fn comb(&self) -> Scope {
        return Scope {
            sync_on: None,
            cond: None,
        }
    }

    pub fn on(&self, signal: Signal) -> Scope {
        return Scope {
            sync_on: Some(signal),
            cond: None,
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
        for item in self.assigns.iter() {
            if index.name() == item.dest.name() {
                return &item.op
            }
        }
        panic!("could not find any assign")
    }
}

use std::ptr::{read, write};

impl IndexMut<Signal> for Module {
    fn index_mut(&mut self, signal: Signal) -> &mut Self::Output {
        unsafe {
            let ptr = read(&self.assign_op);
            if let Some(sig) = self.assign_signal {
                let assign = Assign::new(sig, ptr);
                self.assigns.push(assign);
            }
            write(&mut self.assign_op, Op::fake());
        }

        self.assign_signal = Some(signal);
        &mut self.assign_op
    }
}

impl Scope {
    pub fn when<T>(&mut self, condition: Op, rules: T) -> () where T: Fn() -> () {
        self.cond = Some(condition);
        rules();
    }
}

impl Synth for Scope {
    fn synth(&self) -> String {
        let mut s = String::new();
        if let Some(signal) = self.sync_on {
            s.push_str("always @(posedge ");
            s.push_str(signal.name());
            s.push_str(") begin\n");
            s.push_str("end\n");
        } else {
            s.push_str("always_comb begin\n");
            s.push_str("end\n");
        }
        s
    }
}

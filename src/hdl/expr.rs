use crate::hdl::Synth;
use crate::hdl::Operand;
use crate::hdl::Signal;

pub struct Op {
    pub a: Box<dyn Operand>,
    pub b: Box<dyn Operand>,
    pub op: String,
}

pub struct Assign {
    pub op: Box<dyn Operand>,
    pub dest: Signal,
}

impl Op {
    pub fn new(a: Signal, b: Signal, op: &str) -> Op {
        Op {
            a: Box::new(a.clone()),
            b: Box::new(b.clone()),
            op: String::from(op),
        }
    }
}

impl Operand for Op {
    fn repr(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("({} {} {})", &self.a.repr(), &self.op, &self.b.repr()));
        return s;
    }
}

impl Assign {
    pub fn new(dest: Signal, op: Op) -> Assign {
        Assign {
            op: Box::new(op),
            dest,
        }
    }
}

impl Synth for Assign {
    fn synth(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("assign {} = {};", &self.dest.repr(), &self.op.repr()));
        return s;
    }
}

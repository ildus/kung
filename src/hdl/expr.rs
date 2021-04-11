use crate::hdl::Synth;
use crate::hdl::Operand;
use crate::hdl::Signal;

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

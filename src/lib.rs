#[cfg(test)]

mod tests {
    use crate::hdl::*;

    #[test]
    fn it_works() {
        let mut m = Module::new("adder");
        let a = Signal::new("a", 32);
        let b = Signal::new("b", 32);
        let o = Signal::new("o", 32);

        m += &a;
        m += &b;
        m += &o << &a + &b;
        m -= &o;
        assert_eq!(m.synth(), "module adder();\ninput logic [31:0] a;\ninput logic [31:0] b;\noutput logic [31:0] o;\nassign o = (a + b);\nendmodule");
    }
}

pub mod hdl {
    use std::ops::Add;
    use std::ops::Shl;
    use std::ops::AddAssign;
    use std::ops::SubAssign;

    pub trait Synth {
        fn synth(&self) -> String;
    }

    pub trait Operand {
        fn repr(&self) -> String;
    }

    pub struct Module {
        name: String,
        items: Vec<Box<dyn Synth>>,
        inputs: Vec<Signal>,
        outputs: Vec<Signal>,
    }

    #[derive(Clone)]
    pub struct Signal {
        name: String,
        width: u32,
    }

    pub struct Op {
        a: Box<dyn Operand>,
        b: Box<dyn Operand>,
        op: String,
    }

    pub struct Assign {
        op: Box<dyn Operand>,
        dest: Box<dyn Operand>,
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
            }
        }
    }

    impl Signal {
        pub fn new(name: &str, width: u32) -> Signal {
            return Signal{
                name: String::from(name),
                width,
            }
        }

        fn def(&self) -> String {
            let mut s = String::new();
            s.push_str(&format!("logic [{}:0] {}", &(self.width - 1).to_string(), &self.name));
            return s;
        }
    }

    impl Operand for Signal {
        fn repr(&self) -> String {
            let mut s = String::new();
            s.push_str(&format!("{}", &self.name));
            return s;
        }
    }

    impl Add for &Signal {
        type Output = Op;

        fn add(self, other: Self) -> Self::Output {
            return Op::new(self, other, "+");
        }
    }

    impl Shl<Op> for &Signal {
        type Output = Assign;

        fn shl(self, other: Op) -> Self::Output {
            return Assign::new(self, other);
        }
    }

    impl Op {
        fn new(a: &Signal, b: &Signal, op: &str) -> Op {
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
        fn new(dest: &Signal, op: Op) -> Assign {
            Assign {
                op: Box::new(op),
                dest: Box::new(dest.clone()),
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

    impl AddAssign<Assign> for Module {
        fn add_assign(&mut self, other: Assign) {
            self.items.push(Box::new(other));
        }
    }

    impl AddAssign<&Signal> for Module {
        fn add_assign(&mut self, other: &Signal) {
            self.inputs.push(other.clone());
        }
    }

    impl SubAssign<&Signal> for Module {
        fn sub_assign(&mut self, other: &Signal) {
            self.outputs.push(other.clone());
        }
    }
}

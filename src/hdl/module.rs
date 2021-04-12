use std::ops::{AddAssign, SubAssign, Index, IndexMut};
use super::{Synth, Signal, Operand};
use super::expr::{Assign, Op};
use super::condition::{Condition, Conditional, Conditional::*};
use std::ptr::{read, write};
use duplicate::duplicate;

pub struct Scope {
    cond: Conditional,

    assigns: Vec<Assign>,
    assign_signal: Option<Signal>,
    assign_op: Op,

    sync: bool,
    scopes: Vec<Scope>,
}

pub struct Module {
    name: String,
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,

    assigns: Vec<Assign>,
    assign_signal: Option<Signal>,
    assign_op: Op,

    scopes: Vec<Scope>,
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
            s.push_str("assign ");
            s.push_str(&assign.synth(false));
            s.push_str("\n");
        }
        for scope in self.scopes.iter() {
            s.push_str("\n");
            s.push_str(&scope.synth());
            s.push_str("\n");
        }
        s.push_str("endmodule\n");
        s
    }
}

impl Module {
    pub fn new(name: &str) -> Module {
        return Module {
            name: String::from(name),
            inputs: vec![],
            outputs: vec![],
            scopes: vec![],

            assigns: vec![],
            assign_signal: None,
            assign_op: Op::fake(),
        }
    }

    pub fn comb<T>(&mut self, add_rules: T) where T:Fn(&mut Scope) -> () {
        let mut scope = Scope::new();
        add_rules(&mut scope);
        self.scopes.push(scope);
    }

    pub fn on<T>(&mut self, signal: Signal, add_rules: T) where T:Fn(&mut Scope) -> () {
        let mut scope = Scope::new();
        scope.cond = Posedge(signal);
        scope.sync = true;
        add_rules(&mut scope);
        self.scopes.push(scope);
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

#[duplicate(tt; [Module]; [Scope])]
impl Index<Signal> for tt {
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

#[duplicate(tt; [Module]; [Scope])]
impl IndexMut<Signal> for tt {
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
    pub fn new() -> Self {
        Scope {
            cond: AlwaysComb,
            scopes: vec![],

            assigns: vec![],
            assign_signal: None,
            assign_op: Op::fake(),

            sync: false,
        }
    }

    pub fn when<T>(&mut self, _stub: bool, add_rules: T) -> &mut Self where T: Fn(&mut Scope) -> () {
        let mut scope = Scope::new();

        let last_condition = Condition::pop_last();
        if let Some(cond) = last_condition {
            scope.cond = When(cond);
            scope.sync = self.sync;
        } else {
            panic!("`when` rule expects a proper condition, got none");
        }

        add_rules(&mut scope);
        self.scopes.push(scope);
        return self
    }

    pub fn elsewhen<T>(&mut self, _stub: bool, add_rules: T) -> &mut Self where T: Fn(&mut Scope) -> () {
        let mut scope = Scope::new();

        let last_condition = Condition::pop_last();
        if let Some(cond) = last_condition {
            scope.cond = ElseWhen(cond);
            scope.sync = self.sync;
        } else {
            panic!("`elsewhen` rule expects a proper condition, got none");
        }

        add_rules(&mut scope);
        self.scopes.push(scope);
        return self
    }

    pub fn otherwise<T>(&mut self, add_rules: T) -> () where T: Fn(&mut Scope) -> () {
        let mut scope = Scope::new();
        scope.cond = Otherwise;
        scope.sync = self.sync;

        add_rules(&mut scope);
        self.scopes.push(scope);
    }

    fn statements(&self, sync: bool) -> String {
        let mut s = String::new();
        let assign_op = if sync { "<=" } else { "=" };

        if let Some(sig) = self.assign_signal {
            s.push_str(&format!("{} {} {};", &sig.repr(), assign_op, &self.assign_op.repr()));
            s.push_str("\n");
        }

        for assign in self.assigns.iter() {
            s.push_str(&assign.synth(sync));
            s.push_str("\n");
        }

        for scope in self.scopes.iter() {
            s.push_str(&scope.synth());
        }
        return s;
    }
}

impl Synth for Scope {
    fn synth(&self) -> String {
        let mut s = String::new();

        match &self.cond {
            Posedge(signal) => {
                s.push_str("always_ff @(posedge ");
                s.push_str(&signal.name());
                s.push_str(") begin\n");
            },
            When(cond) => {
                s.push_str(&format!("if ({}) begin\n", cond));
            },
            ElseWhen(cond) => {
                s.push_str(&format!("else if ({}) begin\n", cond));
            },
            Otherwise => {
                s.push_str("else begin\n");
            },
            AlwaysComb => {
                s.push_str("always_comb begin\n");
            }
        }
        s.push_str(&self.statements(self.sync));
        s.push_str("end\n");
        s
    }
}

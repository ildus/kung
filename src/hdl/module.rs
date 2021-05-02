//use std::ops::{AddAssign, SubAssign, Index, IndexMut};
use std::ops::{AddAssign, SubAssign};
use super::{Synth, Signal, Operand};
use super::expr::{Assign, Op};
use super::condition::{Conditional, Conditional::*};
//use std::ptr::{read, write};
//use duplicate::duplicate;
use std::collections::BTreeMap;

pub struct Scope<'module> {
    cond: Conditional<'module>,

    assigns: BTreeMap<String, Assign<'module>>,
    assign_signal: Option<Signal<'module>>,
    assign_op: Option<Op<'module>>,

    sync: bool,
    scopes: Vec<Scope<'module>>,
}

pub struct Module<'module> {
    name: String,
    inputs: BTreeMap<String, Signal<'module>>,
    outputs: BTreeMap<String, Signal<'module>>,

    assigns: BTreeMap<String, Assign<'module>>,
    assign_signal: Option<Signal<'module>>,
    assign_op: Option<Op<'module>>,

    scopes: Vec<Scope<'module>>,
}

impl<'module> Synth for Module<'module> {
    fn synth(&self) -> String {
        let mut s = String::new();

        s.push_str("module ");
        s.push_str(&self.name);
        s.push_str("();\n");

        for (_, item) in &self.inputs {
            s.push_str("input ");
            s.push_str(&item.def());
            s.push_str(";\n");
        }

        for (_, item) in &self.outputs {
            s.push_str("output ");
            s.push_str(&item.def());
            s.push_str(";\n");
        }

        if let Some(sig) = self.assign_signal {
            s.push_str(&format!("assign {} = {};", &sig.repr(), &self.assign_op.as_ref().unwrap().repr()));
            s.push_str("\n");
        }

        for (_, assign) in &self.assigns {
            s.push_str("assign ");
            s.push_str(&assign.synth(false));
            s.push_str("\n");
        }
        for scope in &self.scopes {
            s.push_str("\n");
            s.push_str(&scope.synth());
            s.push_str("\n");
        }
        s.push_str("endmodule\n");
        s
    }
}

impl<'module> Module<'module> {
    pub fn new(name: &str) -> Module {
        return Module {
            name: String::from(name),
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
            scopes: vec![],

            assigns: BTreeMap::new(),
            assign_signal: None,
            assign_op: None,
        }
    }

    pub fn comb<T>(&mut self, add_rules: T) where T:Fn(&mut Scope) -> () {
        let mut scope = Scope::new();
        add_rules(&mut scope);
        self.scopes.push(scope);
    }

    pub fn on<T>(&mut self, signal: &'module Signal<'module>, add_rules: T) where T:Fn(&mut Scope) -> () {
        let mut scope = Scope::new();
        scope.cond = Posedge(signal);
        scope.sync = true;
        add_rules(&mut scope);
        self.scopes.push(scope);
    }
}


impl<'module> AddAssign<Assign<'module>> for Module<'module> {
    fn add_assign(&mut self, other: Assign<'module>) {
        if let None = self.assigns.get(other.dest.name()) {
            self.assigns.insert(String::from(other.dest.name()), other);
        } else {
            panic!("assign with destination '{}' already defined in this module", other.dest.name());
        }
    }
}

impl<'module> AddAssign<Signal<'module>> for Module<'module> {
    fn add_assign(&mut self, other: Signal<'module>) {
        if let None = self.inputs.get(other.name()) {
            self.inputs.insert(String::from(other.name()), other);
        } else {
            panic!("input with name '{}' already defined in the module", other.name());
        }
    }
}

impl<'module> SubAssign<Signal<'module>> for Module<'module> {
    fn sub_assign(&mut self, other: Signal<'module>) {
        if let None = self.outputs.get(other.name()) {
            self.outputs.insert(String::from(other.name()), other);
        } else {
            panic!("output with name '{}' already defined in the module", other.name());
        }
    }
}
/*

#[duplicate(tt; [Module]; [Scope])]
impl Index<Signal> for tt {
    type Output = Op;

    fn index(&self, index: Signal) -> &Self::Output {
        for (name, item) in &self.assigns {
            if index.name() == name {
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
                self.add_assign(assign);
            }
            write(&mut self.assign_op, Op::fake());
        }

        self.assign_signal = Some(signal);
        &mut self.assign_op
    }
}
*/

impl<'module> Scope<'module> {
    pub fn new() -> Self {
        Scope {
            cond: AlwaysComb,
            scopes: vec![],

            assigns: BTreeMap::new(),
            assign_signal: None,
            assign_op: None,

            sync: false,
        }
    }

    pub fn when<T>(&mut self, _cond: &str, add_rules: T) -> &mut Self where T: Fn(&mut Scope) -> () {
        let mut scope = Scope::new();

        scope.sync = self.sync;
        self.cond = When(true);

        add_rules(&mut scope);
        self.scopes.push(scope);
        return self
    }

    pub fn elsewhen<T>(&mut self, _stub: bool, add_rules: T) -> &mut Self where T: Fn(&mut Scope) -> () {
        let mut scope = Scope::new();

        scope.sync = self.sync;
        self.cond = ElseWhen(true);

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
            s.push_str(&format!("{} {} {};", &sig.repr(), assign_op, &self.assign_op.as_ref().unwrap().repr()));
            s.push_str("\n");
        }

        for (_, assign) in &self.assigns {
            s.push_str(&assign.synth(sync));
            s.push_str("\n");
        }

        for scope in self.scopes.iter() {
            s.push_str(&scope.synth());
        }
        return s;
    }
}

/*
impl AddAssign<Assign> for Scope {
    fn add_assign(&mut self, other: Assign) {
        if let None = self.assigns.get(other.dest.name()) {
            self.assigns.insert(String::from(other.dest.name()), other);
        } else {
            panic!("assign with destination '{}' already defined in this scope", other.dest.name());
        }
    }
}
*/

impl Synth for Scope<'_> {
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

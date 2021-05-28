use std::ops::{AddAssign, SubAssign};
use super::{Synth, Signal};
use super::expr::{Assign};
use super::condition::{Conditional, Conditional::*};
//use std::ptr::{read, write};
//use duplicate::duplicate;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct Scope<'module> {
    cond: Conditional<'module>,

    assigns: BTreeMap<String, Assign<'module>>,

    sync: bool,
    scopes: Vec<Scope<'module>>,
}

pub struct Module<'module> {
    name: String,
    inputs: BTreeMap<String, Signal<'module>>,
    outputs: BTreeMap<String, Signal<'module>>,

    assigns: BTreeMap<String, Assign<'module>>,
    scopes: Vec<Scope<'module>>,
}

pub trait SignalHolder {
    fn logic(&self, name: &'_ str, width: u32) -> Signal<'_>;
}
pub type VModule<'a> = Rc<Module<'a>>;

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
    pub fn new(name: &str) -> VModule {
        return Rc::new(Module {
            name: String::from(name),
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
            scopes: vec![],

            assigns: BTreeMap::new(),
        })
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

impl<'module> SignalHolder for VModule<'module> {
    fn logic(&self, name: &'_ str, width: u32) -> Signal<'_> {
        let mut sig = Signal::new(name, width);
        sig.module = Some(Rc::clone(self));
        sig
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

impl<'module> AddAssign<Signal<'module>> for VModule<'module> {
    fn add_assign(&mut self, other: Signal<'module>) {
        let val = Rc::get_mut(self).unwrap();
        if !val.inputs.contains_key(other.name()) {
            val.inputs.insert(String::from(other.name()), other.copy());
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

impl<'module> Scope<'module> {
    pub fn new() -> Self {
        Scope {
            cond: AlwaysComb,
            scopes: vec![],

            assigns: BTreeMap::new(),
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

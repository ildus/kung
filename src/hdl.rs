mod expr;
mod signal;
mod module;

pub use crate::hdl::signal::Signal;
pub use crate::hdl::module::Module;

pub trait Synth {
    fn synth(&self) -> String;
}

pub trait Operand {
    fn repr(&self) -> String;
}


pub mod expr;
mod signal;
mod module;

pub use crate::hdl::signal::Signal;
pub use crate::hdl::module::Module;

use num::Integer;
use std::fmt::Display;

pub trait Synth {
    fn synth(&self) -> String;
}

pub trait Operand {
    fn repr(&self) -> String;
}

impl<T:Integer + Display> Operand for T {
    fn repr(&self) -> String {
        return self.to_string()
    }
}

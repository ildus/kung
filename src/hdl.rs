pub mod expr;
pub mod condition;

mod signal;
mod module;

pub use crate::hdl::signal::Signal;
pub use crate::hdl::module::Module;

use duplicate::duplicate;

pub trait Synth {
    fn synth(&self) -> String;
}

pub trait Operand {
    fn repr(&self) -> String;
}

#[duplicate(tt; [u32]; [i32]; [String])]
impl Operand for tt {
    fn repr(&self) -> String {
        return self.to_string()
    }
}

#[macro_export]
macro_rules! comb {
    ($a:ident := $e:expr) => {{
        {
            //check is valid op
            if let Some(ref mut m) = $a.module {
                m += expr::Assign::new(&$a, $e);
            }
            println!("{} = {}", stringify!{$a}, stringify!{$e});
        }
    }};
}

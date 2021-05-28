pub mod expr;
pub mod condition;

mod signal;
mod module;

pub use crate::hdl::signal::Signal;
pub use crate::hdl::module::{Module, SignalHolder};

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
            use std::ptr;

            unsafe {
                //we expect that module lives longer than the signal anyway
                if let Some(sigmod) = $a.module {
                    let mut uninit: std::mem::MaybeUninit<Module> = std::mem::MaybeUninit::uninit();
                    let ptr = uninit.as_mut_ptr();
                    ptr::write(ptr, ptr::read(&*sigmod));
                    *ptr += expr::Assign::new(&$a, $e);
                } else {
                    panic!("signal doesn't have associated module")
                }
            }
            println!("{} = {}", stringify!{$a}, stringify!{$e});
        }
    }};
}

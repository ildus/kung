use crate::hdl::{Signal};

pub enum Conditional<'module> {
    AlwaysComb,
    Posedge(&'module Signal<'module>),
    When(bool),
    ElseWhen(bool),
    Otherwise,
}

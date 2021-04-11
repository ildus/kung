#[cfg(test)]

use crate::hdl::*;

#[test]
fn simple_adder() {
    let mut m = Module::new("adder");
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);
    let o = Signal::new("o", 32);

    m += a;
    m += b;
    m[o] = a + b;
    m -= o;
    assert_eq!(m.synth(), "module adder();\ninput logic [31:0] a;\ninput logic [31:0] b;\noutput logic [31:0] o;\nassign o = (a + b);\nendmodule");
}

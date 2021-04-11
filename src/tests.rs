#[cfg(test)]

use crate::hdl::*;

#[test]
fn simple_adder() {
    let mut m = Module::new("adder");
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);
    let c = Signal::new("c", 32);
    let o = Signal::new("o", 32);

    m += a;
    m += b;
    m -= o;

    m[c] = a + b;
    m[o] = c + 1;

    assert_eq!(m.synth(), "module adder();\ninput logic [31:0] a;\ninput logic [31:0] b;\noutput logic [31:0] o;\nassign o = (c + 1);\nassign c = (a + b);\nendmodule");
}

#[test]
fn ops() {
    use crate::hdl::expr::Op;

    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);

    let mut op: Op = a + 1;
    assert_eq!(op.repr(), "(a + 1)");

    op = a - 1;
    assert_eq!(op.repr(), "(a - 1)");

    op = a >> 1;
    assert_eq!(op.repr(), "(a >> 1)");

    op = a << 1;
    assert_eq!(op.repr(), "(a << 1)");

    assert_eq!((a + b).repr(), "(a + b)");
    assert_eq!((a - b).repr(), "(a - b)");
    assert_eq!((a << b).repr(), "(a << b)");
    assert_eq!((a >> b).repr(), "(a >> b)");
    assert_eq!((a + (a - b)).repr(), "(a + (a - b))");
    assert_eq!(((a - b) + a).repr(), "((a - b) + a)");
    assert_eq!(((a - b) - a).repr(), "((a - b) - a)");
    assert_eq!(((a - b) + (a + b)).repr(), "((a - b) + (a + b))");
    assert_eq!(((a - b) - (a + b)).repr(), "((a - b) - (a + b))");
}

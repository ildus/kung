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

    //m[c] = a + b;
    //m[o] = c + 1;

    assert_eq!(m.synth(), "module adder();\ninput logic [31:0] a;\ninput logic [31:0] b;\noutput logic [31:0] o;\nassign o = (c + 1);\nassign c = (a + b);\nendmodule\n");
}

    /*
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
    assert_eq!((a * b).repr(), "(a * b)");
    assert_eq!((a / b).repr(), "(a / b)");
    assert_eq!((a << b).repr(), "(a << b)");
    assert_eq!((a >> b).repr(), "(a >> b)");
    assert_eq!((a & b).repr(), "(a & b)");
    assert_eq!((a | b).repr(), "(a | b)");
    assert_eq!((a ^ b).repr(), "(a ^ b)");
    assert_eq!((!a).repr(), "(~a)");
    assert_eq!((a + (a - b)).repr(), "(a + (a - b))");
    assert_eq!(((a - b) + a).repr(), "((a - b) + a)");
    assert_eq!(((a - b) - a).repr(), "((a - b) - a)");
    assert_eq!(((a - b) + (a + b)).repr(), "((a - b) + (a + b))");
    assert_eq!(((a - b) - (a + b)).repr(), "((a - b) - (a + b))");
    assert_eq!(((a - b) * (a + b)).repr(), "((a - b) * (a + b))");
    assert_eq!(((a - b) / (a + b)).repr(), "((a - b) / (a + b))");
    assert_eq!(((a - b) & (a + b)).repr(), "((a - b) & (a + b))");
    assert_eq!(((a - b) | (a + b)).repr(), "((a - b) | (a + b))");
    assert_eq!(((a - b) ^ (a + b)).repr(), "((a - b) ^ (a + b))");
    assert_eq!((!(a - b)).repr(), "(~(a - b))");
    assert_eq!(((a - b) - 1u32).repr(), "((a - b) - 1)");
}

#[test]
fn comb() {
    let mut m = Module::new("comb");
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);
    let c = Signal::new("c", 32);

    // always_comb
    m.comb(|s| {
        s[c] = a + 1;
        s.when(a == 1, |s| {
            s[b] = a + c;
        });
    });

    assert_eq!(m.synth(), "module comb();\n\nalways_comb begin\nc = (a + 1);\nif ((a == 1)) begin\nb = (a + c);\nend\nend\n\nendmodule\n");
}

#[test]
fn sync() {
    let mut m = Module::new("sync");
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);
    let c = Signal::new("c", 32);
    let clk = Signal::bool("clk");

    m.on(clk, |s| {
        s[c] = a + 1;
        s.when(a == 1, |s| {
            s[b] = a + c;
        }).otherwise(|s| {
            s[b] = a + c + 2;
        });
    });

    assert_eq!(m.synth(), "module sync();\n\nalways_ff @(posedge clk) begin\nc <= (a + 1);\nif ((a == 1)) begin\nb <= (a + c);\nend\nelse begin\nb <= ((a + c) + 2);\nend\nend\n\nendmodule\n");
}

#[test]
fn signal_conds() {
    let mut m = Module::new("cond");
    let comp = Signal::new("comp", 32);
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);
    let c = Signal::new("c", 32);
    let d = Signal::new("d", 32);
    let e = Signal::new("e", 32);
    let f = Signal::new("f", 32);
    let g = Signal::new("g", 32);

    m[a] = true.into();
    m[b] = (a == 1).into();
    m[c] = (b != comp).into();
    m[d] = (c >= 1).into();
    m[e] = (d <= comp).into();
    m[f] = (e > 1).into();
    m[g] = (f < 1).into();

    assert_eq!(m.synth(), "module cond();\nassign g = ((f < 1));\nassign a = (1);\nassign b = ((a == 1));\nassign c = ((b != comp));\nassign d = ((c >= 1));\nassign e = ((d <= comp));\nassign f = ((e > 1));\nendmodule\n");
}
*/

#[macro_export]
macro_rules! cond {
    ($e:expr) => {{
        {
            //let _res: bool = $e;
            println!("{}", stringify!{$e});
        }
    }};
}

#[test]
fn complex_conds() {
    let a = Signal::new("a", 32);
    let b = Signal::new("b", 32);

    cond! { a == 1 || b == 2 }
}

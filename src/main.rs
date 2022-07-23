#![allow(incomplete_features)]
// #![feature(const_for)]
#![feature(generic_const_exprs)]
// #![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(let_else)]
#![feature(const_try)]
#![feature(const_convert)]
#![allow(dead_code)]

/// Emulates `for ... in ... {}` loops in `const fn`s.
/// Should be replaced by regular `for` loops when
/// [const_for](https://github.com/rust-lang/rust/issues/87575) becomes usuable because
/// [const_trait_impl](https://github.com/rust-lang/rust/issues/67792) implements `~const` for range operators.
macro_rules! repeat {
    ($i:ident in $from:tt..$to:tt $body:block) => {{
        let mut $i = $from;
        while $i < $to {
            $body
            $i += 1;
        }
    }};
}

mod algebra;
mod common;
mod mv;
mod shape;
mod sign;

use algebra::Algebra;
use common::*;
use mv::MV;
use shape::Shape;

use crate::shape::shape_to_string;

fn main() {
    const N: usize = 2;
    type VGA = Algebra<N>;
    println!("Blade count: {}", basis_blade_count(N));
    let a = Shape([false, true]);
    let b = Shape([true, true]);
    println!("{a} ∧ {b} = {}", shape_to_string(a.ext(b)));
    println!("{a} >> {b} = {}", shape_to_string(a.left_contraction(b)));
    println!("{a} << {b} = {}", shape_to_string(a.right_contraction(b)));
    println!("{a} | {b} = {}", shape_to_string(a.inner(b)));
    println!("{a} ⋅ {b} = {}", shape_to_string(a.dot(b)));
    println!("{a} * {b} = {}", shape_to_string(a.scalar(b)));

    let d: Shape<4> = Shape([true, true, false, true]);
    println!("~{d} = {}{d}", d.rev());

    let x: MV<N> = MV([2.0; basis_blade_count(N)]);
    let y: MV<N> = MV([1.0; basis_blade_count(N)]);
    let z = x.scale(4.0).add(y);
    println!("z = {z}");
}

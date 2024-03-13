use baby_stark_math_lib;

use crate::field::FieldElement;
mod field;
fn main() {
    println!("Hello, world!");

    let x = baby_stark_math_lib::gcd::gcd(28, 32);
    println!("x = {}", x);

    let y = FieldElement::new();
    let z = FieldElement::__init__(30, 2);

    let u = y.__add__(z);

    println!("{:?}", u);

}



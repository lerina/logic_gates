use logic_gates::{and, xor};

// type aliases for easiers reading
pub type Sum = u8;
pub type Carry = u8;

// setup helper function
pub fn half_adder_setup() -> Vec<( (u8, u8), (Sum, Carry) )> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

/// This function implements a half adder 
/// using primitive gates
///
fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
/// one_bit_adder integration test function, 
/// in which we iterate over our half adder input output
/// pairs and assert against the output of the  half_adder.
///
fn one_bit_adder() {
    println!("Testing: a, b -> (Sum, Carry)");
    for (inn, out) in half_adder_setup() {
        let (a, b) = inn;
        println!("Testing: {}, {} -> {:?}", a, b, out);
        assert_eq!(half_adder(a, b), out);
    }
}

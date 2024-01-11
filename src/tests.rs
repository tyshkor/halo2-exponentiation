use std::marker::PhantomData;

use crate::utils::u64_to_bits_rev;

use super::MyCircuit;
use halo2_proofs::{dev::MockProver, pasta::Fp};

#[test]
fn exp_example1() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(2);
    let result = Fp::from(8);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![
        x,
        result,
    ];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
#[should_panic]
fn exp_example1_fail() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(2);
    let result = Fp::from(9);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![
        x,
        result,
    ];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

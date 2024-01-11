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

    let mut public_input = vec![x, result];

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

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
fn exp_example2() {
    let k = 5;

    const N: usize = 5;

    let x = Fp::from(2);
    let result = Fp::from(128);
    let n = 7;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
#[should_panic]
fn exp_example2_fail() {
    let k = 5;

    const N: usize = 5;

    let x = Fp::from(2);
    let result = Fp::from(129);
    let n = 7;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
fn exp_example3() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(5);
    let result = Fp::from(125);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
#[should_panic]
fn exp_example3_fail() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(5);
    let result = Fp::from(99);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
fn exp_example4() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(4);
    let result = Fp::from(256);
    let n = 4;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
#[should_panic]
fn exp_example4_fail() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(4);
    let result = Fp::from(255);
    let n = 4;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
fn exp_example5() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(7);
    let result = Fp::from(343);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

#[test]
#[should_panic]
fn exp_example5_fail() {
    let k = 5;

    const N: usize = 8;

    let x = Fp::from(7);
    let result = Fp::from(120);
    let n = 3;

    let mut n_bits: Vec<_> = u64_to_bits_rev(n, N);

    let circuit = MyCircuit::<Fp, 8>(PhantomData);

    let mut public_input = vec![x, result];

    public_input.append(&mut n_bits);

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
}

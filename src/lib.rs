use halo2_proofs::{arithmetic::FieldExt, circuit::*, plonk::*, poly::Rotation};
use std::marker::PhantomData;

mod utils;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
struct ExponentiationConfig {
    pub col_y: Column<Advice>,
    pub col_x: Column<Advice>,
    pub col_n: Column<Advice>,
    pub selector: Selector,
    pub instance: Column<Instance>,
}

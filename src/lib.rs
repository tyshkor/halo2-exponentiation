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

#[derive(Debug, Clone)]
struct ExponentiationChip<F: FieldExt> {
    config: ExponentiationConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> ExponentiationChip<F> {
    pub fn construct(config: ExponentiationConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>) -> ExponentiationConfig {
        let col_y = meta.advice_column();
        let col_x = meta.advice_column();
        let col_n = meta.advice_column();
        let selector = meta.selector();
        let instance = meta.instance_column();
        meta.enable_equality(instance);

        meta.enable_equality(col_y);
        meta.enable_equality(col_x);
        meta.enable_equality(col_n);

        let const_col = meta.fixed_column();

        meta.enable_constant(const_col);

        meta.create_gate("main", |meta| {
            let s = meta.query_selector(selector);

            let y_prev = meta.query_advice(col_y, Rotation::prev());
            let y_cur = meta.query_advice(col_y, Rotation::cur());

            let x_prev = meta.query_advice(col_x, Rotation::prev());
            let n_prev = meta.query_advice(col_n, Rotation::prev());
            vec![s * (n_prev.clone() * (y_cur.clone() - y_prev.clone() * x_prev) + (Expression::Constant(F::one()) - n_prev) * (y_cur - y_prev))]
        });

        ExponentiationConfig {
            col_y,
            col_x,
            col_n,
            selector,
            instance,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        len: usize,
    ) -> Result<
        
            AssignedCell<F, F>
        ,
        Error,
    > {
        layouter.assign_region(
            || "table",
            |mut region| {
                let mut y_cell =
                    region.assign_advice_from_constant(|| "y", self.config.col_y, 0, F::one())?;

                let mut x_cell = region.assign_advice_from_instance(
                    || "x",
                    self.config.instance,
                    0,
                    self.config.col_x,
                    0,
                )?;

                let mut x_power_vec = Vec::with_capacity(len);

                x_power_vec.push(x_cell.clone());

                for i in 1..len {
                    x_cell = region.assign_advice(
                        || "x",
                        self.config.col_x,
                        i,
                        || x_cell.clone().value().copied() * x_cell.clone().value(),
                    )?;
                    x_power_vec.push(x_cell.clone());
                }

                let mut n_binary_vec = Vec::with_capacity(len);

                for i in 0..len {
                    let cell = region.assign_advice_from_instance(
                        || "n_binary",
                        self.config.instance,
                        i + 2,
                        self.config.col_n,
                        i,
                    )?;
                    n_binary_vec.push(cell.clone());
                }

                for i in 1..len {

                    let one_minus_n = n_binary_vec[i - 1].value().map(|n| {
                        let n_val = n.get_lower_32() as u64;
                        F::from(1 - n_val)
                    });

                    y_cell = region.assign_advice(
                        || "y",
                        self.config.col_y,
                        i,
                        || n_binary_vec[i - 1].value().copied()
                                * (y_cell.value().copied() * x_power_vec[i - 1].value())
                                + one_minus_n * y_cell.value().copied(),
                    )?;

                    
                    if i < len - 1 {
                        self.config.selector.enable(&mut region, i)?;
                    }

                }

                Ok(y_cell)
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: &AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.cell(), self.config.instance, row)
    }
}
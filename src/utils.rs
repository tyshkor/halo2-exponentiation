use halo2_proofs::pasta::Fp;

// bitwise representation of n in reverse order
#[allow(dead_code)]
pub fn u64_to_bits_rev(input: u64, len: usize) -> Vec<Fp> {
    let mut result = Vec::new();

    for i in 0..len {
        // Check the value of the i-th bit
        let bit_value = (input >> i) & 1;

        // Push the bit value to the result vector
        result.push(Fp::from(bit_value));
    }

    result
}

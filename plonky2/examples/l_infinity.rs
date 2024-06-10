use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

/// An example of using Plonky2 to prove that a given value lies in a given range.
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_permutation_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    builder.sorted_vector = vec![F::from_canonical_usize(1), F::from_canonical_usize(2), F::from_canonical_usize(3), F::from_canonical_usize(4), F::from_canonical_usize(5), F::from_canonical_usize(6)];
    builder.original_vector = vec![F::from_canonical_usize(2), F::from_canonical_usize(4), F::from_canonical_usize(6), F::from_canonical_usize(1), F::from_canonical_usize(3), F::from_canonical_usize(5)];

    // The secret value.
    //let value = builder.add_virtual_target();

    // Registered as a public input (even though it's secret) so we can print out the value later.
    //builder.register_public_input(value);

    // vec1 = vec![1, 2, 3, 4, 5, 6];
    // vec2 = vec![2, 4, 6, 1, 3, 5];
    //builder.add_vector(vec1, vec2);


    let mut pw = PartialWitness::new();
    //pw.set_target(value, F::from_canonical_usize(0));

    let data = builder.build::<C>();
    let proof = data.prove(pw)?;

    

    data.verify(proof)
}

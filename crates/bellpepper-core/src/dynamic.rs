
use ff::PrimeField;

use crate::{Index, LinearCombination, Variable, SynthesisError};

pub trait DynamicCS<Scalar: PrimeField>: Send {
    fn new() -> Self where Self: Sized {
        unimplemented!(
            "ConstraintSystem::new must be implemented for extensible types implementing ConstraintSystem"
        );
    }

    fn one(&self) -> Variable {
        Variable::new_unchecked(Index::Input(0))
    }

    fn alloc_strict(
        &mut self,
        annotation: Box<dyn FnOnce() -> String>,
        f: Scalar,
    ) -> Result<Variable, SynthesisError>;

    fn alloc_input_strict(
        &mut self,
        annotation: Box<dyn FnOnce() -> String>,
        f: Scalar,
    ) -> Result<Variable, SynthesisError>;

    fn enforce(
        &mut self,
        annotation: Box<dyn FnOnce() -> String>,
        a: Box<dyn FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>>,
        b: Box<dyn FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>>,
        c: Box<dyn FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>>,
    );

    fn is_extensible(&self) -> bool {
        false
    }

    fn is_witness_generator(&self) -> bool {
        false
    }

    fn extend_inputs(&mut self, _new_inputs: &[Scalar]) {
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::extend_inputs must be implemented for witness generators implementing ConstraintSystem")
    }

    fn extend_aux(&mut self, _new_aux: &[Scalar]) {
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::extend_aux must be implemented for witness generators implementing ConstraintSystem")
    }

    fn allocate_empty(
        &mut self,
        _aux_n: usize,
        _inputs_n: usize,
    ) -> (&mut [Scalar], &mut [Scalar]) {
        // This method should only ever be called on witness generators.
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::allocate_empty must be implemented for witness generators implementing ConstraintSystem")
    }

    fn allocate_empty_inputs(&mut self, _n: usize) -> &mut [Scalar] {
        // This method should only ever be called on witness generators.
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::allocate_empty_inputs must be implemented for witness generators implementing ConstraintSystem")
    }

    fn allocate_empty_aux(&mut self, _n: usize) -> &mut [Scalar] {
        // This method should only ever be called on witness generators.
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::allocate_empty_aux must be implemented for witness generators implementing ConstraintSystem")
    }

    fn inputs_slice(&self) -> &[Scalar] {
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::inputs_slice must be implemented for witness generators implementing ConstraintSystem")
    }

    fn aux_slice(&self) -> &[Scalar] {
        assert!(self.is_witness_generator());
        unimplemented!("ConstraintSystem::aux_slice must be implemented for witness generators implementing ConstraintSystem")
    }
}


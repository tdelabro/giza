use air::{ProcessorAir, PublicInputs};
use giza_core::Felt;
use prover::{Prover, Trace};
use runner::{ExecutionError, ExecutionTrace, Program};

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use prover::StarkProof;

// EXECUTOR
// ================================================================================================

/// Executes the specified `program` and returns the result together with a STARK-based proof of execution.
pub fn execute(
    program: &mut Program,
    options: &ProofOptions,
) -> Result<(Vec<u64>, StarkProof), ExecutionError> {
    // execute the program to create an execution trace
    let trace = program.execute()?;
    let outputs = vec![];

    // generate STARK proof
    let prover = ExecutionProver::new(options.clone());
    let proof = prover.prove(trace).map_err(ExecutionError::ProverError)?;

    Ok((outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver {
    options: ProofOptions,
}

impl ExecutionProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

impl Prover for ExecutionProver {
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;

    fn options(&self) -> &prover::ProofOptions {
        &self.options
    }

    fn get_pub_inputs(&self, trace: &ExecutionTrace) -> PublicInputs {
        let last_step = trace.length() - 1;
        let pc = vec![trace.get(0, 0), trace.get(0, last_step)];
        let ap = vec![trace.get(1, 0), trace.get(1, last_step)];
        PublicInputs::new(pc, ap)
    }
}
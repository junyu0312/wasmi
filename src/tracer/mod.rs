use crate::runner::InstructionOutcome;
use wasmi_core::TrapCode;

pub mod isa;

#[derive(Clone, Copy)]
pub struct Context {
    pub memory_size: usize,
    pub sp: usize,
}

pub enum ExecutionResult {
    Normal,
    Trap(TrapCode),
}

impl From<&Result<InstructionOutcome, TrapCode>> for ExecutionResult {
    fn from(ret: &Result<InstructionOutcome, TrapCode>) -> Self {
        match ret {
            Ok(_) => ExecutionResult::Normal,
            Err(trap_code) => ExecutionResult::Trap(*trap_code),
        }
    }
}

pub struct StepTrace {
    before: Context,
    after: Context,
    execution_result: ExecutionResult,
}

impl StepTrace {
    pub fn new(before: Context, after: Context, execution_result: ExecutionResult) -> Self {
        StepTrace {
            before,
            after,
            execution_result,
        }
    }
}

pub struct StepsTrace {
    steps: Vec<StepTrace>,
}

impl StepsTrace {
    pub fn new() -> Self {
        StepsTrace { steps: vec![] }
    }

    pub fn push(&mut self, step: StepTrace) {
        self.steps.push(step)
    }
}

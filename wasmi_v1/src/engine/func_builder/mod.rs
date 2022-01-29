mod control_frame;
mod control_stack;
mod inst_builder;
mod value_stack;

pub use self::inst_builder::{InstructionIdx, InstructionsBuilder, LabelIdx, Reloc};
use self::{control_frame::ControlFrame, control_stack::ControlFlowStack, value_stack::ValueStack};
use crate::{
    module2::{BlockType, FuncIdx, ModuleResources},
    Engine,
    ModuleError,
};
use wasmi_core::ValueType;

/// The interface to translate a `wasmi` bytecode function using Wasm bytecode.
#[derive(Debug)]
pub struct FunctionBuilder<'engine, 'parser> {
    /// The [`Engine`] for which the function is translated.
    engine: &'engine Engine,
    /// The function under construction.
    func: FuncIdx,
    /// The immutable `wasmi` module resources.
    res: ModuleResources<'parser>,
    /// The control flow frame stack that represents the Wasm control flow.
    control_frames: ControlFlowStack,
    /// The emulated value stack.
    value_stack: ValueStack,
    /// The instruction builder.
    ///
    /// # Note
    ///
    /// Allows to incrementally construct the instruction of a function.
    inst_builder: InstructionsBuilder,
    /// The amount of local variables of the currently compiled function.
    len_locals: usize,
    /// The maximum height of the emulated value stack of the translated function.
    ///
    /// # Note
    ///
    /// This does not include input parameters and local variables.
    max_stack_height: usize,
    /// This represents the reachability of the currently translated code.
    ///
    /// - `true`: The currently translated code is reachable.
    /// - `false`: The currently translated code is unreachable and can be skipped.
    ///
    /// # Note
    ///
    /// Visiting the Wasm `Else` or `End` control flow operator resets
    /// reachability to `true` again.
    reachable: bool,
}

impl<'engine, 'parser> FunctionBuilder<'engine, 'parser> {
    /// Creates a new [`FunctionBuilder`].
    pub fn new(engine: &'engine Engine, func: FuncIdx, res: ModuleResources<'parser>) -> Self {
        Self {
            engine,
            func,
            res,
            control_frames: ControlFlowStack::default(),
            value_stack: ValueStack::default(),
            inst_builder: InstructionsBuilder::default(),
            len_locals: 0,
            max_stack_height: 0,
            reachable: true,
        }
    }

    /// Translates the given local variables for the translated function.
    pub fn translate_locals(
        &mut self,
        amount: u32,
        _value_type: ValueType,
    ) -> Result<(), ModuleError> {
        self.len_locals += amount as usize;
        Ok(())
    }

    /// Translates a Wasm `block` control flow operator.
    pub fn translate_block(&mut self, _block_type: BlockType) -> Result<(), ModuleError> {
        let end_label = self.inst_builder.new_label();
        self.control_frames
            .push_frame(ControlFrame::Block { end_label });
        Ok(())
    }

    /// Translates a Wasm `block` control flow operator.
    pub fn translate_loop(&mut self, _block_type: BlockType) -> Result<(), ModuleError> {
        let header = self.inst_builder.new_label();
        self.inst_builder.resolve_label(header);
        self.control_frames
            .push_frame(ControlFrame::Loop { header });
        Ok(())
    }
}

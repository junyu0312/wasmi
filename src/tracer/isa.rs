pub trait Load<Value> {
    fn new_load_post(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPost<Value>;
}

pub trait Store<Value> {
    fn new_store_pre(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPre<Value>;

    fn new_store_post(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPost<Value>;
}

impl<Value> Load<Value> for i32 {
    fn new_load_post(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPost<Value> {
        InstructionDerivationPost::I32Load {
            offset,
            raw_address,
            value,
        }
    }
}

impl<Value> Store<Value> for i32 {
    fn new_store_pre(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPre<Value> {
        InstructionDerivationPre::I32Store {
            offset,
            raw_address,
            value,
        }
    }

    fn new_store_post(
        offset: u32,
        raw_address: u32,
        value: Value,
    ) -> InstructionDerivationPost<Value> {
        InstructionDerivationPost::I32Store {
            offset,
            raw_address,
            value,
        }
    }
}

#[derive(Copy, Clone)]
pub enum InstructionDerivationPre<Value> {
    Br {
        dst_pc: u32,
        keep: bool,
        drop: u32,
    },
    BrIfEqz {
        cond: bool,
        dst_pc: u32,
        keep: bool,
        drop: u32,
    },
    BrIfNez {
        cond: bool,
        dst_pc: u32,
        keep: bool,
        drop: u32,
    },
    Return {
        keep: bool,
        drop: u32,
    },

    Select {
        left: Value,
        mid: Value,
        right: Value,
    },

    SetLocal {
        depth: u32,
        value: Value,
    },
    SetGlobal {
        index: u32,
        value: Value,
    },

    I32Store {
        offset: u32,
        raw_address: u32,
        value: Value,
    },

    GrowMemory {
        pages: u32,
    },

    I32Add {
        lhs: i32,
        rhs: i32,
    },
}

pub enum InstructionDerivationPost<Value> {
    Select {
        left: Value,
        mid: Value,
        right: Value,
        output: Value,
    },

    Br {
        dst_pc: u32,
        keep: bool,
        drop: u32,
        return_value: Option<Value>,
    },
    BrIfEqz {
        cond: bool,
        dst_pc: u32,
        keep: bool,
        drop: u32,
        return_value: Option<Value>,
    },
    BrIfNez {
        cond: bool,
        dst_pc: u32,
        keep: bool,
        drop: u32,
        return_value: Option<Value>,
    },
    Return {
        keep: bool,
        drop: u32,
        return_value: Option<Value>,
    },

    Call {
        index: u32,
    },

    GetLocal {
        depth: u32,
        value: Value,
    },
    SetLocal {
        depth: u32,
        value: Value,
    },
    TeeLocal {
        depth: u32,
        value: Value,
    },
    GetGlobal {
        index: u32,
        value: Value,
    },
    SetGlobal {
        index: u32,
        value: Value,
    },

    I32Load {
        offset: u32,
        raw_address: u32,
        value: Value,
    },

    I32Store {
        offset: u32,
        raw_address: u32,
        value: Value,
    },

    CurrentMemory(usize),
    GrowMemory {
        pages: u32,
        new_size: u32,
    },

    I32Const(i32),

    I32Add {
        lhs: i32,
        rhs: i32,
        output: i32,
    },
}

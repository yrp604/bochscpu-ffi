use bochscpu::opcode::Opcode;

pub const BOCHSCPU_OPCODE_ERROR: u32 = 0;
const_assert_eq!(BOCHSCPU_OPCODE_ERROR, Opcode::Error as u32);
pub const BOCHSCPU_OPCODE_INSERTED : u32 = 1;
const_assert_eq!(BOCHSCPU_OPCODE_INSERTED, Opcode::Inserted as u32);

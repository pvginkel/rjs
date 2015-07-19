mod emit;

use jit::JitFunction;
use self::emit::{Emit, AluOp, ShiftOp};
pub use self::emit::Reg;

pub struct Codegen {
    emit: Emit
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            emit: Emit::new()
        }
    }
    
    pub fn build(&mut self) -> JitFunction {
        self.emit.build()
    }
    
    pub fn push<A: AsArg>(&mut self, arg: A) {
        match arg.as_arg() {
            Arg::Imm(imm) => self.emit.push_imm_size(imm.as_i32(), imm.size()),
            Arg::MemBase(basereg, disp, size) => self.emit.push_membase_size(basereg, disp, size),
            Arg::MemIndex(basereg, disp, indexreg, shift, size) => self.emit.push_memindex_size(basereg, disp, indexreg, shift, size),
            Arg::Reg(reg) => self.emit.push_reg_size(reg.reg(), reg.size()),
            _ => jit_assert!()
        }
    }
    
    pub fn pop<A: AsArg>(&mut self, arg: A) {
        match arg.as_arg() {
            Arg::MemBase(basereg, disp, size) => self.emit.pop_membase_size(basereg, disp, size),
            Arg::MemIndex(basereg, disp, indexreg, shift, size) => self.emit.pop_memindex_size(basereg, disp, indexreg, shift, size),
            Arg::Reg(reg) => self.emit.pop_reg_size(reg.reg(), reg.size()),
            _ => jit_assert!()
        }
    }
    
    pub fn mov<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        match (arg1.as_arg(), arg2.as_arg()) {
            (Arg::Reg(dreg), Arg::Reg(sreg)) => {
                assert!(dreg.size() == sreg.size());
                self.emit.mov_reg_reg_size(dreg.reg(), sreg.reg(), dreg.size());
            }
            (Arg::MemBase(basereg, disp, _), Arg::Reg(sreg)) => self.emit.mov_membase_reg_size(basereg, disp, sreg.reg(), sreg.size()),
            (Arg::MemIndex(basereg, disp, indexreg, shift, _), Arg::Reg(sreg)) => self.emit.mov_memindex_reg_size(basereg, disp, indexreg, shift, sreg.reg(), sreg.size()),
            (Arg::Mem(mem), Arg::Reg(sreg)) => self.emit.mov_mem_reg_size(mem, sreg.reg(), sreg.size()),
            (Arg::Reg(dreg), Arg::Imm(imm)) => self.emit.mov_reg_imm_size(dreg.reg(), imm.as_i64(), imm.size()),
            (Arg::Reg(dreg), Arg::Mem(mem)) => self.emit.mov_reg_mem_size(dreg.reg(), mem, dreg.size()),
            (Arg::Reg(dreg), Arg::MemBase(basereg, disp, _)) => self.emit.mov_reg_membase_size(dreg.reg(), basereg, disp, dreg.size()),
            (Arg::Reg(dreg), Arg::MemIndex(basereg, disp, indexreg, shift, _)) => self.emit.mov_reg_memindex_size(dreg.reg(), basereg, disp, indexreg, shift, dreg.size()),
            (Arg::MemBase(basereg, disp, _), Arg::Imm(imm)) => self.emit.mov_membase_imm_size(basereg, disp, imm.as_i32(), imm.size()),
            (Arg::MemIndex(basereg, disp, indexreg, shift, _), Arg::Imm(imm)) => self.emit.mov_memindex_imm_size(basereg, disp, indexreg, shift, imm.as_i32(), imm.size()),
            _ => jit_assert!()
        }
    }
    
    fn alu<A1: AsArg, A2: AsArg>(&mut self, opc: AluOp, arg1: A1, arg2: A2) {
        match (arg1.as_arg(), arg2.as_arg()) {
            (Arg::Reg(dreg), Arg::Reg(sreg)) => {
                assert_eq!(dreg.size(), sreg.size());
                self.emit.alu_reg_reg_size(opc, dreg.reg(), sreg.reg(), dreg.size());
            }
            (Arg::MemBase(basereg, disp, size), Arg::Reg(sreg)) => {
                assert_eq!(size, sreg.size());
                self.emit.alu_membase_reg_size(opc, basereg, disp, sreg.reg(), sreg.size());
            }
            (Arg::MemIndex(basereg, disp, indexreg, shift, size), Arg::Reg(sreg)) => {
                assert_eq!(size, sreg.size());
                self.emit.alu_memindex_reg_size(opc, basereg, disp, indexreg, shift, sreg.reg(), sreg.size());
            }
            (Arg::Reg(dreg), Arg::Imm(imm)) => {
                assert_eq!(dreg.size(), imm.size());
                self.emit.alu_reg_imm_size(opc, dreg.reg(), imm.as_i32(), imm.size());
            }
            (Arg::Reg(dreg), Arg::MemBase(basereg, disp, size)) => {
                assert_eq!(dreg.size(), size);
                self.emit.alu_reg_membase_size(opc, dreg.reg(), basereg, disp, dreg.size());
            }
            (Arg::Reg(dreg), Arg::MemIndex(basereg, disp, indexreg, shift, size)) => {
                assert_eq!(dreg.size(), size);
                self.emit.alu_reg_memindex_size(opc, dreg.reg(), basereg, disp, indexreg, shift, dreg.size());
            }
            (Arg::MemBase(basereg, disp, size), Arg::Imm(imm)) => {
                assert_eq!(size, imm.size());
                self.emit.alu_membase_imm_size(opc, basereg, disp, imm.as_i32(), imm.size());
            }
            (Arg::MemIndex(basereg, disp, indexreg, shift, size), Arg::Imm(imm)) => {
                assert_eq!(size, imm.size());
                self.emit.alu_memindex_imm_size(opc, basereg, disp, indexreg, shift, imm.as_i32(), imm.size());
            }
            _ => jit_assert!()
        }
    }
    
    pub fn add<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Add, arg1, arg2);
    }
    
    pub fn or<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Or, arg1, arg2);
    }
    
    pub fn adc<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Adc, arg1, arg2);
    }
    
    pub fn sbb<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Sbb, arg1, arg2);
    }
    
    pub fn and<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::And, arg1, arg2);
    }
    
    pub fn sub<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Sub, arg1, arg2);
    }
    
    pub fn xor<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::XOr, arg1, arg2);
    }
    
    pub fn cmp<A1: AsArg, A2: AsArg>(&mut self, arg1: A1, arg2: A2) {
        self.alu(AluOp::Cmp, arg1, arg2);
    }
    
    fn shift<A1: AsArg, A2: AsArg>(&mut self, opc: ShiftOp, arg1: A1, arg2: A2) {
        match (arg1.as_arg(), arg2.as_arg()) {
            (Arg::Reg(dreg), Arg::Imm(imm))
                => self.emit.shift_reg_imm_size(opc, dreg.reg(), imm.as_i32(), dreg.size()),
            (Arg::MemBase(basereg, disp, size), Arg::Imm(imm))
                => self.emit.shift_membase_imm_size(opc, basereg, disp, imm.as_i32(), size),
            (Arg::MemIndex(basereg, disp, indexreg, shift, size), Arg::Imm(imm))
                => self.emit.shift_memindex_imm_size(opc, basereg, disp, indexreg, shift, imm.as_i32(), size),
            _ => jit_assert!()
        }
    }
    
    fn shl<A1: AsArg, A2: AsArg>(&mut self, opc: ShiftOp, arg1: A1, arg2: A2) {
        self.shift(ShiftOp::Shl, arg1, arg2);
    }
    
    fn shr<A1: AsArg, A2: AsArg>(&mut self, opc: ShiftOp, arg1: A1, arg2: A2) {
        self.shift(ShiftOp::Shr, arg1, arg2);
    }
    
    fn sar<A1: AsArg, A2: AsArg>(&mut self, opc: ShiftOp, arg1: A1, arg2: A2) {
        self.shift(ShiftOp::Sar, arg1, arg2);
    }
    
    pub fn call<A: AsArg>(&mut self, arg: A) {
        match arg.as_arg() {
            Arg::Imm(imm) => self.emit.call_imm(imm.as_i32()),
            Arg::MemBase(basereg, disp, _) => self.emit.call_membase(basereg, disp),
            Arg::MemIndex(basereg, disp, indexreg, shift, _) => self.emit.call_memindex(basereg, disp, indexreg, shift),
            Arg::Reg(reg) => self.emit.call_reg(reg.reg()),
            _ => jit_assert!()
        }
    }
    
    pub fn ret(&mut self) {
        self.emit.ret();
    }
}

#[derive(Copy, Clone)]
pub enum Arg {
    Reg(SizedReg),
    Imm(Imm),
    Mem(i64),
    MemSize(i64, i32),
    MemBase(Reg, i32, i32),
    MemIndex(Reg, i32, Reg, u8, i32)
}

pub trait AsArg : Copy {
    fn as_arg(self) -> Arg;
}

#[derive(Copy, Clone)]
pub enum Imm {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

impl Imm {
    fn as_i32(self) -> i32 {
        match self {
            Imm::U8(value) => value as i32,
            Imm::I8(value) => value as i32,
            Imm::U16(value) => value as i32,
            Imm::I16(value) => value as i32,
            Imm::U32(value) => value as i32,
            Imm::I32(value) => value,
            _ => jit_assert!()
        }
    }
    
    fn as_i64(self) -> i64 {
        match self {
            Imm::U64(value) => value as i64,
            Imm::I64(value) => value,
            _ => self.as_i32() as i64
        }
    }
    
    fn size(self) -> i32 {
        match self {
            Imm::U8(..) | Imm::I8(..) => 1,
            Imm::U16(..) | Imm::I16(..) => 2,
            Imm::U32(..) | Imm::I32(..) | Imm::F32(..) => 4,
            Imm::U64(..) | Imm::I64(..) | Imm::F64(..) => 8
        }
    }
}

#[derive(Copy, Clone)]
pub struct Mem(pub u64);

#[derive(Copy, Clone)]
pub struct MemSize(pub u64, pub i32);

#[derive(Copy, Clone)]
pub struct MemBase(pub SizedReg, pub i32);

#[derive(Copy, Clone)]
pub struct MemIndex(pub SizedReg, pub i32, pub SizedReg, pub u8);

#[derive(Copy, Clone)]
pub enum SizedReg {
    AL,
    AX,
    EAX,
    RAX,
    BL,
    BX,
    EBX,
    RBX,
    CL,
    CX,
    ECX,
    RCX,
    DL,
    DX,
    EDX,
    RDX,
    R8B,
    R8W,
    R8D,
    R8,
    R9B,
    R9W,
    R9D,
    R9,
    R10B,
    R10W,
    R10D,
    R10,
    R11B,
    R11W,
    R11D,
    R11,
    R12B,
    R12W,
    R12D,
    R12,
    R13B,
    R13W,
    R13D,
    R13,
    R14B,
    R14W,
    R14D,
    R14,
    R15B,
    R15W,
    R15D,
    R15,
    BPL,
    BP,
    EBP,
    RBP,
    SIL,
    SI,
    ESI,
    RSI,
    DIL,
    DI,
    EDI,
    RDI,
    SPL,
    SP,
    ESP,
    RSP,
    IP,
    EIP,
    RIP
}

pub mod prologue {
    pub use super::{Arg, AsArg, Imm, Codegen, Mem, MemSize, MemBase};
    pub use super::{MemIndex, Reg, SizedReg};
    pub use super::SizedReg::*;
    
    type M = Mem;
    type MB = MemBase;
    type MI = MemIndex;

    impl AsArg for Imm {
        fn as_arg(self) -> Arg {
            Arg::Imm(self)
        }
    }
    
    impl AsArg for Mem {
        fn as_arg(self) -> Arg {
            Arg::Mem(self.0 as i64)
        }
    }
    
    impl AsArg for MemSize {
        fn as_arg(self) -> Arg {
            Arg::MemSize(self.0 as i64, self.1)
        }
    }
    
    impl AsArg for MemBase {
        fn as_arg(self) -> Arg {
            Arg::MemBase(self.0.reg(), self.1, self.0.size())
        }
    }
    
    impl AsArg for MemIndex {
        fn as_arg(self) -> Arg {
            Arg::MemIndex(self.0.reg(), self.1, self.2.reg(), self.3, self.0.size())
        }
    }
    
    impl AsArg for u8 {
        fn as_arg(self) -> Arg {
            Imm::U8(self).as_arg()
        }
    }
    
    impl AsArg for i8 {
        fn as_arg(self) -> Arg {
            Imm::I8(self).as_arg()
        }
    }
    
    impl AsArg for u16 {
        fn as_arg(self) -> Arg {
            Imm::U16(self).as_arg()
        }
    }
    
    impl AsArg for i16 {
        fn as_arg(self) -> Arg {
            Imm::I16(self).as_arg()
        }
    }
    
    impl AsArg for u32 {
        fn as_arg(self) -> Arg {
            Imm::U32(self).as_arg()
        }
    }
    
    impl AsArg for i32 {
        fn as_arg(self) -> Arg {
            Imm::I32(self).as_arg()
        }
    }
    
    impl AsArg for u64 {
        fn as_arg(self) -> Arg {
            Imm::U64(self).as_arg()
        }
    }
    
    impl AsArg for i64 {
        fn as_arg(self) -> Arg {
            Imm::I64(self).as_arg()
        }
    }
    
    impl AsArg for f32 {
        fn as_arg(self) -> Arg {
            Imm::F32(self).as_arg()
        }
    }
    
    impl AsArg for f64 {
        fn as_arg(self) -> Arg {
            Imm::F64(self).as_arg()
        }
    }

    impl AsArg for SizedReg {
        fn as_arg(self) -> Arg {
            Arg::Reg(self)
        }
    }
    
    impl SizedReg {
        pub fn reg(self) -> Reg {
            match self {
                SizedReg::AL | SizedReg::AX | SizedReg::EAX | SizedReg::RAX => Reg::RAX,
                SizedReg::BL | SizedReg::BX | SizedReg::EBX | SizedReg::RBX => Reg::RBX,
                SizedReg::CL | SizedReg::CX | SizedReg::ECX | SizedReg::RCX => Reg::RCX,
                SizedReg::DL | SizedReg::DX | SizedReg::EDX | SizedReg::RDX => Reg::RDX,
                SizedReg::R8B | SizedReg::R8W | SizedReg::R8D | SizedReg::R8 => Reg::R8,
                SizedReg::R9B | SizedReg::R9W | SizedReg::R9D | SizedReg::R9 => Reg::R9,
                SizedReg::R10B | SizedReg::R10W | SizedReg::R10D | SizedReg::R10 => Reg::R10,
                SizedReg::R11B | SizedReg::R11W | SizedReg::R11D | SizedReg::R11 => Reg::R11,
                SizedReg::R12B | SizedReg::R12W | SizedReg::R12D | SizedReg::R12 => Reg::R12,
                SizedReg::R13B | SizedReg::R13W | SizedReg::R13D | SizedReg::R13 => Reg::R13,
                SizedReg::R14B | SizedReg::R14W | SizedReg::R14D | SizedReg::R14 => Reg::R14,
                SizedReg::R15B | SizedReg::R15W | SizedReg::R15D | SizedReg::R15 => Reg::R15,
                SizedReg::BPL | SizedReg::BP | SizedReg::EBP | SizedReg::RBP => Reg::RBP,
                SizedReg::SIL | SizedReg::SI | SizedReg::ESI | SizedReg::RSI => Reg::RSI,
                SizedReg::DIL | SizedReg::DI | SizedReg::EDI | SizedReg::RDI => Reg::RDI,
                SizedReg::SPL | SizedReg::SP | SizedReg::ESP | SizedReg::RSP => Reg::RSP,
                SizedReg::IP | SizedReg::EIP | SizedReg::RIP => Reg::RIP
            }
        }
        
        pub fn size(self) -> i32 {
            match self {
                SizedReg::AL | SizedReg::BL | SizedReg::CL | SizedReg::DL | SizedReg::R8B |
                SizedReg::R9B | SizedReg::R10B | SizedReg::R11B | SizedReg::R12B | SizedReg::R13B |
                SizedReg::R14B | SizedReg::R15B | SizedReg::BPL | SizedReg::SIL | SizedReg::DIL | SizedReg::SPL
                    => 1,
                SizedReg::AX | SizedReg::BX | SizedReg::CX | SizedReg::DX | SizedReg::R8W |
                SizedReg::R9W | SizedReg::R10W | SizedReg::R11W | SizedReg::R12W | SizedReg::R13W |
                SizedReg::R14W | SizedReg::R15W | SizedReg::BP | SizedReg::SI | SizedReg::DI | SizedReg::SP | SizedReg::IP
                    => 2,
                SizedReg::EAX | SizedReg::EBX | SizedReg::ECX | SizedReg::EDX | SizedReg::R8D | SizedReg::R9D |
                SizedReg::R10D | SizedReg::R11D | SizedReg::R12D | SizedReg::R13D | SizedReg::R14D | SizedReg::R15D |
                SizedReg::EBP | SizedReg::ESI | SizedReg::EDI | SizedReg::ESP | SizedReg::EIP
                    => 4,
                SizedReg::RAX | SizedReg::RBX | SizedReg::RCX | SizedReg::RDX | SizedReg::R8 | SizedReg::R9 |
                SizedReg::R10 | SizedReg::R11 | SizedReg::R12 | SizedReg::R13 | SizedReg::R14 | SizedReg::R15 |
                SizedReg::RBP | SizedReg::RSI | SizedReg::RDI | SizedReg::RSP | SizedReg::RIP
                    => 8
            }
        }
    }
}

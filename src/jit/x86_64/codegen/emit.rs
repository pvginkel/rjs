#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

/*
 * jit-gen-x86-64.h - Macros for generating x86_64 code.
 *
 * Copyright (C) 2008  Southern Storm Software, Pty Ltd.
 *
 * This file is part of the libjit library.
 *
 * The libjit library is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation, either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * The libjit library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with the libjit library.  If not, see
 * <http://www.gnu.org/licenses/>.
 */

use jit::{Writer, JitFunction};
use std::mem::transmute;
use std::i32;

/*
 * X86_64 64 bit general purpose integer registers.
 */

const X86_64_RAX : u8 = 0;
const X86_64_RCX : u8 = 1;
const X86_64_RDX : u8 = 2;
const X86_64_RBX : u8 = 3;
const X86_64_RSP : u8 = 4;
const X86_64_RBP : u8 = 5;
const X86_64_RSI : u8 = 6;
const X86_64_RDI : u8 = 7;
const X86_64_R8  : u8 = 8;
const X86_64_R9  : u8 = 9;
const X86_64_R10 : u8 = 10;
const X86_64_R11 : u8 = 11;
const X86_64_R12 : u8 = 12;
const X86_64_R13 : u8 = 13;
const X86_64_R14 : u8 = 14;
const X86_64_R15 : u8 = 15;
const X86_64_RIP : u8 = 16;
						/* This register encoding doesn't exist in the */
						/* instructions. It's used for RIP relative encoding. */
const X86_64_NOBASEREG : u8 = 0xFF;

/*
 * X86-64 xmm registers.
 */

const X86_64_XMM0 : u8 = 0;
const X86_64_XMM1 : u8 = 1;
const X86_64_XMM2 : u8 = 2;
const X86_64_XMM3 : u8 = 3;
const X86_64_XMM4 : u8 = 4;
const X86_64_XMM5 : u8 = 5;
const X86_64_XMM6 : u8 = 6;
const X86_64_XMM7 : u8 = 7;
const X86_64_XMM8 : u8 = 8;
const X86_64_XMM9 : u8 = 9;
const X86_64_XMM10 : u8 = 10;
const X86_64_XMM11 : u8 = 11;
const X86_64_XMM12 : u8 = 12;
const X86_64_XMM13 : u8 = 13;
const X86_64_XMM14 : u8 = 14;
const X86_64_XMM15 : u8 = 15;

#[repr(usize)]
#[derive(Copy, Clone, PartialEq)]
pub enum Reg {
    RAX = 0,
    RCX = 1,
    RDX = 2,
    RBX = 3,
    RSP = 4,
    RBP = 5,
    RSI = 6,
    RDI = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    RIP = 16,
    XMM0 = 17,
    XMM1 = 18,
    XMM2 = 19,
    XMM3 = 20,
    XMM4 = 21,
    XMM5 = 22,
    XMM6 = 23,
    XMM7 = 24,
    XMM8 = 25,
    XMM9 = 26,
    XMM10 = 27,
    XMM11 = 28,
    XMM12 = 29,
    XMM13 = 30,
    XMM14 = 31,
    XMM15 = 32,
    NONE = 33,
    NOBASEREG = 34
}

impl Reg {
    pub fn value(self) -> u8 {
        x86_64_reg_map[unsafe { transmute::<_, usize>(self) }]
    }
}

/*
 * Bits in the REX prefix byte.
 */
pub type X86_64_REX_Bits = u8;

pub const X86_64_REX_B : X86_64_REX_Bits = 1;
                        /* 1-bit (high) extension of the ModRM r/m field */
						/* SIB base field, or opcode reg field, thus */
						/* permitting access to 16 registers. */
pub const X86_64_REX_X : X86_64_REX_Bits = 2;
                    	/* 1-bit (high) extension of the SIB index field */
						/* thus permitting access to 16 registers. */
pub const X86_64_REX_R : X86_64_REX_Bits = 4;
                    	/* 1-bit (high) extension of the ModRM reg field, */
						/* thus permitting access to 16 registers. */
pub const X86_64_REX_W : X86_64_REX_Bits = 8;
                    	/* 0 = Default operand size */
						/* 1 = 64 bit operand size */

/*
 * Third part of the opcodes for xmm instructions which are encoded
 * Opcode1: 0xF3 (single precision) or 0xF2 (double precision)
 *          This is handled as a prefix.
 * Opcode2: 0x0F
 */

pub const XMM1_MOV : u8 = 0x10;
pub const XMM1_MOV_REV : u8 = 0x11;
pub const XMM1_ADD : u8 = 0x58;
pub const XMM1_MUL : u8 = 0x59;
pub const XMM1_SUB : u8 = 0x5C;
pub const XMM1_DIV : u8 = 0x5E;

/*
 * Logical opcodes used with packed single and double precision values.
 */

pub const XMM_ANDP : u8 = 0x54;
pub const XMM_ORP : u8 = 0x56;
pub const XMM_XORP : u8 = 0x57;

/*
 * Rounding modes for xmm rounding instructions, the mxcsr register and
 * the fpu control word.
 */

pub const X86_ROUND_NEAREST : u8 = 0x00;		/* Round to the nearest integer */
pub const X86_ROUND_DOWN : u8 = 0x01;		/* Round towards negative infinity */
pub const X86_ROUND_UP : u8 = 0x02;		/* Round towards positive infinity */
pub const X86_ROUND_ZERO : u8= 0x03;		/* Round towards zero (truncate) */

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AluOp {
    Add = 0,
    Or = 1,
    Adc = 2,
    Sbb = 3,
    And = 4,
    Sub = 5,
    XOr = 6,
    Cmp = 7
}

impl AluOp {
    fn value(self) -> u8 {
        unsafe { transmute(self) }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShiftOp {
    Shl = 4,
    Shr = 5,
    Sar = 7,
}

impl ShiftOp {
    fn value(self) -> u8 {
        unsafe { transmute(self) }
    }
}

const x86_cc_unsigned_map : [u8; 12] = [
	0x74, /* eq  */
	0x75, /* ne  */
	0x72, /* lt  */
	0x76, /* le  */
	0x77, /* gt  */
	0x73, /* ge  */
	0x78, /* lz  */
	0x79, /* gez */
	0x7a, /* p   */
	0x7b, /* np  */
	0x70, /* o  */
	0x71, /* no  */
];

const x86_cc_signed_map : [u8; 12] = [
	0x74, /* eq  */
	0x75, /* ne  */
	0x7c, /* lt  */
	0x7e, /* le  */
	0x7f, /* gt  */
	0x7d, /* ge  */
	0x78, /* lz  */
	0x79, /* gez */
	0x7a, /* p   */
	0x7b, /* np  */
	0x70, /* o  */
	0x71, /* no  */
];

const x86_64_reg_map : [u8; 35] = [
    X86_64_RAX,
    X86_64_RCX,
    X86_64_RDX,
    X86_64_RBX,
    X86_64_RSP,
    X86_64_RBP,
    X86_64_RSI,
    X86_64_RDI,
    X86_64_R8,
    X86_64_R9,
    X86_64_R10,
    X86_64_R11,
    X86_64_R12,
    X86_64_R13,
    X86_64_R14,
    X86_64_R15,
    X86_64_RIP,
    X86_64_XMM0,
    X86_64_XMM1,
    X86_64_XMM2,
    X86_64_XMM3,
    X86_64_XMM4,
    X86_64_XMM5,
    X86_64_XMM6,
    X86_64_XMM7,
    X86_64_XMM8,
    X86_64_XMM9,
    X86_64_XMM10,
    X86_64_XMM11,
    X86_64_XMM12,
    X86_64_XMM13,
    X86_64_XMM14,
    X86_64_XMM15,
    0 /* NONE */,
    X86_64_NOBASEREG
];

pub struct Emit {
    inst: Writer
}

impl Emit {
    pub fn new() -> Emit {
        Emit {
            inst: Writer::new()
        }
    }
    
    pub fn build(&mut self) -> JitFunction {
        self.inst.build()
    }
    
    fn address_byte(&mut self, m: u8, o: u8, r: u8) {
        self.inst.push(((((m)&0x03)<<6)|(((o)&0x07)<<3)|(((r)&0x07))));
    }

    fn imm_emit32(&mut self, imm: i32) {
        let imb = unsafe { transmute::<_, [u8; 4]>(imm) };
        self.inst.push(imb [0]);
        self.inst.push(imb [1]);
        self.inst.push(imb [2]);
        self.inst.push(imb [3]);
    }

    // TODO: inst is the offset into the stream!
    fn imm_emit16(&mut self, imm: i32) {
        let imb = unsafe { transmute::<_, [u8; 2]>(imm as i16) };
        self.inst.push(imb [0]);
        self.inst.push(imb [1]);
    }

    fn imm_emit8(&mut self, imm: i32) {
        self.inst.push(imm as u8);
    }

    fn imm_emit64(&mut self, imm: i64) {
        let imb = unsafe { transmute::<_, [u8; 8]>(imm) };
        self.inst.push(imb[0]);
        self.inst.push(imb[1]);
        self.inst.push(imb[2]);
        self.inst.push(imb[3]);
        self.inst.push(imb[4]);
        self.inst.push(imb[5]);
        self.inst.push(imb[6]);
        self.inst.push(imb[7]);
    }
    
    fn imm_emit_max32(&mut self, imm: i32, size: i32) {
        match size {
            1 =>  {
                self.imm_emit8((imm));
            }
            2 => {
                self.imm_emit16((imm));
            }
            4 | 8 => {
                self.imm_emit32((imm));
            }
            _ => jit_assert!()
        }
    }
    
    fn imm_emit_max64(&mut self, imm: i64, size: i32) {
        match size {
            1 => {
                self.imm_emit8((imm) as i32);
            }
            2 => {
                self.imm_emit16((imm) as i32);
            }
            4 => {
                self.imm_emit32((imm) as i32);
            }
            8 => {
                self.imm_emit64((imm));
            }
            _ => jit_assert!()
        }
    }
    
    fn is_imm8(imm: i32) -> bool {
        ((imm) >= -128 && (imm) <= 127)
    }
    
    fn is_imm16(imm: i32) -> bool {
        ((imm) >= -(1<<16) && (imm) <= ((1<<16)-1))
    }
    
    /*
     * Emit the Rex prefix.
     * The natural size is a power of 2 (1, 2, 4 or 8).
     * For accessing the low byte registers DIL, SIL, BPL and SPL we have to
     * generate a Rex prefix with the value 0x40 too.
     * To enable this OR the natural size with 1.
     */
    fn rex(rex_bits: u8) -> u8 {
        (0x40 | (rex_bits))
    }
    
    fn rex_emit(&mut self, width: i32, modrm_reg: Reg, index_reg: Reg, rm_base_opcode_reg: Reg) {
        let __rex_bits =
            if ((width) & 8) != 0 { X86_64_REX_W } else { 0 } |
            if ((modrm_reg.value()) & 8) != 0 { X86_64_REX_R } else { 0 } |
            if ((index_reg.value()) & 8) != 0 { X86_64_REX_X } else { 0 } |
            if ((rm_base_opcode_reg.value()) & 8) != 0 { X86_64_REX_B } else { 0 };
        if((__rex_bits != 0)) {
             self.inst.push(Self::rex(__rex_bits));
        } else if(((width) & 1) != 0 && ((modrm_reg.value() & 4) != 0 || (rm_base_opcode_reg.value() & 4) != 0)) {
             self.inst.push(Self::rex(0));
        }
    }
    
    /*
     * Helper for emitting the rex prefix for opcodes with 64bit default size.
     */
    fn rex_emit64(&mut self, width: i32, modrm_reg: Reg, index_reg: Reg, rm_base_opcode_reg: Reg) {
        self.rex_emit(0, (modrm_reg), (index_reg), (rm_base_opcode_reg));
    }
    
    fn reg_emit(&mut self, r: u8, regno: Reg) {
        self.address_byte (3, (r) & 0x7, (regno.value()) & 0x7);
    }
    
    fn mem_emit(&mut self, r: u8, disp: i32) {
        self.address_byte (0, ((r) & 0x7), 4);
        self.address_byte (0, 4, 5);
        self.imm_emit32((disp));
    }
    
    fn mem64_emit(&mut self, r: u8, disp: i64) {
        self.address_byte (0, ((r) & 0x7), 4);
        self.address_byte (0, 4, 5);
        self.imm_emit64((disp));
    }
    
    fn membase_emit(&mut self, r: u8, basereg: Reg, disp: i32) {
        let basereg = basereg.value();
        
        if((basereg) == X86_64_RIP) {
            self.address_byte(0, ((r) & 0x7), 5);
            self.imm_emit32((disp));
            return;
        }
        
        let r = r & 0x7;

        if ((basereg) == X86_64_RSP) {
            if ((disp) == 0) {
                self.address_byte (0, (r), X86_64_RSP);
                self.address_byte (0, X86_64_RSP, X86_64_RSP);
            } else if (Self::is_imm8((disp))) {
                self.address_byte (1, (r), X86_64_RSP);
                self.address_byte (0, X86_64_RSP, X86_64_RSP);
                self.imm_emit8 ((disp));
            } else {
                self.address_byte (2, (r), X86_64_RSP);
                self.address_byte (0, X86_64_RSP, X86_64_RSP);
                self.imm_emit32 ((disp));
            }
            return;
        }
        
        if ((disp) == 0 && (basereg) != X86_64_RBP) {
            self.address_byte (0, (r), (basereg));
            return;
        }
        
        if (Self::is_imm8((disp))) {
            self.address_byte (1, (r), (basereg));
            self.imm_emit8 ((disp));
        } else {
            self.address_byte (2, (r), (basereg));
            self.imm_emit32 ((disp));
        }
    }
    
    fn memindex_emit(&mut self, r: u8, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        let r = r & 0x7;
        let basereg = basereg.value() & 0x7;
        let indexreg = indexreg.value() & 0x7;
        
        if ((basereg) == X86_64_NOBASEREG) {
            self.address_byte (0, (r), 4);
            self.address_byte ((shift), (indexreg), 5);
            self.imm_emit32 ((disp));
        } else if ((disp) == 0 && (basereg) != X86_64_RBP) {
            self.address_byte (0, (r), 4);
            self.address_byte ((shift), (indexreg), (basereg));
        } else if (Self::is_imm8((disp))) {
            self.address_byte (1, (r), 4);
            self.address_byte ((shift), (indexreg), (basereg));
            self.imm_emit8 ((disp));
        } else {
            self.address_byte (2, (r), 4);
            self.address_byte ((shift), (indexreg), (basereg));
            self.imm_emit32 ((disp));
        }
    }
    
    /*
     * RSP, RBP and the corresponding upper registers (R12 and R13) can't be used
     * for relative addressing without displacement because their codes are used
     * for encoding addressing modes with diplacement.
     * So we do a membase addressing in this case with a zero offset.
     */
    fn regp_emit(&mut self, r: u8, regno: Reg) {
        match regno.value() {
            X86_64_RSP | X86_64_RBP | X86_64_R12 | X86_64_R13 => {
                self.membase_emit((r), (regno), 0);
            }
            _ => {
                self.address_byte(0, ((r) & 0x7), ((regno.value()) & 0x7));
            }
        }
    }
    
    /*
     * Helper to encode an opcode where the encoding is different between
     * 8bit and 16 ... 64 bit width in the following way:
     * 8 bit == opcode given
     * 16 ... 64 bit = opcode given | 0x1
     */
    fn opcode1_emit(&mut self, opc: u8, size: i32) {
        match size {
            1 => {
                self.inst.push((opc));
            }
            2 | 4 | 8 => {
                self.inst.push(((opc) | 0x1));
            }
            _ => jit_assert!()
        }
    }
    
    /*
     * Macros to implement the simple opcodes.
     */
    pub fn alu_reg_reg_size(&mut self, opc: AluOp, dreg: Reg, sreg: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (dreg), Reg::NONE, (sreg));
                self.inst.push((((opc.value())) << 3) + 2);
                self.reg_emit((dreg.value()), (sreg));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (dreg), Reg::NONE, (sreg));
                self.inst.push((((opc.value())) << 3) + 3);
                self.reg_emit((dreg.value()), (sreg));
            }
            _ => {}
        }
    }
    
    pub fn alu_regp_reg_size(&mut self, opc: AluOp, dregp: Reg, sreg: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (sreg), Reg::NONE, (dregp));
                self.inst.push((((opc.value())) << 3));
                self.regp_emit((sreg.value()), (dregp));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (sreg), Reg::NONE, (dregp));
                self.inst.push((((opc.value())) << 3) + 1);
                self.regp_emit((sreg.value()), (dregp));
            }
            _ => {}
        }
    }
    
    pub fn alu_mem_reg_size(&mut self, opc: AluOp, mem: i32, sreg: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (sreg), Reg::NONE, Reg::NONE);
                self.inst.push((((opc.value())) << 3));
                self.mem_emit((sreg.value()), (mem));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (sreg), Reg::NONE, Reg::NONE);
                self.inst.push((((opc.value())) << 3) + 1);
                self.mem_emit((sreg.value()), (mem));
            }
            _ => {}
        }
    }
    
    pub fn alu_membase_reg_size(&mut self, opc: AluOp, basereg: Reg, disp: i32, sreg: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (sreg), Reg::NONE, (basereg));
                self.inst.push((((opc.value())) << 3));
                self.membase_emit((sreg.value()), (basereg), (disp));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (sreg), Reg::NONE, (basereg));
                self.inst.push((((opc.value())) << 3) + 1);
                self.membase_emit((sreg.value()), (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn alu_memindex_reg_size(&mut self, opc: AluOp, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (sreg), (indexreg), (basereg));
                self.inst.push((((opc.value())) << 3));
                self.memindex_emit((sreg.value()), (basereg), (disp), (indexreg), (shift));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (sreg), (indexreg), (basereg));
                self.inst.push((((opc.value())) << 3) + 1);
                self.memindex_emit((sreg.value()), (basereg), (disp), (indexreg), (shift));
            }
            _ => {}
        }
    }
    
    pub fn alu_reg_regp_size(&mut self, opc: AluOp, dreg: Reg, sregp: Reg, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (dreg), Reg::NONE, (sregp));
                self.inst.push((((opc.value())) << 3) + 2);
                self.regp_emit((dreg.value()), (sregp));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (dreg), Reg::NONE, (sregp));
                self.inst.push((((opc.value())) << 3) + 3);
                self.regp_emit((dreg.value()), (sregp));
            }
            _ => {}
        }
    }
    
    pub fn alu_reg_mem_size(&mut self, opc: AluOp, dreg: Reg, mem: i32, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (dreg), Reg::NONE, Reg::NONE);
                self.inst.push((((opc.value())) << 3) + 2);
                self.mem_emit((dreg.value()), (mem));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (dreg), Reg::NONE, Reg::NONE);
                self.inst.push((((opc.value())) << 3) + 3);
                self.mem_emit((dreg.value()), (mem));
            }
            _ => {}
        }
    }
    
    pub fn alu_reg_membase_size(&mut self, opc: AluOp, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (dreg), Reg::NONE, (basereg));
                self.inst.push((((opc.value())) << 3) + 2);
                self.membase_emit((dreg.value()), (basereg), (disp));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (dreg), Reg::NONE, (basereg));
                self.inst.push((((opc.value())) << 3) + 3);
                self.membase_emit((dreg.value()), (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn alu_reg_memindex_size(&mut self, opc: AluOp, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        match size {
            1 => {
                self.rex_emit(size, (dreg), (indexreg), (basereg));
                self.inst.push((((opc.value())) << 3) + 2);
                self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
            }
            2 | 4 | 8 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.rex_emit(size, (dreg), (indexreg), (basereg));
                self.inst.push((((opc.value())) << 3) + 3);
                self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
            }
            _ => {}
        }
    }
    
    /*
     * The immediate value has to be at most 32 bit wide.
     */
    pub fn alu_reg_imm_size(&mut self, opc: AluOp, dreg: Reg, imm: i32, size: i32) {
        if((dreg) == Reg::RAX) {
            match size {
                1 => {
                    self.inst.push((((opc.value())) << 3) + 4);
                    self.imm_emit8((imm));
                }
                2 => {
                    self.inst.push(0x66);
                    self.inst.push((((opc.value())) << 3) + 5);
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push((((opc.value())) << 3) + 5);
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        } else if(Self::is_imm8((imm))) {
            match size {
                1 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (dreg));
                    self.inst.push(0x80);
                }
                2 | 4 | 8 => {
                    if size == 2 {
                        self.inst.push(0x66);
                    }
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (dreg));
                    self.inst.push(0x83);
                }
                _ => {}
            }
            self.reg_emit((opc.value()), (dreg));
            self.imm_emit8((imm));
        } else {
            match size {
                1 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (dreg));
                    self.inst.push(0x80);
                    self.reg_emit((opc.value()), (dreg));
                    self.imm_emit8((imm));
    //                jit_assert!(1);
                }
                2 => {
                    self.inst.push(0x66);
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (dreg));
                    self.inst.push(0x81);
                    self.reg_emit((opc.value()), (dreg));
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (dreg));
                    self.inst.push(0x81);
                    self.reg_emit((opc.value()), (dreg));
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn alu_regp_imm_size(&mut self, opc: AluOp, reg: Reg, imm: i32, size: i32) {
        if(Self::is_imm8((imm))) {
            match size {
                1 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (reg));
                    self.inst.push(0x80);
                }
                2 | 4 | 8 => {
                    if size == 2 {
                        self.inst.push(0x66);
                    }
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (reg));
                    self.inst.push(0x83);
                }
                _ => {}
            }
            self.regp_emit((opc.value()), (reg));
            self.imm_emit8((imm));
        } else {
            match size {
                1 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (reg));
                    self.inst.push(0x80);
                    self.regp_emit((opc.value()), (reg));
                    self.imm_emit8((imm));
    //                jit_assert!(1);
                }
                2 => {
                    self.inst.push(0x66);
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (reg));
                    self.inst.push(0x81);
                    self.regp_emit((opc.value()), (reg));
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit(size, Reg::NONE, Reg::NONE, (reg));
                    self.inst.push(0x81);
                    self.regp_emit((opc.value()), (reg));
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn alu_mem_imm_size(&mut self, opc: AluOp, mem: i32, imm: i32, size: i32) {
        if (Self::is_imm8((imm))) {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push(0x80);
                }
                2 | 4 | 8 => {
                    if size == 2 {
                        self.inst.push(0x66);
                    }
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push(0x83);
                }
                _ => {}
            }
            self.mem_emit((opc.value()), (mem));
            self.imm_emit8((imm));
        } else {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push(0x80);
                    self.mem_emit((opc.value()), (mem));
                    self.imm_emit8((imm));
    //                jit_assert!(1);
                }
                2 => {
                    self.inst.push(0x66);
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push(0x81);
                    self.mem_emit((opc.value()), (mem));
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
                    self.inst.push(0x81);
                    self.mem_emit((opc.value()), (mem));
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn alu_membase_imm_size(&mut self, opc: AluOp, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if (Self::is_imm8((imm))) {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
                    self.inst.push(0x80);
                }
                2 | 4 | 8 => {
                    if size == 2 {
                        self.inst.push(0x66);
                    }
                    self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
                    self.inst.push(0x83);
                }
                _ => {}
            }
            self.membase_emit((opc.value()), (basereg), (disp));
            self.imm_emit8((imm));
        } else {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
                    self.inst.push(0x80);
                    self.membase_emit((opc.value()), (basereg), (disp));
                    self.imm_emit8((imm));
    //                jit_assert!(1);
                }
                2 => {
                    self.inst.push(0x66);
                    self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
                    self.inst.push(0x81);
                    self.membase_emit((opc.value()), (basereg), (disp));
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
                    self.inst.push(0x81);
                    self.membase_emit((opc.value()), (basereg), (disp));
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn alu_memindex_imm_size(&mut self, opc: AluOp, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if (Self::is_imm8((imm))) {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
                    self.inst.push(0x80);
                }
                2 | 4 | 8 => {
                    if size == 2 {
                        self.inst.push(0x66);
                    }
                    self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
                    self.inst.push(0x83);
                }
                _ => {}
            }
            self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
            self.imm_emit8((imm));
        } else {
            match size {
                1 => {
                    self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
                    self.inst.push(0x80);
                    self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
                    self.imm_emit8((imm));
    //                jit_assert!(1);
                }
                2 => {
                    self.inst.push(0x66);
                    self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
                    self.inst.push(0x81);
                    self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
                    self.inst.push(0x81);
                    self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    /*
     * Instructions with one opcode (plus optional r/m)
     */
    
    /*
     * Unary opcodes
     */
    pub fn alu1_reg(&mut self, opc1: u8, r: u8, reg: Reg) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (reg));
        self.inst.push((opc1));
        self.reg_emit((r), (reg));
    }
    
    pub fn alu1_regp(&mut self, opc1: u8, r: u8, regp: Reg) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (regp));
        self.inst.push((opc1));
        self.regp_emit((r), (regp));
    }
    
    pub fn alu1_mem(&mut self, opc1: u8, r: u8, mem: i32) {
        self.inst.push((opc1));
        self.mem_emit((r), (mem));
    }
    
    pub fn alu1_membase(&mut self, opc1: u8, r: u8, basereg: Reg, disp: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.membase_emit((r), (basereg), (disp));
    }
    
    pub fn alu1_memindex(&mut self, opc1: u8, r: u8, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.rex_emit(0, Reg::NONE, (indexreg), (basereg));
        self.inst.push((opc1));
        self.memindex_emit((r), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn alu1_reg_size(&mut self, opc1: u8, r: u8, reg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (reg));
        self.opcode1_emit((opc1), (size));
        self.reg_emit((r), (reg));
    }
    
    pub fn alu1_regp_size(&mut self, opc1: u8, r: u8, regp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (regp));
        self.opcode1_emit((opc1), (size));
        self.regp_emit((r), (regp));
    }
    
    pub fn alu1_mem_size(&mut self, opc1: u8, r: u8, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
        self.opcode1_emit((opc1), (size));
        self.mem_emit((r), (mem));
    }
    
    pub fn alu1_membase_size(&mut self, opc1: u8, r: u8, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
        self.opcode1_emit((opc1), (size));
        self.membase_emit((r), (basereg), (disp));
    }
    
    pub fn alu1_memindex_size(&mut self, opc1: u8, r: u8, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
        self.opcode1_emit((opc1), (size));
        self.memindex_emit((r), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn alu1_reg_reg_size(&mut self, opc1: u8, dreg: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        self.inst.push((opc1));
        self.reg_emit((dreg.value()), (sreg));
    }
    
    pub fn alu1_reg_regp_size(&mut self, opc1: u8, dreg: Reg, sregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        self.inst.push((opc1));
        self.regp_emit((dreg.value()), (sregp));
    }
    
    pub fn alu1_reg_mem_size(&mut self, opc1: u8, dreg: Reg, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        self.inst.push((opc1));
        self.mem_emit((dreg.value()), (mem));
    }
    
    pub fn alu1_reg_membase_size(&mut self, opc1: u8, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    pub fn alu1_reg_memindex_size(&mut self, opc1: u8, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.inst.push((opc1));
        self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn alu2_reg_reg_size(&mut self, opc1: u8, opc2: u8, dreg: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.reg_emit((dreg.value()), (sreg));
    }
    
    pub fn alu2_reg_regp_size(&mut self, opc1: u8, opc2: u8, dreg: Reg, sregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.regp_emit((dreg.value()), (sregp));
    }
    
    pub fn alu2_reg_mem_size(&mut self, opc1: u8, opc2: u8, dreg: Reg, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.mem_emit((dreg.value()), (mem));
    }
    
    pub fn alu2_reg_membase_size(&mut self, opc1: u8, opc2: u8, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    pub fn alu2_reg_memindex_size(&mut self, opc1: u8, opc2: u8, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Group1 general instructions
     */
    pub fn alu_reg_reg(&mut self, opc: AluOp, dreg: Reg, sreg: Reg) {
        self.alu_reg_reg_size((opc), (dreg), (sreg), 8);
    }
    
    pub fn alu_reg_imm(&mut self, opc: AluOp, dreg: Reg, imm: i32) {
        self.alu_reg_imm_size((opc), (dreg), (imm), 8);
    }
    
    /*
     * dec
     */
    pub fn dec_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xfe, 1, (reg), (size));
    }
    
    pub fn dec_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xfe, 1, (regp), (size));
    }
    
    pub fn dec_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xfe, 1, (mem), (size));
    }
    
    pub fn dec_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xfe, 1, (basereg), (disp), (size));
    }
    
    pub fn dec_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xfe, 1, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * div: unsigned division RDX:RAX / operand
     */
    pub fn div_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xf6, 6, (reg), (size));
    }
    
    pub fn div_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xf6, 6, (regp), (size));
    }
    
    pub fn div_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xf6, 6, (mem), (size));
    }
    
    pub fn div_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xf6, 6, (basereg), (disp), (size));
    }
    
    pub fn div_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xf6, 6, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * idiv: signed division RDX:RAX / operand
     */
    pub fn idiv_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xf6, 7, (reg), (size));
    }
    
    pub fn idiv_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xf6, 7, (regp), (size));
    }
    
    pub fn idiv_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xf6, 7, (mem), (size));
    }
    
    pub fn idiv_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xf6, 7, (basereg), (disp), (size));
    }
    
    pub fn idiv_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xf6, 7, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * inc
     */
    pub fn inc_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xfe, 0, (reg), (size));
    }
    
    pub fn inc_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xfe, 0, (regp), (size));
    }
    
    pub fn inc_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xfe, 0, (mem), (size));
    }
    
    pub fn inc_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xfe, 0, (basereg), (disp), (size));
    }
    
    pub fn inc_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xfe, 0, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * mul: multiply RDX:RAX = RAX * operand
     * is_signed == 0 -> unsigned multiplication
     * signed multiplication otherwise.
     */
    pub fn mul_reg_issigned_size(&mut self, reg: Reg, is_signed: bool, size: i32) {
        self.alu1_reg_size(0xf6, if is_signed { 5 } else { 4 }, (reg), (size));
    }
    
    pub fn mul_regp_issigned_size(&mut self, regp: Reg, is_signed: bool, size: i32) {
        self.alu1_regp_size(0xf6, if is_signed { 5 } else { 4 }, (regp), (size));
    }
    
    pub fn mul_mem_issigned_size(&mut self, mem: i32, is_signed: bool, size: i32) {
        self.alu1_mem_size(0xf6, if is_signed { 5 } else { 4 }, (mem), (size));
    }
    
    pub fn mul_membase_issigned_size(&mut self, basereg: Reg, disp: i32, is_signed: bool, size: i32) {
        self.alu1_membase_size(0xf6, if is_signed { 5 } else { 4 }, (basereg), (disp), (size));
    }
    
    pub fn mul_memindex_issigned_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_signed: bool, size: i32) {
        self.alu1_memindex_size(0xf6, if is_signed { 5 } else { 4 }, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * neg 
     */
    pub fn neg_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xf6, 3, (reg), (size));
    }
    
    pub fn neg_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xf6, 3, (regp), (size));
    }
    
    pub fn neg_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xf6, 3, (mem), (size));
    }
    
    pub fn neg_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xf6, 3, (basereg), (disp), (size));
    }
    
    pub fn neg_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xf6, 3, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * not
     */
    pub fn not_reg_size(&mut self, reg: Reg, size: i32) {
        self.alu1_reg_size(0xf6, 2, (reg), (size));
    }
    
    pub fn not_regp_size(&mut self, regp: Reg, size: i32) {
        self.alu1_regp_size(0xf6, 2, (regp), (size));
    }
    
    pub fn not_mem_size(&mut self, mem: i32, size: i32) {
        self.alu1_mem_size(0xf6, 2, (mem), (size));
    }
    
    pub fn not_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.alu1_membase_size(0xf6, 2, (basereg), (disp), (size));
    }
    
    pub fn not_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_memindex_size(0xf6, 2, (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * Note: x86_64_clear_reg () changes the condition code!
     */
    pub fn clear_reg(&mut self, reg: Reg) {
        self.alu_reg_reg_size(AluOp::XOr, (reg), (reg), 4)
    }
    
    /*
     * shift instructions
     */
    pub fn shift_reg_imm_size(&mut self, opc: ShiftOp, dreg: Reg, imm: i32, size: i32) {
        if ((imm) == 1) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
            self.opcode1_emit(0xd0, (size));
            self.reg_emit((opc.value()), (dreg));
        } else {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
            self.opcode1_emit(0xc0, (size));
            self.reg_emit((opc.value()), (dreg));
            self.imm_emit8((imm));
        }
    }
    
    pub fn shift_mem_imm_size(&mut self, opc: ShiftOp, mem: i32, imm: i32, size: i32) {
        if ((imm) == 1) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
            self.opcode1_emit(0xd0, (size));
            self.mem_emit((opc.value()), (mem));
        } else {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
            self.opcode1_emit(0xc0, (size));
            self.mem_emit((opc.value()), (mem));
            self.imm_emit8((imm));
        }
    }
    
    pub fn shift_regp_imm_size(&mut self, opc: ShiftOp, dregp: Reg, imm: i32, size: i32) {
        if ((imm) == 1) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (dregp));
            self.opcode1_emit(0xd0, (size));
            self.regp_emit((opc.value()), (dregp));
        } else {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (dregp));
            self.opcode1_emit(0xc0, (size));
            self.regp_emit((opc.value()), (dregp));
            self.imm_emit8((imm));
        }
    }
    
    pub fn shift_membase_imm_size(&mut self, opc: ShiftOp, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if ((imm) == 1) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
            self.opcode1_emit(0xd0, (size));
            self.membase_emit((opc.value()), (basereg), (disp));
        } else {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
            self.opcode1_emit(0xc0, (size));
            self.membase_emit((opc.value()), (basereg), (disp));
            self.imm_emit8((imm));
        }
    }
    
    pub fn shift_memindex_imm_size(&mut self, opc: ShiftOp, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if ((imm) == 1) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
            self.opcode1_emit(0xd0, (size));
            self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
        } else {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
            self.opcode1_emit(0xc0, (size));
            self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
            self.imm_emit8((imm));
        }
    }
    
    /*
     * shift by the number of bits in %cl
     */
    pub fn shift_reg_size(&mut self, opc: ShiftOp, dreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
        self.opcode1_emit(0xd2, (size));
        self.reg_emit((opc.value()), (dreg));
    }
    
    pub fn shift_mem_size(&mut self, opc: ShiftOp, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
        self.opcode1_emit(0xd2, (size));
        self.mem_emit((opc.value()), (mem));
    }
    
    pub fn shift_regp_size(&mut self, opc: ShiftOp, dregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (dregp));
        self.opcode1_emit(0xd2, (size));
        self.regp_emit((opc.value()), (dregp));
    }
    
    pub fn shift_membase_size(&mut self, opc: ShiftOp, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
        self.opcode1_emit(0xd2, (size));
        self.membase_emit((opc.value()), (basereg), (disp));
    }
    
    pub fn shift_memindex_size(&mut self, opc: ShiftOp, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
        self.opcode1_emit(0xd2, (size));
        self.memindex_emit((opc.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * test: and tha values and set sf, zf and pf according to the result
     */
    pub fn test_reg_imm_size(&mut self, reg: Reg, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (reg));
        if((reg) == Reg::RAX) {
            self.opcode1_emit(0xa8, (size));
        } else {
            self.opcode1_emit(0xf6, (size));
            self.reg_emit(0, (reg));
        }
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn test_regp_imm_size(&mut self, regp: Reg, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (regp));
        self.opcode1_emit(0xf6, (size));
        self.regp_emit(0, (regp));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn test_mem_imm_size(&mut self, mem: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
        self.opcode1_emit(0xf6, (size));
        self.mem_emit(0, (mem));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn test_membase_imm_size(&mut self, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
        self.opcode1_emit(0xf6, (size));
        self.membase_emit(0, (basereg), (disp));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn test_memindex_imm_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
        self.opcode1_emit(0xf6, (size));
        self.memindex_emit(0, (basereg), (disp), (indexreg), (shift));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn test_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, (dreg));
        self.opcode1_emit(0x84, (size));
        self.reg_emit((sreg.value()), (dreg));
    }
    
    pub fn test_regp_reg_size(&mut self, dregp: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, (dregp));
        self.opcode1_emit(0x84, (size));
        self.regp_emit((sreg.value()), (dregp));
    }
    
    pub fn test_mem_reg_size(&mut self, mem: i32, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, Reg::NONE);
        self.opcode1_emit(0x84, (size));
        self.mem_emit((sreg.value()), (mem));
    }
    
    pub fn test_membase_reg_size(&mut self, basereg: Reg, disp: i32, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, (basereg));
        self.opcode1_emit(0x84, (size));
        self.membase_emit((sreg.value()), (basereg), (disp));
    }
    
    pub fn test_memindex_reg_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), (indexreg), (basereg));
        self.opcode1_emit(0x84, (size));
        self.memindex_emit((sreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * imul: signed multiply
     */
    pub fn imul_reg_reg_imm_size(&mut self, dreg: Reg, sreg: Reg, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        if (Self::is_imm8((imm))) {
            self.inst.push(0x6b);
            self.reg_emit((dreg.value()), (sreg));
            self.imm_emit8((imm));
        } else {
            self.inst.push(0x69);
            self.reg_emit((dreg.value()), (sreg));
            match size {
                2 => {
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn imul_reg_regp_imm_size(&mut self, dreg: Reg, sregp: Reg, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        if (Self::is_imm8((imm))) {
            self.inst.push(0x6b);
            self.regp_emit((dreg.value()), (sregp));
            self.imm_emit8((imm));
        } else {
            self.inst.push(0x69);
            self.regp_emit((dreg.value()), (sregp));
            match size {
                2 => {
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn imul_reg_mem_imm_size(&mut self, dreg: Reg, mem: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        if (Self::is_imm8((imm))) {
            self.inst.push(0x6b);
            self.mem_emit((dreg.value()), (mem));
            self.imm_emit8((imm));
        } else {
            self.inst.push(0x69);
            self.mem_emit((dreg.value()), (mem));
            match size {
                2 => {
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn imul_reg_membase_imm_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        if (Self::is_imm8((imm))) {
            self.inst.push(0x6b);
            self.membase_emit((dreg.value()), (basereg), (disp));
            self.imm_emit8((imm));
        } else {
            self.inst.push(0x69);
            self.membase_emit((dreg.value()), (basereg), (disp));
            match size {
                2 => {
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn imul_reg_memindex_imm_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        if (Self::is_imm8((imm))) {
            self.inst.push(0x6b);
            self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
            self.imm_emit8((imm));
        } else {
            self.inst.push(0x69);
            self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
            match size {
                2 => {
                    self.imm_emit16((imm));
                }
                4 | 8 => {
                    self.imm_emit32((imm));
                }
                _ => {}
            }
        }
    }
    
    pub fn imul_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        self.inst.push(0x0F);
        self.inst.push(0xAF);
        self.reg_emit((dreg.value()), (sreg));
    }
    
    pub fn imul_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        self.inst.push(0x0F);
        self.inst.push(0xAF);
        self.regp_emit((dreg.value()), (sregp));
    }
    
    pub fn imul_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        self.inst.push(0x0F);
        self.inst.push(0xAF);
        self.mem_emit((dreg.value()), (mem));
    }
    
    pub fn imul_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.inst.push(0x0F);
        self.inst.push(0xAF);
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    pub fn imul_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.inst.push(0x0F);
        self.inst.push(0xAF);
        self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * cwd, cdq, cqo: sign extend ax to dx (used for div and idiv)
     */
    pub fn cwd(&mut self) {
        self.inst.push(0x66);
        self.inst.push(0x99);
    }
    
    pub fn cdq(&mut self) {
        self.inst.push(0x99);
    }
    
    pub fn cqo(&mut self) {
        self.inst.push(0x48);
        self.inst.push(0x99);
    }
    
    /*
     * Lea instructions
     */
    pub fn lea_mem(&mut self, reg: Reg, mem: i32) {
        self.inst.push(0x8d);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn lea_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
        self.lea_mem((dreg /* & 0x7 */), (mem));
    }
    
    pub fn lea_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.inst.push(0x8d);
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    pub fn lea_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.inst.push(0x8d);
        self.memindex_emit ((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Move instructions.
     */
    pub fn mov_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        self.opcode1_emit(0x8a, (size));
        self.reg_emit(((dreg.value()) & 0x7), (sreg /* & 0x7 */));
    }
    
    pub fn mov_regp_reg_size(&mut self, regp: Reg, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, (regp));
        self.opcode1_emit(0x88, (size));
        self.regp_emit((sreg.value()), (regp));
    }
    
    pub fn mov_membase_reg_size(&mut self, basereg: Reg, disp: i32, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, (basereg));
        self.opcode1_emit(0x88, (size));
        self.membase_emit((sreg.value()), (basereg), (disp));
    }
    
    pub fn mov_memindex_reg_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), (indexreg), (basereg));
        self.opcode1_emit(0x88, (size));
        self.memindex_emit((sreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Using the AX register is the only possibility to address 64bit.
     * All other registers are bound to 32bit values.
     */
    pub fn mov_mem_reg_size(&mut self, mem: i64, sreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (sreg), Reg::NONE, Reg::NONE);
        if ((sreg) == Reg::RAX) {
            self.opcode1_emit(0xa2, (size));
            self.imm_emit64((mem));
        } else {
            self.opcode1_emit(0x88, (size));
            self.address_byte(0, ((sreg.value()) & 0x7), 4);
            self.address_byte(0, 4, 5);
            self.imm_emit32((mem) as i32);
        }
    }
    
    pub fn mov_reg_imm_size(&mut self, dreg: Reg, imm: i64, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
        match size {
            1 => {
                self.inst.push(0xb0 + ((dreg.value()) & 0x7));
                self.imm_emit8((imm) as i32);
            }
            2 => {
                self.inst.push(0xb8 + ((dreg.value()) & 0x7));
                self.imm_emit16((imm) as i32);
            }
            4 => {
                self.inst.push(0xb8 + ((dreg.value()) & 0x7));
                self.imm_emit32((imm) as i32);
            }
            8 => {
                let __x86_64_imm = (imm);
                if (__x86_64_imm >= i32::MIN as i64 && __x86_64_imm <= i32::MAX as i64) {
                    self.inst.push(0xc7);
                    self.reg_emit(0, (dreg));
                    self.imm_emit32((__x86_64_imm) as i32);
                } else {
                    self.inst.push(0xb8 + ((dreg.value()) & 0x7));
                    self.imm_emit64((__x86_64_imm));
                }
            }
            _ => {}
        }
    }
    
    /*
     * Using the AX register is the only possibility to address 64bit.
     * All other registers are bound to 32bit values.
     */
    pub fn mov_reg_mem_size(&mut self, dreg: Reg, mem: i64, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        if ((dreg) == Reg::RAX) {
            self.opcode1_emit(0xa0, (size));
            self.imm_emit64((mem));
        } else {
            self.opcode1_emit(0x8a, (size));
            self.address_byte (0, (dreg.value()), 4);
            self.address_byte (0, 4, 5);
            self.imm_emit32 ((mem) as i32);
        }
    }
    
    pub fn mov_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        self.opcode1_emit(0x8a, (size));
        self.regp_emit((dreg.value()), (sregp));
    }
    
    pub fn mov_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.opcode1_emit(0x8a, (size));
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    
    pub fn mov_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.opcode1_emit(0x8a, (size));
        self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Only 32bit mem and imm values are allowed here.
     * mem is be RIP relative.
     * 32 bit imm will be sign extended to 64 bits for 64 bit size.
     */
    pub fn mov_mem_imm_size(&mut self, mem: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, Reg::NONE);
        self.opcode1_emit(0xc6, (size));
        self.mem_emit(0, (mem));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn mov_regp_imm_size(&mut self, dregp: Reg, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (dregp));
        self.opcode1_emit(0xc6, (size));
        self.regp_emit(0, (dregp));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn mov_membase_imm_size(&mut self, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, Reg::NONE, (basereg));
        self.opcode1_emit(0xc6, (size));
        self.membase_emit(0, (basereg), (disp));
        self.imm_emit_max32((imm), (size));
    }
    
    pub fn mov_memindex_imm_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), Reg::NONE, (indexreg), (basereg));
        self.opcode1_emit(0xc6, (size));
        self.memindex_emit(0, (basereg), (disp), (indexreg), (shift));
        self.imm_emit_max32((imm), (size));
    }
    
    /*
     * Move with sign extension to the given size (signed)
     */
    pub fn movsx8_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        self.alu2_reg_reg_size(0x0f, 0xbe, (dreg), (sreg), (size) | 1);
    }
    
    pub fn movsx8_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.alu2_reg_regp_size(0x0f, 0xbe, (dreg), (sregp), (size));
    }
    
    pub fn movsx8_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.alu2_reg_mem_size(0x0f, 0xbe, (dreg), (mem), (size));
    }
    
    pub fn movsx8_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.alu2_reg_membase_size(0x0f, 0xbe, (dreg), (basereg), (disp), (size));
    }
    
    pub fn movsx8_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu2_reg_memindex_size(0x0f, 0xbe, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    pub fn movsx16_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        self.alu2_reg_reg_size(0x0f, 0xbf, (dreg), (sreg), (size));
    }
    
    pub fn movsx16_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.alu2_reg_regp_size(0x0f, 0xbf, (dreg), (sregp), (size));
    }
    
    pub fn movsx16_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.alu2_reg_mem_size(0x0f, 0xbf, (dreg), (mem), (size));
    }
    
    pub fn movsx16_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.alu2_reg_membase_size(0x0f, 0xbf, (dreg), (basereg), (disp), (size));
    }
    
    pub fn movsx16_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu2_reg_memindex_size(0x0f, 0xbf, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    pub fn movsx32_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        self.alu1_reg_reg_size(0x63, (dreg), (sreg), (size));
    }
    
    pub fn movsx32_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.alu1_reg_regp_size(0x63, (dreg), (sregp), (size));
    }
    
    pub fn movsx32_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.alu1_reg_mem_size(0x63, (dreg), (mem), (size));
    }
    
    pub fn movsx32_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.alu1_reg_membase_size(0x63, (dreg), (basereg), (disp), (size));
    }
    
    pub fn movsx32_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu1_reg_memindex_size(0x63, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * Move with zero extension to the given size (unsigned)
     */
    pub fn movzx8_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        self.alu2_reg_reg_size(0x0f, 0xb6, (dreg), (sreg), (size) | 1);
    }
    
    pub fn movzx8_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.alu2_reg_regp_size(0x0f, 0xb6, (dreg), (sregp), (size));
    }
    
    pub fn movzx8_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.alu2_reg_mem_size(0x0f, 0xb6, (dreg), (mem), (size));
    }
    
    pub fn movzx8_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.alu2_reg_membase_size(0x0f, 0xb6, (dreg), (basereg), (disp), (size));
    }
    
    pub fn movzx8_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu2_reg_memindex_size(0x0f, 0xb6, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    pub fn movzx16_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        self.alu2_reg_reg_size(0x0f, 0xb7, (dreg), (sreg), (size));
    }
    
    pub fn movzx16_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.alu2_reg_regp_size(0x0f, 0xb7, (dreg), (sregp), (size));
    }
    
    pub fn movzx16_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.alu2_reg_mem_size(0x0f, 0xb7, (dreg), (mem), (size));
    }
    
    pub fn movzx16_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.alu2_reg_membase_size(0x0f, 0xb7, (dreg), (basereg), (disp), (size));
    }
    
    pub fn movzx16_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.alu2_reg_memindex_size(0x0f, 0xb7, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cmov: conditional move
     */
    pub fn cmov_reg_reg_size(&mut self, cond: i32, dreg: Reg, sreg: Reg, is_signed: bool, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sreg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
        }
        self.reg_emit((dreg.value()), (sreg));
    }
    
    pub fn cmov_reg_regp_size(&mut self, cond: i32, dreg: Reg, sregp: Reg, is_signed: bool, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (sregp));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
        }
        self.regp_emit((dreg.value()), (sregp));
    }
    
    pub fn cmov_reg_mem_size(&mut self, cond: i32, dreg: Reg, mem: i32, is_signed: bool, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, Reg::NONE);
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
        }
        self.mem_emit((dreg.value()), (mem));
    }
    
    pub fn cmov_reg_membase_size(&mut self, cond: i32, dreg: Reg, basereg: Reg, disp: i32, is_signed: bool, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), Reg::NONE, (basereg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
        }
        self.membase_emit((dreg.value()), (basereg), (disp));
    }
    
    pub fn cmov_reg_memindex_size(&mut self, cond: i32, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_signed: bool, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit((size), (dreg), (indexreg), (basereg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
        }
        self.memindex_emit((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Stack manupulation instructions (push and pop)
     */
    
    /*
     * Push instructions have a default size of 64 bit. mode.
     * There is no way to encode a 32 bit push.
     * So only the sizes 8 and 2 are allowed in 64 bit mode.
     */
    pub fn push_reg_size(&mut self, reg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE, (reg));
        self.inst.push(0x50 + ((reg.value()) & 0x7));
    }
    
    pub fn push_regp_size(&mut self, sregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE, (sregp));
        self.inst.push(0xff);
        self.regp_emit(6, (sregp));
    }
    
    pub fn push_mem_size(&mut self, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE, Reg::NONE);
        self.inst.push(0xff);
        self.mem_emit(6, (mem));
    }
    
    pub fn push_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE, (basereg));
        self.inst.push(0xff);
        self.membase_emit(6, (basereg), (disp));
    }
    
    pub fn push_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, (indexreg), (basereg));
        self.inst.push(0xff);
        self.memindex_emit(6, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * We can push only 32 bit immediate values.
     * The value is sign extended to 64 bit on the stack.
     */
    pub fn push_imm(&mut self, imm: i32) {
        let _imm = (imm);
        if (Self::is_imm8(_imm)) {
            self.inst.push(0x6A);
            self.imm_emit8 ((_imm));
        } else {
            self.inst.push(0x68);
            self.imm_emit32((_imm));
        }
    }
    
    /*
     * Use this version if you need a specific width of the value
     * pushed. The Value on the stack will allways be 64bit wide.
     */
    pub fn push_imm_size(&mut self, imm: i32, size: i32) {
        match size {
            1 => {
                self.inst.push(0x6A);
                self.imm_emit8((imm));
            }
            2 => {
                self.inst.push(0x66);
                self.inst.push(0x68);
                self.imm_emit16((imm));
            }
            4 => {
                self.inst.push(0x68);
                self.imm_emit32((imm));
            }
            _ => {}
        }
    }
    
    
    /*
     * Pop instructions have a default size of 64 bit in 64 bit mode.
     * There is no way to encode a 32 bit pop.
     * So only the sizes 2 and 8 are allowed.
     */
    pub fn pop_reg_size(&mut self, dreg: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64(0, Reg::NONE, Reg::NONE, (dreg));
        self.inst.push(0x58 + ((dreg.value()) & 0x7));
    }
    
    pub fn pop_regp_size(&mut self, dregp: Reg, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE, (dregp));
        self.inst.push(0x8f);
        self.regp_emit(0, (dregp));
    }
    
    pub fn pop_mem_size(&mut self, mem: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.inst.push(0x8f);
        self.mem_emit(0, (mem));
    }
    
    pub fn pop_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, Reg::NONE,(basereg));
        self.inst.push(0x8f);
        self.membase_emit(0, (basereg), (disp));
    }
    
    pub fn pop_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        if ((size) == 2) {
            self.inst.push(0x66);
        }
        self.rex_emit64((size), Reg::NONE, (indexreg), (basereg));
        self.inst.push(0x8f);
        self.memindex_emit(0, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * control flow change instructions
     */
    
    /*
     * call
     */
    
    /*
     * call_imm is a relative call.
     * imm has to be a 32bit offset from the instruction following the
     * call instruction (absolute - (inst + 5)).
     * For offsets greater that 32bit an indirect call (via register)
     * has to be used.
     */
    pub fn call_imm(&mut self, disp: i32) {
        self.inst.push(0xe8);
        self.imm_emit32 ((disp));
    }
    
    pub fn call_reg(&mut self, reg: Reg) {
        self.alu1_reg(0xff, 2, (reg));
    }
    
    pub fn call_regp(&mut self, regp: Reg) {
        self.alu1_regp(0xff, 2, (regp));
    }
    
    /*
     * call_mem is a absolute indirect call.
     * To be able to use this instruction the address must be either
     * in the lowest 2GB or in the highest 2GB addressrange.
     * This is because mem is sign extended to 64bit.
     */
    pub fn call_mem(&mut self, mem: i32) {
        self.alu1_mem(0xff, 2, (mem));
    }
    
    pub fn call_membase(&mut self, basereg: Reg, disp: i32) {
        self.alu1_membase(0xff, 2, (basereg), (disp));
    }
    
    pub fn call_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.alu1_memindex(0xff, 2, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * jmp
     */
    
    /*
     * unconditional relative jumps
     */
    pub fn jmp_imm8(&mut self, disp: i32) {
        self.inst.push(0xEB);
        self.imm_emit8((disp));
    }
    
    pub fn jmp_imm(&mut self, disp: i32) {
        self.inst.push(0xE9);
        self.imm_emit32((disp));
    }
    
    /*
     * unconditional indirect jumps
     */
    pub fn jmp_reg(&mut self, reg: Reg) {
        self.alu1_reg(0xff, 4, (reg));
    }
    
    pub fn jmp_regp(&mut self, regp: Reg) {
        self.alu1_regp(0xff, 4, (regp));
    }
    
    pub fn jmp_mem(&mut self, mem: i32) {
        self.alu1_mem(0xff, 4, (mem));
    }
    
    pub fn jmp_membase(&mut self, basereg: Reg, disp: i32) {
        self.alu1_membase(0xff, 4, (basereg), (disp));
    }
    
    pub fn jmp_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.alu1_memindex(0xff, 4, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Set the low byte in a register to 0x01 if a condition is met
     * or 0x00 otherwise.
     */
    pub fn set_reg(&mut self, cond: i32, dreg: Reg, is_signed: bool) {
        self.rex_emit(1, Reg::NONE, Reg::NONE, (dreg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
        }
        self.reg_emit(0, (dreg));
    }
    
    pub fn set_mem(&mut self, cond: i32, mem: i32, is_signed: bool) {
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
        }
        self.mem_emit(0, (mem));
    }
    
    pub fn set_membase(&mut self, cond: i32, basereg: Reg, disp: i32, is_signed: bool) {
        self.rex_emit(4, Reg::NONE, Reg::NONE, (basereg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
        }
        self.membase_emit(0, (basereg), (disp));
    }
    
    /*
     * ret
     */
    pub fn ret(&mut self) {
        self.inst.push(0xc3);
    }
    
    /*
     * xchg: Exchange values
     */
    pub fn xchg_reg_reg_size(&mut self, dreg: Reg, sreg: Reg, size: i32) {
        if (((size) > 1) && ((dreg) == Reg::RAX || (sreg) == Reg::RAX)) {
            if ((size) == 2) {
                self.inst.push(0x66);
            }
            if ((dreg) == Reg::RAX) {
                self.rex_emit((size), Reg::NONE, Reg::NONE, (sreg));
                self.inst.push((0x90 + (sreg.value() & 0x7)));
            } else {
                self.rex_emit((size), Reg::NONE, Reg::NONE, (dreg));
                self.inst.push((0x90 + (dreg.value() & 0x7)));
            }
        } else {
            if ((size) == 1) {
                self.alu1_reg_reg_size(0x86, (dreg), (sreg), (size));
            } else {
                self.alu1_reg_reg_size(0x87, (dreg), (sreg), (size));
            }
        }
    }
     
    /*
     * XMM instructions
     */
    
    /*
     * xmm instructions with two opcodes
     */
    pub fn xmm2_reg_reg(&mut self, opc1: u8, opc2: u8, r: Reg, reg: Reg) {
        self.rex_emit(0, (r), Reg::NONE, (reg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.reg_emit((r.value()), (reg));
    }
    
    pub fn xmm2_reg_regp(&mut self, opc1: u8, opc2: u8, r: Reg, regp: Reg) {
        self.rex_emit(0, (r), Reg::NONE, (regp));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.regp_emit((r.value()), (regp));
    }
    
    pub fn xmm2_reg_mem(&mut self, opc1: u8, opc2: u8, r: Reg, mem: i32) {
        self.rex_emit(0, (r), Reg::NONE, Reg::NONE);
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.mem_emit((r.value()), (mem));
    }
    
    pub fn xmm2_reg_membase(&mut self, opc1: u8, opc2: u8, r: Reg, basereg: Reg, disp: i32) {
        self.rex_emit(0, (r), Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.membase_emit((r.value()), (basereg), (disp));
    }
    
    pub fn xmm2_reg_memindex(&mut self, opc1: u8, opc2: u8, r: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.rex_emit(0, (r), (indexreg), (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.memindex_emit((r.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * xmm instructions with a prefix and two opcodes
     */
    pub fn p1_xmm2_reg_reg_size(&mut self, p1: u8, opc1: u8, opc2: u8, r: Reg, reg: Reg, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (reg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.reg_emit((r.value()), (reg));
    }
    
    pub fn p1_xmm2_reg_regp_size(&mut self, p1: u8, opc1: u8, opc2: u8, r: Reg, regp: Reg, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (regp));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.regp_emit((r.value()), (regp));
    }
    
    pub fn p1_xmm2_reg_mem_size(&mut self, p1: u8, opc1: u8, opc2: u8, r: Reg, mem: i32, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, Reg::NONE);
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.mem_emit((r.value()), (mem));
    }
    
    pub fn p1_xmm2_reg_membase_size(&mut self, p1: u8, opc1: u8, opc2: u8, r: Reg, basereg: Reg, disp: i32, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.membase_emit((r.value()), (basereg), (disp));
    }
    
    pub fn p1_xmm2_reg_memindex_size(&mut self, p1: u8, opc1: u8, opc2: u8, r: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), (indexreg), (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.memindex_emit((r.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * xmm instructions with a prefix and three opcodes
     */
    pub fn p1_xmm3_reg_reg_size(&mut self, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: Reg, reg: Reg, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (reg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.inst.push((opc3));
        self.reg_emit((r.value()), (reg));
    }
    
    pub fn p1_xmm3_reg_regp_size(&mut self, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: Reg, regp: Reg, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (regp));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.inst.push((opc3));
        self.regp_emit((r.value()), (regp));
    }
    
    pub fn p1_xmm3_reg_mem_size(&mut self, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: Reg, mem: i32, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, Reg::NONE);
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.inst.push((opc3));
        self.mem_emit((r.value()), (mem));
    }
    
    pub fn p1_xmm3_reg_membase_size(&mut self, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: Reg, basereg: Reg, disp: i32, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), Reg::NONE, (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.inst.push((opc3));
        self.membase_emit((r.value()), (basereg), (disp));
    }
    
    pub fn p1_xmm3_reg_memindex_size(&mut self, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.inst.push((p1));
        self.rex_emit((size), (r), (indexreg), (basereg));
        self.inst.push((opc1));
        self.inst.push((opc2));
        self.inst.push((opc3));
        self.memindex_emit((r.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * xmm1: Macro for use of the X86_64_XMM1 enum
     */
    pub fn xmm1_reg_reg(&mut self, opc: u8, dreg: Reg, sreg: Reg, is_double: bool) {
        self.p1_xmm2_reg_reg_size(if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (sreg), 0);
    }
    
    pub fn xmm1_reg_regp(&mut self, opc: u8, dreg: Reg, sregp: Reg, is_double: bool) {
        self.p1_xmm2_reg_regp_size(if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (sregp), 0);
    }
    
    pub fn xmm1_reg_mem(&mut self, opc: u8, dreg: Reg, mem: i32, is_double: bool) {
        self.p1_xmm2_reg_mem_size(if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (mem), 0);
    }
    
    pub fn xmm1_reg_membase(&mut self, opc: u8, dreg: Reg, basereg: Reg, disp: i32, is_double: bool) {
        self.p1_xmm2_reg_membase_size(if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (basereg), (disp), 0);
    }
    
    pub fn xmm1_reg_memindex(&mut self, opc: u8, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_double: bool) {
        self.p1_xmm2_reg_memindex_size(if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Load and store MXCSR register state
     */
    
    /*
     * ldmxcsr: Load MXCSR register
     */
    pub fn ldmxcsr_regp(&mut self, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0xae, Reg::RDX /* 2 */, (sregp));
    }
    
    pub fn ldmxcsr_mem(&mut self, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0xae, Reg::RDX /* 2 */, (mem));
    }
    
    pub fn ldmxcsr_membase(&mut self, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0xae, Reg::RDX /* 2 */, (basereg), (disp));
    }
    
    pub fn ldmxcsr_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0xae, Reg::RDX /* 2 */, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * stmxcsr: Store MXCSR register
     */
    pub fn stmxcsr_regp(&mut self, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0xae, Reg::RBX /* 3 */, (sregp));
    }
    
    pub fn stmxcsr_mem(&mut self, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0xae, Reg::RBX /* 3 */, (mem));
    }
    
    pub fn stmxcsr_membase(&mut self, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0xae, Reg::RBX /* 3 */, (basereg), (disp));
    }
    
    pub fn stmxcsr_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0xae, Reg::RBX /* 3 */, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Move instructions
     */
    
    /*
     * movd: Move doubleword from/to xmm register
     */
    pub fn movd_xreg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x6e, (dreg), (sreg), 4);
    }
    
    pub fn movd_xreg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x6e, (dreg), (mem), 4);
    }
    
    pub fn movd_xreg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x6e, (dreg), (sregp), 4);
    }
    
    pub fn movd_xreg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), 4);
    }
    
    pub fn movd_xreg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), (indexreg), (shift), 4);
    }
    
    pub fn movd_reg_xreg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x7e, (sreg), (dreg), 4);
    }
    
    pub fn movd_mem_xreg(&mut self, mem: i32, sreg: Reg) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x7e, (sreg), (mem), 4);
    }
    
    pub fn movd_regp_xreg(&mut self, dregp: Reg, sreg: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x7e, (sreg), (dregp), 4);
    }
    
    pub fn movd_membase_xreg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), 4);
    }
    
    pub fn movd_memindex_xreg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), (indexreg), (shift), 4);
    }
    
    /*
     * movq: Move quadword from/to xmm register
     */
    pub fn movq_xreg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x6e, (dreg), (sreg), 8);
    }
    
    pub fn movq_xreg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x6e, (dreg), (mem), 8);
    }
    
    pub fn movq_xreg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x6e, (dreg), (sregp), 8);
    }
    
    pub fn movq_xreg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), 8);
    }
    
    pub fn movq_xreg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), (indexreg), (shift), 8);
    }
    
    pub fn movq_reg_xreg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x7e, (sreg), (dreg), 8);
    }
    
    pub fn movq_mem_xreg(&mut self, mem: i32, sreg: Reg) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x7e, (sreg), (mem), 8);
    }
    
    pub fn movq_regp_xreg(&mut self, dregp: Reg, sreg: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x7e, (sreg), (dregp), 8);
    }
    
    pub fn movq_membase_xreg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), 8);
    }
    
    pub fn movq_memindex_xreg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), (indexreg), (shift), 8);
    }
    
    /*
     * movaps: Move aligned quadword (16 bytes)
     */
    pub fn movaps_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x28, (dreg), (sreg));
    }
    
    pub fn movaps_regp_reg(&mut self, dregp: Reg, sreg: Reg) {
        self.xmm2_reg_regp(0x0f, 0x29, (sreg), (dregp));
    }
    
    pub fn movaps_mem_reg(&mut self, mem: i32, sreg: Reg) {
        self.xmm2_reg_mem(0x0f, 0x29, (sreg), (mem));
    }
    
    pub fn movaps_membase_reg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.xmm2_reg_membase(0x0f, 0x29, (sreg), (basereg), (disp));
    }
    
    pub fn movaps_memindex_reg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.xmm2_reg_memindex(0x0f, 0x29, (sreg), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn movaps_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x28, (dreg), (sregp));
    }
    
    pub fn movaps_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x28, (dreg), (mem));
    }
    
    pub fn movaps_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x28, (dreg), (basereg), (disp));
    }
    
    pub fn movaps_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x28, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * movups: Move unaligned quadword (16 bytes)
     */
    pub fn movups_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x10, (dreg), (sreg));
    }
    
    pub fn movups_regp_reg(&mut self, dregp: Reg, sreg: Reg) {
        self.xmm2_reg_regp(0x0f, 0x11, (sreg), (dregp));
    }
    
    pub fn movups_mem_reg(&mut self, mem: i32, sreg: Reg) {
        self.xmm2_reg_mem(0x0f, 0x11, (sreg), (mem));
    }
    
    pub fn movups_membase_reg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.xmm2_reg_membase(0x0f, 0x11, (sreg), (basereg), (disp));
    }
    
    pub fn movups_memindex_reg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.xmm2_reg_memindex(0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn movups_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x10, (dreg), (sregp));
    }
    
    pub fn movups_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x10, (dreg), (mem));
    }
    
    pub fn movups_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x10, (dreg), (basereg), (disp));
    }
    
    pub fn movups_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * movsd: Move scalar double (64bit float)
     */
    pub fn movsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x10, (dreg), (sreg), 0);
    }
    
    pub fn movsd_regp_reg(&mut self, dregp: Reg, sreg: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x11, (sreg), (dregp), 0);
    }
    
    pub fn movsd_mem_reg(&mut self, mem: i32, sreg: Reg) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x11, (sreg), (mem), 0);
    }
    
    pub fn movsd_membase_reg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x11, (sreg), (basereg), (disp), 0);
    }
    
    pub fn movsd_memindex_reg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    pub fn movsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x10, (dreg), (sregp), 0);
    }
    
    pub fn movsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x10, (dreg), (mem), 0);
    }
    
    pub fn movsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x10, (dreg), (basereg), (disp), 0);
    }
    
    pub fn movsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * movss: Move scalar single (32bit float)
     */
    pub fn movss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x10, (dreg), (sreg), 0);
    }
    
    pub fn movss_regp_reg(&mut self, dregp: Reg, sreg: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x11, (sreg), (dregp), 0);
    }
    
    pub fn movss_mem_reg(&mut self, mem: i32, sreg: Reg) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x11, (sreg), (mem), 0);
    }
    
    pub fn movss_membase_reg(&mut self, basereg: Reg, disp: i32, sreg: Reg) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x11, (sreg), (basereg), (disp), 0);
    }
    
    pub fn movss_memindex_reg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, sreg: Reg) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    pub fn movss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x10, (dreg), (sregp), 0);
    }
    
    pub fn movss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x10, (dreg), (mem), 0);
    }
    
    pub fn movss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x10, (dreg), (basereg), (disp), 0);
    }
    
    pub fn movss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Conversion opcodes
     */
    
    /*
     * cvtsi2ss: Convert signed integer to float32
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvtsi2ss_reg_reg_size(&mut self, dxreg: Reg, sreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x2a, (dxreg), (sreg), (size));
    }
    
    pub fn cvtsi2ss_reg_regp_size(&mut self, dxreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x2a, (dxreg), (sregp), (size));
    }
    
    pub fn cvtsi2ss_reg_mem_size(&mut self, dxreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x2a, (dxreg), (mem), (size));
    }
    
    pub fn cvtsi2ss_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x2a, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvtsi2ss_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x2a, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvtsi2sd: Convert signed integer to float64
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvtsi2sd_reg_reg_size(&mut self, dxreg: Reg, sreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x2a, (dxreg), (sreg), (size));
    }
    
    pub fn cvtsi2sd_reg_regp_size(&mut self, dxreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x2a, (dxreg), (sregp), (size));
    }
    
    pub fn cvtsi2sd_reg_mem_size(&mut self, dxreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x2a, (dxreg), (mem), (size));
    }
    
    pub fn cvtsi2sd_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x2a, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvtsi2sd_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x2a, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvtss2si: Convert float32.to a signed integer using the rounding mode
     * in the mxcsr register
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvtss2si_reg_reg_size(&mut self, dreg: Reg, sxreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x2d, (dreg), (sxreg), (size));
    }
    
    pub fn cvtss2si_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x2d, (dreg), (sregp), (size));
    }
    
    pub fn cvtss2si_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x2d, (dreg), (mem), (size));
    }
    
    pub fn cvtss2si_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x2d, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvtss2si_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x2d, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvttss2si: Convert float32.to a signed integer using the truncate rounding mode.
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvttss2si_reg_reg_size(&mut self, dreg: Reg, sxreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x2c, (dreg), (sxreg), (size));
    }
    
    pub fn cvttss2si_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x2c, (dreg), (sregp), (size));
    }
    
    pub fn cvttss2si_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x2c, (dreg), (mem), (size));
    }
    
    pub fn cvttss2si_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x2c, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvttss2si_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x2c, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvtsd2si: Convert float64 to a signed integer using the rounding mode
     * in the mxcsr register
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvtsd2si_reg_reg_size(&mut self, dreg: Reg, sxreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x2d, (dreg), (sxreg), (size));
    }
    
    pub fn cvtsd2si_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x2d, (dreg), (sregp), (size));
    }
    
    pub fn cvtsd2si_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x2d, (dreg), (mem), (size));
    }
    
    pub fn cvtsd2si_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x2d, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvtsd2si_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x2d, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvttsd2si: Convert float64 to a signed integer using the truncate rounding mode.
     * The size is the size of the integer value (4 or 8)
     */
    pub fn cvttsd2si_reg_reg_size(&mut self, dreg: Reg, sxreg: Reg, size: i32) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x2c, (dreg), (sxreg), (size));
    }
    
    pub fn cvttsd2si_reg_regp_size(&mut self, dreg: Reg, sregp: Reg, size: i32) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x2c, (dreg), (sregp), (size));
    }
    
    pub fn cvttsd2si_reg_mem_size(&mut self, dreg: Reg, mem: i32, size: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x2c, (dreg), (mem), (size));
    }
    
    pub fn cvttsd2si_reg_membase_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, size: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x2c, (dreg), (basereg), (disp), (size));
    }
    
    pub fn cvttsd2si_reg_memindex_size(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x2c, (dreg), (basereg), (disp), (indexreg), (shift), (size));
    }
    
    /*
     * cvtss2sd: Convert float32 to float64
     */
    pub fn cvtss2sd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x5a, (dreg), (sreg), 0);
    }
    
    pub fn cvtss2sd_reg_regp(&mut self, dxreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x5a, (dxreg), (sregp), 0);
    }
    
    pub fn cvtss2sd_reg_mem(&mut self, dxreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x5a, (dxreg), (mem), 0);
    }
    
    pub fn cvtss2sd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x5a, (dreg), (basereg), (disp), 0);
    }
    
    pub fn cvtss2sd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x5a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * cvtsd2ss: Convert float64 to float32
     */
    pub fn cvtsd2ss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x5a, (dreg), (sreg), 0);
    }
    
    pub fn cvtsd2ss_reg_regp(&mut self, dxreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x5a, (dxreg), (sregp), 0);
    }
    
    pub fn cvtsd2ss_reg_mem(&mut self, dxreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x5a, (dxreg), (mem), 0);
    }
    
    pub fn cvtsd2ss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x5a, (dreg), (basereg), (disp), 0);
    }
    
    pub fn cvtsd2ss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x5a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Compare opcodes
     */
    
    /*
     * comiss: Compare ordered scalar single precision values
     */
    pub fn comiss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x2f, (dreg), (sreg));
    }
    
    pub fn comiss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x2f, (dreg), (sregp));
    }
    
    pub fn comiss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x2f, (dreg), (mem));
    }
    
    pub fn comiss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x2f, (dreg), (basereg), (disp));
    }
    
    pub fn comiss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x2f, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * comisd: Compare ordered scalar double precision values
     */
    pub fn comisd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x2f, (dreg), (sreg), 0);
    }
    
    pub fn comisd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x2f, (dreg), (sregp), 0);
    }
    
    pub fn comisd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x2f, (dreg), (mem), 0);
    }
    
    pub fn comisd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x2f, (dreg), (basereg), (disp), 0);
    }
    
    pub fn comisd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x2f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * ucomiss: Compare unordered scalar single precision values
     */
    pub fn ucomiss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x2e, (dreg), (sreg));
    }
    
    pub fn ucomiss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x2e, (dreg), (sregp));
    }
    
    pub fn ucomiss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x2e, (dreg), (mem));
    }
    
    pub fn ucomiss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x2e, (dreg), (basereg), (disp));
    }
    
    pub fn ucomiss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x2e, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * ucomisd: Compare unordered scalar double precision values
     */
    pub fn ucomisd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x2e, (dreg), (sreg), 0);
    }
    
    pub fn ucomisd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x2e, (dreg), (sregp), 0);
    }
    
    pub fn ucomisd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x2e, (dreg), (mem), 0);
    }
    
    pub fn ucomisd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x2e, (dreg), (basereg), (disp), 0);
    }
    
    pub fn ucomisd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x2e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Arithmetic opcodes
     */
    
    /*
     * addss: Add scalar single precision float values
     */
    pub fn addss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x58, (dreg), (sreg), 0);
    }
    
    pub fn addss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x58, (dreg), (sregp), 0);
    }
    
    pub fn addss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x58, (dreg), (mem), 0);
    }
    
    pub fn addss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x58, (dreg), (basereg), (disp), 0);
    }
    
    pub fn addss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x58, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * subss: Substract scalar single precision float values
     */
    pub fn subss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x5c, (dreg), (sreg), 0);
    }
    
    pub fn subss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x5c, (dreg), (sregp), 0);
    }
    
    pub fn subss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x5c, (dreg), (mem), 0);
    }
    
    pub fn subss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x5c, (dreg), (basereg), (disp), 0);
    }
    
    pub fn subss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x5c, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * mulss: Multiply scalar single precision float values
     */
    pub fn mulss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x59, (dreg), (sreg), 0);
    }
    
    pub fn mulss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x59, (dreg), (sregp), 0);
    }
    
    pub fn mulss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x59, (dreg), (mem), 0);
    }
    
    pub fn mulss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x59, (dreg), (basereg), (disp), 0);
    }
    
    pub fn mulss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x59, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * divss: Divide scalar single precision float values
     */
    pub fn divss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x5e, (dreg), (sreg), 0);
    }
    
    pub fn divss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x5e, (dreg), (sregp), 0);
    }
    
    pub fn divss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x5e, (dreg), (mem), 0);
    }
    
    pub fn divss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x5e, (dreg), (basereg), (disp), 0);
    }
    
    pub fn divss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x5e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Macros for the logical operations with packed single precision values.
     */
    pub fn plops_reg_reg(&mut self, op: u8, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, (op), (dreg), (sreg));
    }
    
    pub fn plops_reg_regp(&mut self, op: u8, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, (op), (dreg), (sregp));
    }
    
    pub fn plops_reg_mem(&mut self, op: u8, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, (op), (dreg), (mem));
    }
    
    pub fn plops_reg_membase(&mut self, op: u8, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, (op), (dreg), (basereg), (disp));
    }
    
    pub fn plops_reg_memindex(&mut self, op: u8, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, (op), (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * andps: And
     */
    pub fn andps_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x54, (dreg), (sreg));
    }
    
    pub fn andps_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x54, (dreg), (sregp));
    }
    
    pub fn andps_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x54, (dreg), (mem));
    }
    
    pub fn andps_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x54, (dreg), (basereg), (disp));
    }
    
    pub fn andps_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x54, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * orps: Or
     */
    pub fn orps_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x56, (dreg), (sreg));
    }
    
    pub fn orps_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x56, (dreg), (sregp));
    }
    
    pub fn orps_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x56, (dreg), (mem));
    }
    
    pub fn orps_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x56, (dreg), (basereg), (disp));
    }
    
    pub fn orps_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x56, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * xorps: Xor
     */
    pub fn xorps_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.xmm2_reg_reg(0x0f, 0x57, (dreg), (sreg));
    }
    
    pub fn xorps_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.xmm2_reg_regp(0x0f, 0x57, (dreg), (sregp));
    }
    
    pub fn xorps_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.xmm2_reg_mem(0x0f, 0x57, (dreg), (mem));
    }
    
    pub fn xorps_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.xmm2_reg_membase(0x0f, 0x57, (dreg), (basereg), (disp));
    }
    
    pub fn xorps_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.xmm2_reg_memindex(0x0f, 0x57, (dreg), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * maxss: Maximum value
     */
    pub fn maxss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x5f, (dreg), (sreg), 0);
    }
    
    pub fn maxss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x5f, (dreg), (sregp), 0);
    }
    
    pub fn maxss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x5f, (dreg), (mem), 0);
    }
    
    pub fn maxss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x5f, (dreg), (basereg), (disp), 0);
    }
    
    pub fn maxss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x5f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * minss: Minimum value
     */
    pub fn minss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x5d, (dreg), (sreg), 0);
    }
    
    pub fn minss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x5d, (dreg), (sregp), 0);
    }
    
    pub fn minss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x5d, (dreg), (mem), 0);
    }
    
    pub fn minss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x5d, (dreg), (basereg), (disp), 0);
    }
    
    pub fn minss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x5d, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * sqrtss: Square root
     */
    pub fn sqrtss_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf3, 0x0f, 0x51, (dreg), (sreg), 0);
    }
    
    pub fn sqrtss_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf3, 0x0f, 0x51, (dreg), (sregp), 0);
    }
    
    pub fn sqrtss_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf3, 0x0f, 0x51, (dreg), (mem), 0);
    }
    
    pub fn sqrtss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf3, 0x0f, 0x51, (dreg), (basereg), (disp), 0);
    }
    
    pub fn sqrtss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf3, 0x0f, 0x51, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    
    /*
     * Macros for the logical operations with packed double precision values.
     */
    pub fn plopd_reg_reg(&mut self, op: u8, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, (op), (dreg), (sreg), 0);
    }
    
    pub fn plopd_reg_regp(&mut self, op: u8, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, (op), (dreg), (sregp), 0);
    }
    
    pub fn plopd_reg_mem(&mut self, op: u8, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, (op), (dreg), (mem), 0);
    }
    
    pub fn plopd_reg_membase(&mut self, op: u8, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, (op), (dreg), (basereg), (disp), 0);
    }
    
    pub fn plopd_reg_memindex(&mut self, op: u8, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, (op), (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * addsd: Add scalar double precision float values
     */
    pub fn addsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x58, (dreg), (sreg), 0);
    }
    
    pub fn addsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x58, (dreg), (sregp), 0);
    }
    
    pub fn addsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x58, (dreg), (mem), 0);
    }
    
    pub fn addsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x58, (dreg), (basereg), (disp), 0);
    }
    
    pub fn addsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x58, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * subsd: Substract scalar double precision float values
     */
    pub fn subsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x5c, (dreg), (sreg), 0);
    }
    
    pub fn subsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x5c, (dreg), (sregp), 0);
    }
    
    pub fn subsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x5c, (dreg), (mem), 0);
    }
    
    pub fn subsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x5c, (dreg), (basereg), (disp), 0);
    }
    
    pub fn subsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x5c, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * mulsd: Multiply scalar double precision float values
     */
    pub fn mulsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x59, (dreg), (sreg), 0);
    }
    
    pub fn mulsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x59, (dreg), (sregp), 0);
    }
    
    pub fn mulsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x59, (dreg), (mem), 0);
    }
    
    pub fn mulsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x59, (dreg), (basereg), (disp), 0);
    }
    
    pub fn mulsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x59, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * divsd: Divide scalar double precision float values
     */
    pub fn divsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x5e, (dreg), (sreg), 0);
    }
    
    pub fn divsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x5e, (dreg), (sregp), 0);
    }
    
    pub fn divsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x5e, (dreg), (mem), 0);
    }
    
    pub fn divsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x5e, (dreg), (basereg), (disp), 0);
    }
    
    pub fn divsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x5e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * andpd: And
     */
    pub fn andpd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x54, (dreg), (sreg), 0);
    }
    
    pub fn andpd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x54, (dreg), (sregp), 0);
    }
    
    pub fn andpd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x54, (dreg), (mem), 0);
    }
    
    pub fn andpd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x54, (dreg), (basereg), (disp), 0);
    }
    
    pub fn andpd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x54, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * orpd: Or
     */
    pub fn orpd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x56, (dreg), (sreg), 0);
    }
    
    pub fn orpd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x56, (dreg), (sregp), 0);
    }
    
    pub fn orpd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x56, (dreg), (mem), 0);
    }
    
    pub fn orpd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x56, (dreg), (basereg), (disp), 0);
    }
    
    pub fn orpd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x56, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * xorpd: Xor
     */
    pub fn xorpd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0x66, 0x0f, 0x57, (dreg), (sreg), 0);
    }
    
    pub fn xorpd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0x66, 0x0f, 0x57, (dreg), (sregp), 0);
    }
    
    pub fn xorpd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0x66, 0x0f, 0x57, (dreg), (mem), 0);
    }
    
    pub fn xorpd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0x66, 0x0f, 0x57, (dreg), (basereg), (disp), 0);
    }
    
    pub fn xorpd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0x66, 0x0f, 0x57, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * maxsd: Maximum value
     */
    pub fn maxsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x5f, (dreg), (sreg), 0);
    }
    
    pub fn maxsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x5f, (dreg), (sregp), 0);
    }
    
    pub fn maxsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x5f, (dreg), (mem), 0);
    }
    
    pub fn maxsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x5f, (dreg), (basereg), (disp), 0);
    }
    
    pub fn maxsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x5f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * minsd: Minimum value
     */
    pub fn minsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x5d, (dreg), (sreg), 0);
    }
    
    pub fn minsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x5d, (dreg), (sregp), 0);
    }
    
    pub fn minsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x5d, (dreg), (mem), 0);
    }
    
    pub fn minsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x5d, (dreg), (basereg), (disp), 0);
    }
    
    pub fn minsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x5d, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * sqrtsd: Square root
     */
    pub fn sqrtsd_reg_reg(&mut self, dreg: Reg, sreg: Reg) {
        self.p1_xmm2_reg_reg_size(0xf2, 0x0f, 0x51, (dreg), (sreg), 0);
    }
    
    pub fn sqrtsd_reg_regp(&mut self, dreg: Reg, sregp: Reg) {
        self.p1_xmm2_reg_regp_size(0xf2, 0x0f, 0x51, (dreg), (sregp), 0);
    }
    
    pub fn sqrtsd_reg_mem(&mut self, dreg: Reg, mem: i32) {
        self.p1_xmm2_reg_mem_size(0xf2, 0x0f, 0x51, (dreg), (mem), 0);
    }
    
    pub fn sqrtsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32) {
        self.p1_xmm2_reg_membase_size(0xf2, 0x0f, 0x51, (dreg), (basereg), (disp), 0);
    }
    
    pub fn sqrtsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.p1_xmm2_reg_memindex_size(0xf2, 0x0f, 0x51, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    }
    
    /*
     * Rounding: Available in SSE 4.1 only
     */
    
    /*
     * roundss: Round scalar single precision value
     */
    pub fn roundss_reg_reg(&mut self, dreg: Reg, sreg: Reg, mode: i32) {
        self.p1_xmm3_reg_reg_size(0x66, 0x0f, 0x3a, 0x0a, (dreg), (sreg), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundss_reg_regp(&mut self, dreg: Reg, sregp: Reg, mode: i32) {
        self.p1_xmm3_reg_regp_size(0x66, 0x0f, 0x3a, 0x0a, (dreg), (sregp), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundss_reg_mem(&mut self, dreg: Reg, mem: i32, mode: i32) {
        self.p1_xmm3_reg_mem_size(0x66, 0x0f, 0x3a, 0x0a, (dreg), (mem), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundss_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32, mode: i32) {
        self.p1_xmm3_reg_membase_size(0x66, 0x0f, 0x3a, 0x0a, (dreg), (basereg), (disp), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundss_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, mode: i32) {
        self.p1_xmm3_reg_memindex_size(0x66, 0x0f, 0x3a, 0x0a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
        self.imm_emit8((mode));
    }
    
    /*
     * roundsd: Round scalar double precision value
     */
    pub fn roundsd_reg_reg(&mut self, dreg: Reg, sreg: Reg, mode: i32) {
        self.p1_xmm3_reg_reg_size(0x66, 0x0f, 0x3a, 0x0b, (dreg), (sreg), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundsd_reg_regp(&mut self, dreg: Reg, sregp: Reg, mode: i32) {
        self.p1_xmm3_reg_regp_size(0x66, 0x0f, 0x3a, 0x0b, (dreg), (sregp), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundsd_reg_mem(&mut self, dreg: Reg, mem: i32, mode: i32) {
        self.p1_xmm3_reg_mem_size(0x66, 0x0f, 0x3a, 0x0b, (dreg), (mem), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundsd_reg_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32, mode: i32) {
        self.p1_xmm3_reg_membase_size(0x66, 0x0f, 0x3a, 0x0b, (dreg), (basereg), (disp), 0);
        self.imm_emit8((mode));
    }
    
    pub fn roundsd_reg_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, mode: i32) {
        self.p1_xmm3_reg_memindex_size(0x66, 0x0f, 0x3a, 0x0b, (dreg), (basereg), (disp), (indexreg), (shift), 0);
        self.imm_emit8((mode));
    }
    
    /*
     * Clear xmm register
     */
    pub fn clear_xreg(&mut self, reg: Reg) {
        self.xorps_reg_reg((reg), (reg));
    }
    
    /*
     * fpu instructions
     */
    
    /*
     * fld
     */
    
    pub fn fld_regp_size(&mut self, sregp: Reg, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (sregp));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.regp_emit(0, (sregp));
            }
            8 => {
                self.inst.push(0xdd);
                self.regp_emit(0, (sregp));
            }
            10 => {
                self.inst.push(0xdb);
                self.regp_emit(5, (sregp));
            }
            _ => {}
        }
    }
    
    pub fn fld_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            4 => {
                self.inst.push(0xd9);
                self.mem_emit(0, (mem));
            }
            8 => {
                self.inst.push(0xdd);
                self.mem_emit(0, (mem));
            }
            10 => {
                self.inst.push(0xdb);
                self.mem_emit(5, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fld_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.membase_emit(0, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdd);
                self.membase_emit(0, (basereg), (disp));
            }
            10 => {
                self.inst.push(0xdb);
                self.membase_emit(5, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn fld_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.rex_emit(0, Reg::NONE, (indexreg), (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.memindex_emit(0, (basereg), (disp), (indexreg), (shift));
            }
            8 => {
                self.inst.push(0xdd);
                self.memindex_emit(0, (basereg), (disp), (indexreg), (shift));
            }
            10 => {
                self.inst.push(0xdb);
                self.memindex_emit(5, (basereg), (disp), (indexreg), (shift));
            }
            _ => {}
        }
    }
    
    /*
     * fild: Load an integer and convert it to long double
     */
    pub fn fild_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            2 => {
                self.inst.push(0xdf);
                self.mem_emit(0, (mem));
            }
            4 => {
                self.inst.push(0xdb);
                self.mem_emit(0, (mem));
            }
            8 => {
                self.inst.push(0xdf);
                self.mem_emit(5, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fild_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            2 => {
                self.inst.push(0xdf);
                self.membase_emit(0, (basereg), (disp));
            }
            4 => {
                self.inst.push(0xdb);
                self.membase_emit(0, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdf);
                self.membase_emit(5, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    /*
     * fst: Store fpu register to memory (only float32 and float64 allowed)
     */
    
    pub fn fst_regp_size(&mut self, sregp: Reg, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (sregp));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.regp_emit(2, (sregp));
            }
            8 => {
                self.inst.push(0xdd);
                self.regp_emit(2, (sregp));
            }
            _ => {}
        }
    }
    
    pub fn fst_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            4 => {
                self.inst.push(0xd9);
                self.mem_emit(2, (mem));
            }
            8 => {
                self.inst.push(0xdd);
                self.mem_emit(2, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fst_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.membase_emit(2, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdd);
                self.membase_emit(2, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn fst_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.rex_emit(0, Reg::NONE, (indexreg), (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.memindex_emit(2, (basereg), (disp), (indexreg), (shift));
            }
            8 => {
                self.inst.push(0xdd);
                self.memindex_emit(2, (basereg), (disp), (indexreg), (shift));
            }
            _ => {}
        }
    }
    
    /*
     * fstp: store top fpu register to memory and pop it from the fpu stack
     */
    pub fn fstp_regp_size(&mut self, sregp: Reg, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (sregp));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.regp_emit(3, (sregp));
            }
            8 => {
                self.inst.push(0xdd);
                self.regp_emit(3, (sregp));
            }
            10 => {
                self.inst.push(0xdb);
                self.regp_emit(7, (sregp));
            }
            _ => {}
        }
    }
    
    pub fn fstp_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            4 => {
                self.inst.push(0xd9);
                self.mem_emit(3, (mem));
            }
            8 => {
                self.inst.push(0xdd);
                self.mem_emit(3, (mem));
            }
            10 => {
                self.inst.push(0xdb);
                self.mem_emit(7, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fstp_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.membase_emit(3, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdd);
                self.membase_emit(3, (basereg), (disp));
            }
            10 => {
                self.inst.push(0xdb);
                self.membase_emit(7, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn fstp_memindex_size(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        self.rex_emit(0, Reg::NONE, (indexreg), (basereg));
        match size {
            4 => {
                self.inst.push(0xd9);
                self.memindex_emit(3, (basereg), (disp), (indexreg), (shift));
            }
            8 => {
                self.inst.push(0xdd);
                self.memindex_emit(3, (basereg), (disp), (indexreg), (shift));
            }
            10 => {
                self.inst.push(0xdb);
                self.memindex_emit(7, (basereg), (disp), (indexreg), (shift));
            }
            _ => {}
        }
    }
    
    /*
     * fistp: Convert long double to integer
     */
    pub fn fistp_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            2 => {
                self.inst.push(0xdf);
                self.mem_emit(3, (mem));
            }
            4 => {
                self.inst.push(0xdb);
                self.mem_emit(3, (mem));
            }
            8 => {
                self.inst.push(0xdf);
                self.mem_emit(7, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fistp_regp_size(&mut self, dregp: Reg, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (dregp));
        match size {
            2 => {
                self.inst.push(0xdf);
                self.regp_emit(3, (dregp));
            }
            4 => {
                self.inst.push(0xdb);
                self.regp_emit(3, (dregp));
            }
            8 => {
                self.inst.push(0xdf);
                self.regp_emit(7, (dregp));
            }
            _ => {}
        }
    }
    
    pub fn fistp_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            2 => {
                self.inst.push(0xdf);
                self.membase_emit(3, (basereg), (disp));
            }
            4 => {
                self.inst.push(0xdb);
                self.membase_emit(3, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdf);
                self.membase_emit(7, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    /*
     * frndint: Round st(0) to integer according to the rounding mode set in the fpu control word.
     */
    pub fn frndint(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xfc);
    }
    
    /*
     * fisttp: Convert long double to integer using truncation as rounding mode Available in SSE 3 only
     */
    pub fn fisttp_regp_size(&mut self, dregp: Reg, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (dregp));
        match size {
            2 => {
                self.inst.push(0xdf);
                self.regp_emit(1, (dregp));
            }
            4 => {
                self.inst.push(0xdb);
                self.regp_emit(1, (dregp));
            }
            8 => {
                self.inst.push(0xdd);
                self.regp_emit(1, (dregp));
            }
            _ => {}
        }
    }
    
    pub fn fisttp_mem_size(&mut self, mem: i32, size: i32) {
        match size {
            2 => {
                self.inst.push(0xdf);
                self.mem_emit(1, (mem));
            }
            4 => {
                self.inst.push(0xdb);
                self.mem_emit(1, (mem));
            }
            8 => {
                self.inst.push(0xdd);
                self.mem_emit(1, (mem));
            }
            _ => {}
        }
    }
    
    pub fn fisttp_membase_size(&mut self, basereg: Reg, disp: i32, size: i32) {
        self.rex_emit(0, Reg::NONE, Reg::NONE, (basereg));
        match size {
            2 => {
                self.inst.push(0xdf);
                self.membase_emit(1, (basereg), (disp));
            }
            4 => {
                self.inst.push(0xdb);
                self.membase_emit(1, (basereg), (disp));
            }
            8 => {
                self.inst.push(0xdd);
                self.membase_emit(1, (basereg), (disp));
            }
            _ => {}
        }
    }
    
    pub fn fabs(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe1);
    }
    
    pub fn fchs(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe0);
    }
    
    /*
     * Store fpu control word after checking for pending unmasked fpu exceptions
     */
    pub fn fnstcw(&mut self, mem: i32) {
        self.inst.push(0xd9);
        self.mem_emit(7, (mem));
    }
    
    pub fn fnstcw_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xd9);
        self.membase_emit(7, (basereg), (disp));
    }
    
    /*
     * Load fpu control word
     */
    pub fn fldcw(&mut self, mem: i32) {
        self.inst.push(0xd9);
        self.mem_emit(5, (mem));
    }
    
    pub fn fldcw_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xd9);
        self.membase_emit (5, (basereg), (disp));
    }
}

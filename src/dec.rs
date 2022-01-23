use crate::{ByteReader, Csr, Gpr, Inst};

use crate::extr::{
    b_imm_bits,
    c_f3_bits,
    f3_bits,
    //    rs3_bits,
    //    f2_bits,
    f5_bits,
    f6_bits,
    f7_bits,
    i_imm_bits,
    j_imm_bits,
    op_bits,
    rd_bits,
    rs1_bits,
    rs2_bits,
    s_imm_bits,
    shamt32_imm_bits,
    shamt64_imm_bits,
    u_imm_bits,
};

#[allow(dead_code)]
pub fn decode(bytes: &mut dyn ByteReader, bits: u8) -> (Inst, u8) {
    let b0 = match bytes.next() {
        None => return (Inst::ERROR, 0),
        Some(b) => b,
    };

    match b0 & 0b11 {
        0b00 => decode_quadrant0(bytes, b0, bits),
        0b01 => decode_quadrant1(bytes, b0, bits),
        0b10 => decode_quadrant2(bytes, b0, bits),
        0b11 => decode_quadrant3(bytes, b0, bits),
        _ => unreachable!(),
    }
}

fn decode_quadrant0(bytes: &mut dyn ByteReader, b0: u8, bits: u8) -> (Inst, u8) {
    let mut w = b0 as u16;
    let wn = match bytes.next() {
        None => return (Inst::ERROR, 0),
        Some(b) => b,
    };
    w |= (wn as u16) << 8;

    (
        match c_f3_bits(w) {
            0b000 => {
                if w == 0 {
                    Inst::CILLEGAL
                } else {
                    Inst::CADDI4SPN // (crd_gpr(w), nzuimm_bits(w))
                }
            }
            0b001 => {
                if bits != 128 {
                    Inst::CFLD // 001 uimm[5:3] rs1′ uimm[7:6] rd′ 00  (RV32/64)
                } else {
                    Inst::CLQ // 001 uimm[5:4|8] rs1′ uimm[7:6] rd′ 00  (RV128)
                }
            }
            0b010 => {
                Inst::CLW // 010 uimm[5:3] rs1′ uimm[2|6] rd′ 00
            }
            0b011 => {
                if bits == 32 {
                    Inst::CFLW // 011 uimm[5:3] rs1′ uimm[2|6] rd′ 00  (RV32)
                } else {
                    Inst::CLD // 011 uimm[5:3] rs1′ uimm[7:6] rd′ 00  (RV64/128)
                }
            }
            0b101 => {
                if bits != 128 {
                    Inst::CFSD // 101 uimm[5:3] rs1′ uimm[7:6] rs2′ 00  (RV32/64)
                } else {
                    Inst::CSQ // 101 uimm[5:4|8] rs1′ uimm[7:6] rs2′ 00  (RV128)
                }
            }
            0b110 => {
                Inst::CSW // 110 uimm[5:3] rs1′ uimm[2|6] rs2′ 00
            }
            0b111 => {
                if bits == 32 {
                    Inst::CFSW // 111 uimm[5:3] rs1′ uimm[2|6] rs2′ 00  (RV32)
                } else {
                    Inst::CSD // 111 uimm[5:3] rs1′ uimm[7:6] rs2′ 00  (RV64/128)
                }
            }
            _ => Inst::CUNDEF,
        },
        2,
    )
}

/*
fn crd_gpr(_w: u16) -> Gpr {
    Gpr::zero
}

fn nzuimm_bits(_w: u16) -> u8 {
    0
}
*/

fn decode_quadrant1(bytes: &mut dyn ByteReader, b0: u8, bits: u8) -> (Inst, u8) {
    let mut w = b0 as u16;
    let wn = match bytes.next() {
        None => return (Inst::ERROR, 0),
        Some(b) => b,
    };
    w |= (wn as u16) << 8;

    (
        match c_f3_bits(w) {
            0b000 => {
                // CNOP // 000 nzimm[5] 0 nzimm[4:0] 01  (HINT, nzimm̸=0)
                Inst::CADDI // 000 nzimm[5] rs1/rd̸=0 nzimm[4:0] 01  (HINT, nzimm=0)
            }

            0b001 => {
                if bits == 32 {
                    Inst::CJAL // 001 imm[11|4|9:8|10|6|7|3:1|5] 01  (RV32)
                } else {
                    Inst::CADDIW // 001 imm[5] rs1/rd̸=0 imm[4:0] 01  (RV64/128; RES, rd=0)
                }
            }

            0b010 => Inst::CLI, // 010 imm[5] rd̸=0 imm[4:0] 01  (HINT, rd=0)

            0b011 => {
                // Inst::CADDI16SP // 011 nzimm[9] 2 nzimm[4|6|8:7|5] 01  (RES, nzimm=0)
                Inst::CLUI // 011 nzimm[17] rd̸={0, 2} nzimm[16:12] 01  (RES, nzimm=0; HINT, rd=0)
            }

            0b100 => {
                // Inst::CSRLI               100 nzuimm[5] 00 rs1′/rd′ nzuimm[4:0] 01  (RV32 NSE, nzuimm[5]=1)
                // Inst::CSRLI64             100 0 00 rs1′/rd′ 0 01  (RV128; RV32/64 HINT)
                // Inst::CSRAI               100 nzuimm[5] 01 rs1′/rd′ nzuimm[4:0] 01  (RV32 NSE, nzuimm[5]=1)
                // Inst::CSRAI64             100 0 01 rs1′/rd′ 0 01  (RV128; RV32/64 HINT)
                // Inst::CANDI               100 imm[5] 10 rs1′/rd′ imm[4:0] 01
                // Inst::CSUB                100 0 11 rs1′/rd′ 00 rs2′ 01
                // Inst::CXOR                100 0 11 rs1′/rd′ 01 rs2′ 01
                // Inst::COR                 100 0 11 rs1′/rd′ 10 rs2′ 01
                // Inst::CAND                100 0 11 rs1′/rd′ 11 rs2′ 01
                // Inst::CSUBW               100 1 11 rs1′/rd′ 00 rs2′ 01  (RV64/128; RV32 RES)
                // Inst::CADDW               100 1 11 rs1′/rd′ 01 rs2′ 01  (RV64/128; RV32 RES)
                Inst::CUNDEF
            }

            0b101 => Inst::CJ,    // 101 imm[11|4|9:8|10|6|7|3:1|5] 01
            0b110 => Inst::CBEQZ, // 110 imm[8|4:3] rs1′ imm[7:6|2:1|5] 01
            0b111 => Inst::CBNEZ, // 111 imm[8|4:3] rs1′ imm[7:6|2:1|5] 01
            _ => Inst::CUNDEF,
        },
        2,
    )
}

fn decode_quadrant2(bytes: &mut dyn ByteReader, b0: u8, bits: u8) -> (Inst, u8) {
    let mut w = b0 as u16;
    let wn = match bytes.next() {
        None => return (Inst::ERROR, 0),
        Some(b) => b,
    };
    w |= (wn as u16) << 8;

    (
        match c_f3_bits(w) {
            0b000 => {
                if bits == 32 {
                    Inst::CSLLI // 000 nzuimm[5] rs1/rd̸=0 nzuimm[4:0] 10  (HINT, rd=0; RV32 NSE, nzuimm[5]=1)
                } else {
                    Inst::CSLLI64 // 000 0 rs1/rd̸=0 0 10  (RV128; RV32/64 HINT; HINT, rd=0)
                }
            }
            0b001 => {
                Inst::CFLDSP // 001 uimm[5] rd uimm[4:3|8:6] 10  (RV32/64)
                             // Inst::CLQSP // 001 uimm[5] rd̸=0 uimm[4|9:6] 10  (RV128; RES, rd=0)
            }
            0b010 => {
                Inst::CLWSP // 010 uimm[5] rd̸=0 uimm[4:2|7:6] 10  (RES, rd=0)
            }
            0b011 => {
                Inst::CFLWSP // 011 uimm[5] rd uimm[4:2|7:6] 10  (RV32)
                             // Inst::CLDSP // 011 uimm[5] rd̸=0 uimm[4:3|8:6] 10  (RV64/128; RES, rd=0)
            }
            0b100 => {
                Inst::CJR // 100 0 rs1̸=0 0 10  (RES, rs1=0)
                          // Inst::CMV  // 100 0 rd̸=0 rs2̸=0 10  (HINT, rd=0)
                          // Inst::CEBREAK  // 100 1 0 0 10
                          // Inst::CJALR  // 100 1 rs1̸=0 0 10
                          // Inst::CADD  // 100 1 rs1/rd̸=0 rs2̸=0 10  (HINT, rd=0)
            }
            0b101 => {
                Inst::CFSDSP // 101 uimm[5:3|8:6] rs2 10  (RV32/64)
                             // Inst::CSQSP // 101 uimm[5:4|9:6] rs2 10  (RV128)
            }
            0b110 => {
                Inst::CSWSP // 110 uimm[5:2|7:6] rs2 10
            }
            0b111 => {
                Inst::CFSWSP // 111 uimm[5:2|7:6] rs2 10  (RV32)
                             // Inst::CSDSP // 111 uimm[5:3|8:6] rs2 10  (RV64/128)
            }
            _ => Inst::CUNDEF,
        },
        2,
    )
}

fn decode_quadrant3(bytes: &mut dyn ByteReader, b0: u8, bits: u8) -> (Inst, u8) {
    let mut w = b0 as u32;
    for n in 1..4 {
        let wn = match bytes.next() {
            None => return (Inst::ERROR, 0),
            Some(b) => b,
        };
        w |= (wn as u32) << n * 8;
    }

    (
        match op_bits(w) {
            0b00000 => match f3_bits(w) {
                0b000 => Inst::LB(rd(w), rs1(w), i_imm_bits(w)),
                0b001 => Inst::LH(rd(w), rs1(w), i_imm_bits(w)),
                0b010 => Inst::LW(rd(w), rs1(w), i_imm_bits(w)),
                0b011 => Inst::LD(rd(w), rs1(w), i_imm_bits(w)),
                0b100 => Inst::LBU(rd(w), rs1(w), i_imm_bits(w)),
                0b101 => Inst::LHU(rd(w), rs1(w), i_imm_bits(w)),
                0b110 => Inst::LWU(rd(w), rs1(w), i_imm_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b00011 => match f3_bits(w) {
                0b000 => Inst::FENCE(rd(w), rs1(w), i_imm_bits(w)),
                0b001 => Inst::FENCEI(rd(w), rs1(w), i_imm_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b00100 => match f3_bits(w) {
                0b000 => Inst::ADDI(rd(w), rs1(w), i_imm_bits(w)),
                0b001 => {
                    if bits == 64 {
                        match f6_bits(w) {
                            00 => Inst::SLLI(rd(w), rs1(w), shamt64_imm_bits(w)),
                            _ => Inst::UNDEF(w),
                        }
                    } else {
                        match f7_bits(w) {
                            00 => Inst::SLLI(rd(w), rs1(w), shamt32_imm_bits(w)),
                            _ => Inst::UNDEF(w),
                        }
                    }
                }
                0b010 => Inst::SLTI(rd(w), rs1(w), i_imm_bits(w)),
                0b011 => Inst::SLTUI(rd(w), rs1(w), i_imm_bits(w) as u32),
                0b100 => Inst::XORI(rd(w), rs1(w), i_imm_bits(w)),
                0b101 => {
                    if bits == 64 {
                        match f6_bits(w) {
                            0x00 => Inst::SRLI(rd(w), rs1(w), shamt64_imm_bits(w)),
                            0x10 => Inst::SRAI(rd(w), rs1(w), shamt64_imm_bits(w)),
                            _ => Inst::UNDEF(w),
                        }
                    } else {
                        match f7_bits(w) {
                            0x00 => Inst::SRLI(rd(w), rs1(w), shamt32_imm_bits(w)),
                            0x20 => Inst::SRAI(rd(w), rs1(w), shamt32_imm_bits(w)),
                            _ => Inst::UNDEF(w),
                        }
                    }
                }
                0b110 => Inst::ORI(rd(w), rs1(w), i_imm_bits(w)),
                0b111 => Inst::ANDI(rd(w), rs1(w), i_imm_bits(w)),
                _ => unreachable!(),
            },
            0b00101 => Inst::AUIPC(rd(w), u_imm_bits(w)),
            0b00110 => match f3_bits(w) {
                0x00 => Inst::ADDIW(rd(w), rs1(w), i_imm_bits(w)),
                0x01 => Inst::SLLIW(rd(w), rs1(w), shamt32_imm_bits(w)),
                0x05 => match f7_bits(w) {
                    0x00 => Inst::SRLIW(rd(w), rs1(w), shamt32_imm_bits(w)),
                    0x20 => Inst::SRAIW(rd(w), rs1(w), shamt32_imm_bits(w)),
                    _ => Inst::UNDEF(w),
                },
                _ => Inst::UNDEF(w),
            },
            0b01000 => match f3_bits(w) {
                0b001 => Inst::SH(rd(w), rs1(w), s_imm_bits(w)),
                0b010 => Inst::SW(rd(w), rs1(w), s_imm_bits(w)),
                0b000 => Inst::SB(rd(w), rs1(w), s_imm_bits(w)),
                0b011 => Inst::SD(rd(w), rs1(w), s_imm_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b01011 => match f3_bits(w) {
                0b010 => match f5_bits(w) {
                    0b00010 => match rs2_bits(w) {
                        0b00000 => Inst::LRW(rd(w), rs1(w)),
                        _ => Inst::UNDEF(w),
                    },
                    0b00011 => Inst::SCW(rd(w), rs1(w), rs2(w)),
                    0b00001 => Inst::AMOSWAPW(rd(w), rs1(w), rs2(w)),
                    0b00000 => Inst::AMOADDW(rd(w), rs1(w), rs2(w)),
                    0b00100 => Inst::AMOXORW(rd(w), rs1(w), rs2(w)),
                    0b01100 => Inst::AMOANDW(rd(w), rs1(w), rs2(w)),
                    0b01000 => Inst::AMOORW(rd(w), rs1(w), rs2(w)),
                    0b10000 => Inst::AMOMINW(rd(w), rs1(w), rs2(w)),
                    0b10100 => Inst::AMOMAXW(rd(w), rs1(w), rs2(w)),
                    0b11000 => Inst::AMOMINUW(rd(w), rs1(w), rs2(w)),
                    0b11100 => Inst::AMOMAXUW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b011 => match f5_bits(w) {
                    0b00010 => match rs2_bits(w) {
                        0b00000 => Inst::LRD(rd(w), rs1(w)),
                        _ => Inst::UNDEF(w),
                    },
                    0b00011 => Inst::SCD(rd(w), rs1(w), rs2(w)),
                    0b00001 => Inst::AMOSWAPD(rd(w), rs1(w), rs2(w)),
                    0b00000 => Inst::AMOADDD(rd(w), rs1(w), rs2(w)),
                    0b00100 => Inst::AMOXORD(rd(w), rs1(w), rs2(w)),
                    0b01100 => Inst::AMOANDD(rd(w), rs1(w), rs2(w)),
                    0b01000 => Inst::AMOORD(rd(w), rs1(w), rs2(w)),
                    0b10000 => Inst::AMOMIND(rd(w), rs1(w), rs2(w)),
                    0b10100 => Inst::AMOMAXD(rd(w), rs1(w), rs2(w)),
                    0b11000 => Inst::AMOMINUD(rd(w), rs1(w), rs2(w)),
                    0b11100 => Inst::AMOMAXUD(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                _ => Inst::UNDEF(w),
            },
            0b01100 => match f3_bits(w) {
                0b000 => match f7_bits(w) {
                    0x00 => Inst::ADD(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::MUL(rd(w), rs1(w), rs2(w)),
                    0x20 => Inst::SUB(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b001 => match f7_bits(w) {
                    0x00 => Inst::SLL(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::MULH(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b010 => match f7_bits(w) {
                    0x00 => Inst::SLT(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::MULSU(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b011 => match f7_bits(w) {
                    0x00 => Inst::SLTU(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::MULU(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b100 => match f7_bits(w) {
                    0x00 => Inst::XOR(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::DIV(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b101 => match f7_bits(w) {
                    0x00 => Inst::SRL(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::DIVU(rd(w), rs1(w), rs2(w)),
                    0x20 => Inst::SRA(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b110 => match f7_bits(w) {
                    0x00 => Inst::OR(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::REM(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b111 => match f7_bits(w) {
                    0x00 => Inst::AND(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::REMU(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                _ => unreachable!(),
            },
            0b01101 => Inst::LUI(rd(w), u_imm_bits(w)),
            0b01110 => match f3_bits(w) {
                0b000 => match f7_bits(w) {
                    0x00 => Inst::ADDW(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::MULW(rd(w), rs1(w), rs2(w)),
                    0x20 => Inst::SUBW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b001 => match f7_bits(w) {
                    0x00 => Inst::SLL(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b100 => match f7_bits(w) {
                    0x01 => Inst::DIVW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b101 => match f7_bits(w) {
                    0x00 => Inst::SRLW(rd(w), rs1(w), rs2(w)),
                    0x01 => Inst::DIVUW(rd(w), rs1(w), rs2(w)),
                    0x20 => Inst::SRAW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b110 => match f7_bits(w) {
                    0x01 => Inst::REMW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b111 => match f7_bits(w) {
                    0x01 => Inst::REMUW(rd(w), rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                _ => Inst::UNDEF(w),
            },
            0b11000 => match f3_bits(w) {
                0b000 => Inst::BEQ(rs1(w), rs2(w), b_imm_bits(w)),
                0b001 => Inst::BNE(rs1(w), rs2(w), b_imm_bits(w)),
                0b100 => Inst::BLT(rs1(w), rs2(w), b_imm_bits(w)),
                0b101 => Inst::BGE(rs1(w), rs2(w), b_imm_bits(w)),
                0b110 => Inst::BLTU(rs1(w), rs2(w), b_imm_bits(w)),
                0b111 => Inst::BGEU(rs1(w), rs2(w), b_imm_bits(w)),
                _ => Inst::UNDEF(w),
            },
            0b11001 => Inst::JALR(rd(w), rs1(w), i_imm_bits(w)),
            0b11011 => Inst::JAL(rd(w), j_imm_bits(w)),
            0b11100 => match f3_bits(w) {
                0b000 => match (f7_bits(w), rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                    (0b0000000, 0b00000, 0b00000, 0b00000) => Inst::ECALL,
                    (0b0000000, 0b00001, 0b00000, 0b00000) => Inst::EBREAK,
                    (0b0001000, 0b00010, 0b00000, 0b00000) => Inst::SRET,
                    (0b0011000, 0b00010, 0b00000, 0b00000) => Inst::MRET,
                    (0b0001000, 0b00101, 0b00000, 0b00000) => Inst::WFI,
                    (0b0001001, _, _, 0b00000) => Inst::SFENCEVMA(rs1(w), rs2(w)),
                    (0b0001011, _, _, 0b00000) => Inst::SINVALVMA(rs1(w), rs2(w)),
                    (0b0001100, 0b00000, 0b00000, 0b00000) => Inst::SFENCEWINVAL,
                    (0b0001100, 0b00001, 0b00000, 0b00000) => Inst::SFENCEINVALIR,
                    (0b0010001, _, _, 0b00000) => Inst::HFENCEVVMA(rs1(w), rs2(w)),
                    (0b0110001, _, _, 0b00000) => Inst::HFENCEGVMA(rs1(w), rs2(w)),
                    (0b0010011, _, _, 0b00000) => Inst::HINVALVVMA(rs1(w), rs2(w)),
                    (0b0110011, _, _, 0b00000) => Inst::HINVALGVMA(rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b100 => match (f7_bits(w), rs2_bits(w), rs1_bits(w), rd_bits(w)) {
                    (0b0110000, 0b00000, _, _) => Inst::HLVB(rd(w), rs1(w)),
                    (0b0110000, 0b00001, _, _) => Inst::HLVBU(rd(w), rs1(w)),
                    (0b0110010, 0b00000, _, _) => Inst::HLVH(rd(w), rs1(w)),
                    (0b0110010, 0b00001, _, _) => Inst::HLVHU(rd(w), rs1(w)),
                    (0b0110010, 0b00011, _, _) => Inst::HLVXHU(rd(w), rs1(w)),
                    (0b0110100, 0b00000, _, _) => Inst::HLVW(rd(w), rs1(w)),
                    (0b0110100, 0b00011, _, _) => Inst::HLVXWU(rd(w), rs1(w)),
                    (0b0110001, _, _, 0b00000) => Inst::HSVB(rs1(w), rs2(w)),
                    (0b0110011, _, _, 0b00000) => Inst::HSVH(rs1(w), rs2(w)),
                    (0b0110101, _, _, 0b00000) => Inst::HSVW(rs1(w), rs2(w)),
                    (0b0110100, 0b00001, _, _) => Inst::HLVWU(rd(w), rs1(w)),
                    (0b0110110, 0b00000, _, _) => Inst::HLVD(rd(w), rs1(w)),
                    (0b0110111, _, _, 0b00000) => Inst::HSVD(rs1(w), rs2(w)),
                    _ => Inst::UNDEF(w),
                },
                0b001 => Inst::CSRRW(rd(w), rs1(w), csr(w)),
                0b010 => Inst::CSRRS(rd(w), rs1(w), csr(w)),
                0b011 => Inst::CSRRC(rd(w), rs1(w), csr(w)),
                0b101 => Inst::CSRRWI(rd(w), rs1_bits(w), csr(w)),
                0b110 => Inst::CSRRSI(rd(w), rs1_bits(w), csr(w)),
                0b111 => Inst::CSRRCI(rd(w), rs1_bits(w), csr(w)),
                _ => Inst::UNDEF(w),
            },
            _ => Inst::UNDEF(w),
        },
        4,
    )
}

fn rd(w: u32) -> Gpr {
    Gpr::from_u8(rd_bits(w) as u8).unwrap()
}

fn rs1(w: u32) -> Gpr {
    Gpr::from_u8(rs1_bits(w) as u8).unwrap()
}

fn rs2(w: u32) -> Gpr {
    Gpr::from_u8(rs2_bits(w) as u8).unwrap()
}

// fn rs3(w: u32) -> Gpr {
//     Gpr::from_u8(rs3_bits(w) as u8).unwrap()
// }

fn csr(w: u32) -> Csr {
    Csr::from_u16(i_imm_bits(w) as u16)
}

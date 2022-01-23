fn bits(w: u32, shr: u8, msh: u8) -> u32 {
    let t0 = w >> shr;
    let t1 = (1 << msh) - 1;
    t0 & t1
}

fn sign(w: u32) -> u32 {
    if w & 0x80000000 == 0 {
        0
    } else {
        !0
    }
}

pub fn op_bits(w: u32) -> u8 {
    bits(w, 2, 5) as u8
}

// funct bits

pub fn f3_bits(w: u32) -> u8 {
    bits(w, 12, 3) as u8
}

pub fn f5_bits(w: u32) -> u8 {
    bits(w, 27, 5) as u8
}

pub fn f6_bits(w: u32) -> u8 {
    bits(w, 26, 6) as u8
}

pub fn f7_bits(w: u32) -> u8 {
    bits(w, 25, 7) as u8
}

// pub fn f2_bits(w: u32) -> u8 {
//     bits(w, 25, 2) as u8
// }

// R-type

pub fn rd_bits(w: u32) -> u8 {
    bits(w, 7, 5) as u8
}

pub fn rs1_bits(w: u32) -> u8 {
    bits(w, 15, 5) as u8
}

pub fn rs2_bits(w: u32) -> u8 {
    bits(w, 20, 5) as u8
}

// pub fn rs3_bits(w: u32) -> u8 {
//     bits(w, 27, 5) as u8
// }

// I-type (immediate)

pub fn i_imm_bits(w: u32) -> i32 {
    (bits(w, 20, 12) | sign(w) << 12) as i32
}

// SHAMT

pub fn shamt32_imm_bits(w: u32) -> u8 {
    bits(w, 20, 5) as u8
}

pub fn shamt64_imm_bits(w: u32) -> u8 {
    bits(w, 20, 6) as u8
}

// S-type (store)

pub fn s_imm_bits(w: u32) -> i32 {
    (bits(w, 7, 5) | bits(w, 25, 7) << 5 | sign(w) << 12) as i32
}

// U-type: (LUI, AUIPC)

pub fn u_imm_bits(w: u32) -> u32 {
    bits(w, 12, 20) as u32
}

// J-type (JAL/R)

pub fn j_imm_bits(w: u32) -> i32 {
    (bits(w, 21, 10) << 1 | bits(w, 20, 1) << 11 | bits(w, 12, 8) << 12 | sign(w) << 20) as i32
}

// B-type (Bcc)

pub fn b_imm_bits(w: u32) -> i32 {
    (bits(w, 8, 4) << 1 | bits(w, 25, 6) << 5 | bits(w, 7, 1) << 11 | sign(w) << 12) as i32
}

//
// RVC instruction format:
//

pub fn c_f3_bits(w: u16) -> u8 {
    bits(w as u32, 13, 3) as u8
}

// CR   Register                funct4 rd/rs1   rs2                 op
// CI   Immediate               funct3 imm      rd/rs1  imm         op
// CSS  Stack-relative Store    funct3 imm      rs2                 op
// CIW  Wide Immediate          funct3 imm      rd′                 op
// CL   Load                    funct3 imm      rs1′    imm rd′     op
// CS   Store                   funct3 imm      rs1′    imm rs2′    op
// CA   Arithmetic              funct6 rd′/rs1′ funct2  rs2′        op
// CB   Branch                  funct3 offset   rs1′    offset      op
// CJ   Jump                    funct3 jump target                  op
//

// RVC              Register Number     000 001 010 011 100 101 110 111
// Integer          Register Number     x8  x9  x10 x11 x12 x13 x14 x15
// Integer          Register ABI Name   s0  s1  a0  a1  a2  a3  a4  a5
// Floating-Point   Register Number     f8  f9  f10 f11 f12 f13 f14 f15
// Floating-Point   Register ABI Name   fs0 fs1 fa0 fa1 fa2 fa3 fa4 fa5

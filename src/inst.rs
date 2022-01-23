use crate::{Gpr, Csr};

#[derive(Debug)]
pub enum Inst {
    ERROR,

    UNDEF(u32),

    ADD(Gpr, Gpr, Gpr),
    SUB(Gpr, Gpr, Gpr),
    XOR(Gpr, Gpr, Gpr),
    OR(Gpr, Gpr, Gpr),
    AND(Gpr, Gpr, Gpr),

    SLL(Gpr, Gpr, Gpr),
    SRL(Gpr, Gpr, Gpr),
    SRA(Gpr, Gpr, Gpr),

    SLT(Gpr, Gpr, Gpr),
    SLTU(Gpr, Gpr, Gpr),

    ADDI(Gpr, Gpr, u32),
    XORI(Gpr, Gpr, u32),
    ORI(Gpr, Gpr, u32),
    ANDI(Gpr, Gpr, u32),

    SLLI(Gpr, Gpr, u32),
    SRLI(Gpr, Gpr, u32),
    SRAI(Gpr, Gpr, u32),

    SLTI(Gpr, Gpr, u32),
    SLTUI(Gpr, Gpr, u32),

    LB(Gpr, Gpr, u32),
    LH(Gpr, Gpr, u32),
    LW(Gpr, Gpr, u32),

    LBU(Gpr, Gpr, u32),
    LHU(Gpr, Gpr, u32),

    SB(Gpr, Gpr, u32),
    SH(Gpr, Gpr, u32),
    SW(Gpr, Gpr, u32),

    BEQ(Gpr, Gpr, u32),
    BNE(Gpr, Gpr, u32),
    BLT(Gpr, Gpr, u32),
    BGE(Gpr, Gpr, u32),
    BLTU(Gpr, Gpr, u32),
    BGEU(Gpr, Gpr, u32),

    JAL(Gpr, u32),
    JALR(Gpr, Gpr, u32),

    LUI(Gpr, u32),
    AUIPC(Gpr, u32),

    ECALL,
    EBREAK,

    // RV32/RV64 Zifencei extensions

    FENCE(Gpr, Gpr, u32),
    FENCEI(Gpr, Gpr, u32),
    
    // RV32/RV64 Zicsr extensions
    
    CSRRW(Gpr, Gpr, Csr),
    CSRRS(Gpr, Gpr, Csr),
    CSRRC(Gpr, Gpr, Csr),
    CSRRWI(Gpr, u32, Csr),
    CSRRSI(Gpr, u32, Csr),
    CSRRCI(Gpr, u32, Csr),

    // priviledges instructions

    SRET,
    MRET,
    WFI,

    // 64 bits extensions

    LD(Gpr, Gpr, u32),
    LWU(Gpr, Gpr, u32),
    SD(Gpr, Gpr, u32),

    ADDIW(Gpr, Gpr, u32),
    SLLIW(Gpr, Gpr, u32),
    SRLIW(Gpr, Gpr, u32),
    SRAIW(Gpr, Gpr, u32),

    ADDW(Gpr, Gpr, Gpr),
    SUBW(Gpr, Gpr, Gpr),
    SLLW(Gpr, Gpr, Gpr),
    SRLW(Gpr, Gpr, Gpr),
    SRAW(Gpr, Gpr, Gpr),

    // Multiply Extension

    MUL(Gpr, Gpr, Gpr),
    MULH(Gpr, Gpr, Gpr),
    MULSU(Gpr, Gpr, Gpr),
    MULU(Gpr, Gpr, Gpr),

    // Multiply Extension (64 bits)

    MULW(Gpr, Gpr, Gpr),

    // Divide/Remaining Extension

    DIV(Gpr, Gpr, Gpr),
    DIVU(Gpr, Gpr, Gpr),
    REM(Gpr, Gpr, Gpr),
    REMU(Gpr, Gpr, Gpr),

    // Divide/Remaining Extension (64 bits)

    DIVW(Gpr, Gpr, Gpr),
    DIVUW(Gpr, Gpr, Gpr),
    REMW(Gpr, Gpr, Gpr),
    REMUW(Gpr, Gpr, Gpr),

    // Load eXclusive / Store Conditional Extension

    LRW(Gpr, Gpr),
    SCW(Gpr, Gpr, Gpr),

    // Load eXclusive / Store Conditional Extension (64 bits)

    LRD(Gpr, Gpr),
    SCD(Gpr, Gpr, Gpr),

    // Atomic Extension

    AMOSWAPW(Gpr, Gpr, Gpr),    // 00001 aq rl rs2 rs1 010 rd 0101111 
    AMOADDW(Gpr, Gpr, Gpr),     // 00000 aq rl rs2 rs1 010 rd 0101111 
    AMOXORW(Gpr, Gpr, Gpr),     // 00100 aq rl rs2 rs1 010 rd 0101111 
    AMOANDW(Gpr, Gpr, Gpr),     // 01100 aq rl rs2 rs1 010 rd 0101111 
    AMOORW(Gpr, Gpr, Gpr),      // 01000 aq rl rs2 rs1 010 rd 0101111 
    AMOMINW(Gpr, Gpr, Gpr),     // 10000 aq rl rs2 rs1 010 rd 0101111 
    AMOMAXW(Gpr, Gpr, Gpr),     // 10100 aq rl rs2 rs1 010 rd 0101111 
    AMOMINUW(Gpr, Gpr, Gpr),    // 11000 aq rl rs2 rs1 010 rd 0101111 
    AMOMAXUW(Gpr, Gpr, Gpr),    // 11100 aq rl rs2 rs1 010 rd 0101111 
    
    // Atomic Extension (64 bits)

    AMOSWAPD(Gpr, Gpr, Gpr),    // 00001 aq rl rs2 rs1 011 rd 0101111 
    AMOADDD(Gpr, Gpr, Gpr),     // 00000 aq rl rs2 rs1 011 rd 0101111 
    AMOXORD(Gpr, Gpr, Gpr),     // 00100 aq rl rs2 rs1 011 rd 0101111 
    AMOANDD(Gpr, Gpr, Gpr),     // 01100 aq rl rs2 rs1 011 rd 0101111 
    AMOORD(Gpr, Gpr, Gpr),      // 01000 aq rl rs2 rs1 011 rd 0101111 
    AMOMIND(Gpr, Gpr, Gpr),     // 10000 aq rl rs2 rs1 011 rd 0101111 
    AMOMAXD(Gpr, Gpr, Gpr),     // 10100 aq rl rs2 rs1 011 rd 0101111 
    AMOMINUD(Gpr, Gpr, Gpr),    // 11000 aq rl rs2 rs1 011 rd 0101111 
    AMOMAXUD(Gpr, Gpr, Gpr),    // 11100 aq rl rs2 rs1 011 rd 0101111 

/*
    // Floating Point Extension

    FLW(Gpr, Gpr, u32),
    FSW(Gpr, Gpr, u32),

    FMADDS(Gpr, Gpr, Gpr, Gpr, u8),
    FMSUBS(Gpr, Gpr, Gpr, Gpr, u8),
    FNMSUBS(Gpr, Gpr,Gpr, Gpr, u8),
    FNMADDS(Gpr, Gpr,Gpr, Gpr, u8),

    FADDS(Gpr, Gpr, Gpr, u8),
    FSUBS(Gpr, Gpr, Gpr, u8),
    FMULS(Gpr, Gpr, Gpr, u8),
    FDIVS(Gpr, Gpr, Gpr, u8),
    FSQRTS(Gpr, Gpr, Gpr, u8),

    FSGNJS(Gpr, Gpr, Gpr, u8),
    FSGNJNS(Gpr, Gpr, Gpr, u8),
    FSGNJXS(Gpr, Gpr, Gpr, u8),

    FMINS(Gpr, Gpr, Gpr, u8),
    FMAXS(Gpr, Gpr, Gpr, u8),

    FCVTWS(Gpr, Gpr, Gpr, u8),
    FCVTWUS(Gpr, Gpr, Gpr, u8),

    FMVXW(Gpr, Gpr, Gpr, u8),

    FEQS(Gpr, Gpr, Gpr, u8),
    FLTS(Gpr, Gpr, Gpr, u8),
    FLES(Gpr, Gpr, Gpr, u8),

    FCLASSS(Gpr, Gpr, Gpr, u8),

    FCVTSW(Gpr, Gpr, Gpr, u8),
    FCVTSWU(Gpr, Gpr, Gpr, u8),
    FMVWX(Gpr, Gpr, Gpr, u8),

    // Floating Point Extension (64 bits)

    FCVTLS(Gpr, Gpr, Gpr, u8),
    FCVTLUS(Gpr, Gpr, Gpr, u8),
    FCVTSL(Gpr, Gpr, Gpr, u8),
    FCVTSLU(Gpr, Gpr, Gpr, u8),

    // Floating Point D Extension

    FLD(Gpr),
    FSD(Gpr),

    FMADDD(Gpr),
    FMSUBD(Gpr),
    FNMSUBD(Gpr),
    FNMADDD(Gpr),

    FADDD(Gpr),
    FSUBD(Gpr),
    FMULD(Gpr),
    FDIVD(Gpr),
    FSQRTD(Gpr),

    FSGNJD(Gpr),
    FSGNJND(Gpr),
    FSGNJXD(Gpr),

    FMIND(Gpr),
    FMAXD(Gpr),

    FCVTSD(Gpr),
    FCVTDS(Gpr),

    FEQD(Gpr),
    FLTD(Gpr),
    FLED(Gpr),

    FCLASSD(Gpr),

    FCVTWD(Gpr),
    FCVTWUD(Gpr),
    FCVTDW(Gpr),
    FCVTDWU(Gpr),

    // Floating point D extensions (64 bits)

    FCVTLD(Gpr),
    FCVTLUD(Gpr),
    FMVXD(Gpr),
    FCVTDL(Gpr),
    FCVTDLU(Gpr),
    FMVDX(Gpr),

    // Floating point Q extensions

    FLQ(Gpr),
    FSQ(Gpr),

    FMADDQ(Gpr),
    FMSUBQ(Gpr),
    FNMSUBQ(Gpr),
    FNMADDQ(Gpr),

    FADDQ(Gpr),
    FSUBQ(Gpr),
    FMULQ(Gpr),
    FDIVQ(Gpr),
    FSQRTQ(Gpr),

    FSGNJQ(Gpr),
    FSGNJNQ(Gpr),
    FSGNJXQ(Gpr),

    FMINQ(Gpr),
    FMAXQ(Gpr),

    FCVTSQ(Gpr),
    FCVTQS(Gpr),
    FCVTDQ(Gpr),
    FCVTQD(Gpr),

    FEQQ(Gpr),
    FLTQ(Gpr),
    FLEQ(Gpr),

    FCLASSQ(Gpr),

    FCVTWQ(Gpr),
    FCVTWUQ(Gpr),
    FCVTQW(Gpr),
    FCVTQWU(Gpr),

    // Floating point Q extensions (64 bits)

    FCVTLQ(Gpr),
    FCVTLUQ(Gpr),
    FCVTQL(Gpr),
    FCVTQLU(Gpr),
*/

    // Supervisor Memory-Management Instructions

    SFENCEVMA(Gpr, Gpr),    // 0001001 rs2 rs1 000 00000 1110011       
    SINVALVMA(Gpr, Gpr),    // 0001011 rs2 rs1 000 00000 1110011       
    SFENCEWINVAL,           // 0001100 00000 00000 000 00000 1110011   
    SFENCEINVALIR,          // 0001100 00001 00000 000 00000 1110011   

    // Hypervisor Memory-Management Instructions

    HFENCEVVMA(Gpr, Gpr),   // 0010001 rs2 rs1 000 00000 1110011 
    HFENCEGVMA(Gpr, Gpr),   // 0110001 rs2 rs1 000 00000 1110011 
    HINVALVVMA(Gpr, Gpr),   // 0010011 rs2 rs1 000 00000 1110011 
    HINVALGVMA(Gpr, Gpr),   // 0110011 rs2 rs1 000 00000 1110011 

    // Hypervisor Virtual-Machine Load and Store Instructions

    HLVB(Gpr, Gpr),         // 0110000 00000 rs1 100 rd 1110011 
    HLVBU(Gpr, Gpr),        // 0110000 00001 rs1 100 rd 1110011 
    HLVH(Gpr, Gpr),         // 0110010 00000 rs1 100 rd 1110011 
    HLVHU(Gpr, Gpr),        // 0110010 00001 rs1 100 rd 1110011 
    HLVXHU(Gpr, Gpr),       // 0110010 00011 rs1 100 rd 1110011 
    HLVW(Gpr, Gpr),         // 0110100 00000 rs1 100 rd 1110011 
    HLVXWU(Gpr, Gpr),       // 0110100 00011 rs1 100 rd 1110011 
    HSVB(Gpr, Gpr),         // 0110001 rs2 rs1 100 00000 1110011 
    HSVH(Gpr, Gpr),         // 0110011 rs2 rs1 100 00000 1110011 
    HSVW(Gpr, Gpr),         // 0110101 rs2 rs1 100 00000 1110011 

    // Hypervisor Virtual-Machine Load and Store Instructions, RV64 only

    HLVWU(Gpr, Gpr),        // 0110100 00001 rs1 100 rd 1110011     
    HLVD(Gpr, Gpr),         // 0110110 00000 rs1 100 rd 1110011     
    HSVD(Gpr, Gpr),         // 0110111 rs2 rs1 100 00000 1110011    

    // TODO: bit manipulation
    // TODO: vector (with separate floating point H extension)
    // TODO: decimal
    // TODO: packed-simd
    // TODO: compressed instruction
}

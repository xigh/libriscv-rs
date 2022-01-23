#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum Csr {
    unknown(u16),

    fflags = 0x001,
    frm = 0x002,
    fcr = 0x003,

    cycle = 0xc00,
    time = 0xc01,
    instret = 0xc02,

    hpmcounter(u16), // n: 3 [0xc00] -> 31 [0xc1f]

    cycleh = 0xc80,
    timeh = 0xc81,
    instreth = 0xc82,

    hpmcounterh(u16), // n: 3 [0xc80] -> 31 [0xc9f]

    sstatus = 0x100,
    sie = 0x104,
    stvec = 0x105,
    scounteren = 0x106,

    senvcfg = 0x10a,

    sscratch = 0x140,
    sepc = 0x141,
    scause = 0x142,
    stval = 0x143,
    sip = 0x144,

    satp = 0x180,

    scontext = 0x58a,

    hstatus = 0x600,

    hedeleg = 0x602,
    hideleg = 0x603,
    hie = 0x604,

    hcounteren = 0x606,
    hgeie = 0x607,

    htval = 0x643,
    hip = 0x644,
    hvip = 0x645,
    htinst = 0x64a,
    hgeip = 0xe12,

    henvcfg = 0x60a,
    henvcfgh = 0x61a,

    hgatp = 0x680,

    hcontext = 0x6a8,

    htimedelta = 0x605,
    htimedeltah = 0x615,

    vsstatus = 0x200,
    vsie = 0x204,
    vstvec = 0x205,
    vsscratch = 0x240,
    vsepc = 0x241,
    vscause = 0x242,
    vstval = 0x243,
    vsip = 0x244,
    vsatp = 0x280,

    mvendorid = 0xf11,
    marchid = 0xf12,
    mimpid = 0xf13,
    mhartid = 0xf14,
    mconfigptr = 0xf15,

    mstatus = 0x300,
    misa = 0x301,
    medeleg = 0x302,
    mideleg = 0x303,
    mie = 0x304,
    mtvec = 0x305,
    mcounteren = 0x306,
    mstatush = 0x310,
    mscratch = 0x340,
    mepc = 0x341,
    mcause = 0x342,
    mtval = 0x343,
    mip = 0x344,
    mtinst = 0x34a,
    mtval2 = 0x34b,
    menvcfg = 0x30a,
    menvcfgh = 0x31a,
    mseccfg = 0x747,
    mseccfgh = 0x757,

    pmpcfg(u16), // n = 0 [0x3a0] -> 63 [0x3ef]

    mcycle = 0xb00,
    minstret = 0xb02,
    mhpmcounter(u16), // n = 3 [0xb03] -> 31 [0xb1f]

    mcycleh = 0xb80,
    minstreth = 0xb82,
    mhpmcounterh(u16), // n = 3 [0xb83] -> 31 [0xb9f]

    mcountinhibit = 0x320,
    mhpmevent(u16), // n = 3 [0x323] -> 31 [0x33f]

    tselect = 0x7a0,
    tdata1 = 0x7a1,
    tdata2 = 0x7a2,
    tdata3 = 0x7a3,
    mcontext = 0x7a8,

    dcsr = 0x7b0,
    dpc = 0x7b1,
    dscratch0 = 0x7b2,
    dscratch1 = 0x7b3,
}

impl Csr {
    pub fn from_u16(r: u16) -> Csr {
        match r {
            0x001 => Csr::fflags,
            0x002 => Csr::frm,
            0x003 => Csr::fcr,
            0xc00 => Csr::cycle,
            0xc01 => Csr::time,
            0xc02 => Csr::instret,
            0xc03 .. 0xc20 => Csr::hpmcounter(r-0xc00), // n: 3 [0xc03] -> 31 [0xc1f]
            0xc80 => Csr::cycleh,
            0xc81 => Csr::timeh,
            0xc82 => Csr::instreth,
            0xc83 .. 0xca0 => Csr::hpmcounterh(r-0xc80), // n: 3 [0xc83] -> 31 [0xc9f]
            0x100 => Csr::sstatus,
            0x104 => Csr::sie,
            0x105 => Csr::stvec,
            0x106 => Csr::scounteren,
            0x10a => Csr::senvcfg,
            0x140 => Csr::sscratch,
            0x141 => Csr::sepc,
            0x142 => Csr::scause,
            0x143 => Csr::stval,
            0x144 => Csr::sip,
            0x180 => Csr::satp,
            0x58a => Csr::scontext,
            0x600 => Csr::hstatus,
            0x602 => Csr::hedeleg,
            0x603 => Csr::hideleg,
            0x604 => Csr::hie,
            0x606 => Csr::hcounteren,
            0x607 => Csr::hgeie,
            0x643 => Csr::htval,
            0x644 => Csr::hip,
            0x645 => Csr::hvip,
            0x64a => Csr::htinst,
            0xe12 => Csr::hgeip,
            0x60a => Csr::henvcfg,
            0x61a => Csr::henvcfgh,
            0x680 => Csr::hgatp,
            0x6a8 => Csr::hcontext,
            0x605 => Csr::htimedelta,
            0x615 => Csr::htimedeltah,
            0x200 => Csr::vsstatus,
            0x204 => Csr::vsie,
            0x205 => Csr::vstvec,
            0x240 => Csr::vsscratch,
            0x241 => Csr::vsepc,
            0x242 => Csr::vscause,
            0x243 => Csr::vstval,
            0x244 => Csr::vsip,
            0x280 => Csr::vsatp,
            0xf11 => Csr::mvendorid,
            0xf12 => Csr::marchid,
            0xf13 => Csr::mimpid,
            0xf14 => Csr::mhartid,
            0xf15 => Csr::mconfigptr,
            0x300 => Csr::mstatus,
            0x301 => Csr::misa,
            0x302 => Csr::medeleg,
            0x303 => Csr::mideleg,
            0x304 => Csr::mie,
            0x305 => Csr::mtvec,
            0x306 => Csr::mcounteren,
            0x310 => Csr::mstatush,
            0x340 => Csr::mscratch,
            0x341 => Csr::mepc,
            0x342 => Csr::mcause,
            0x343 => Csr::mtval,
            0x344 => Csr::mip,
            0x34a => Csr::mtinst,
            0x34b => Csr::mtval2,
            0x30a => Csr::menvcfg,
            0x31a => Csr::menvcfgh,
            0x747 => Csr::mseccfg,
            0x757 => Csr::mseccfgh,
            0x3a0 .. 0x3f0 => Csr::pmpcfg(r - 0x3a0), // n = 0 [0x3a0] -> 63 [0x3ef]
            0xb00 => Csr::mcycle,
            0xb02 => Csr::minstret,
            0xb03 .. 0xb1f => Csr::mhpmcounter(r-0xb00), // n = 3 [0xb03] -> 31 [0xb1f]
            0xb80 => Csr::mcycleh,
            0xb82 => Csr::minstreth,
            0xb83 .. 0xba0 => Csr::mhpmcounterh(r-0xb80), // n = 3 [0xb83] -> 31 [0xb9f]
            0x320 => Csr::mcountinhibit,
            0x323 .. 0x340 => Csr::mhpmevent(r-0x320), // n = 3 [0x323] -> 31 [0x33f]
            0x7a0 => Csr::tselect,
            0x7a1 => Csr::tdata1,
            0x7a2 => Csr::tdata2,
            0x7a3 => Csr::tdata3,
            0x7a8 => Csr::mcontext,
            0x7b0 => Csr::dcsr,
            0x7b1 => Csr::dpc,
            0x7b2 => Csr::dscratch0,
            0x7b3 => Csr::dscratch1,            
            _ => Csr::unknown(r),
        }
    }
}

impl std::fmt::Display for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Csr::unknown(v) = self {
            write!(f, "0x{:x}", v)    
        } else {
            write!(f, "{:?}", self)
        }
    }
}

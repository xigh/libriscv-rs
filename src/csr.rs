#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum Csr {
    fflags = 0x001,
    frm = 0x002,
    fcr = 0x003,

    cycle = 0xc00,
    time = 0xc01,
    instret = 0xc02,

    hpmcounter(u8), // n: 3 [0xc00] -> 31 [0xc1f]

    cycleh = 0xc80,
    timeh = 0xc81,
    instreth = 0xc82,

    hpmcounterh(u8), // n: 3 [0xc80] -> 31 [0xc9f]

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

    pmpcfg(u8), // n = 0 [0x3a0] -> 63 [0x3ef]

    mcycle = 0xb00,
    minstret = 0xb02,
    mhpmcounter(u8), // n = 3 [0xb03] -> 31 [0xb1f]

    mcycleh = 0xb80,
    minstreth = 0xb82,
    mhpmcounterh(u8), // n = 3 [0xb83] -> 31 [0xb9f]

    mcountinhibit = 0x320,
    mhpmevent(u8), // n = 3 [0x323] -> 31 [0x33f]

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
    pub fn from_u16(r: u16) -> Option<Csr> {
        match r {
            0x001 => fflags,
            0x002 => frm,
            0x003 => fcr,
            0xc00 => cycle,
            0xc01 => time,
            0xc02 => instret,
            0xc03 .. 0xc20 => hpmcounter(r-0xc00), // n: 3 [0xc03] -> 31 [0xc1f]
            0xc80 => cycleh,
            0xc81 => timeh,
            0xc82 => instreth,
            0xc83 .. 0xca0 => hpmcounterh(r-0xc80), // n: 3 [0xc83] -> 31 [0xc9f]
            0x100 => sstatus,
            0x104 => sie,
            0x105 => stvec,
            0x106 => scounteren,
            0x10a => senvcfg,
            0x140 => sscratch,
            0x141 => sepc,
            0x142 => scause,
            0x143 => stval,
            0x144 => sip,
            0x180 => satp,
            0x58a => scontext,
            0x600 => hstatus,
            0x602 => hedeleg,
            0x603 => hideleg,
            0x604 => hie,
            0x606 => hcounteren,
            0x607 => hgeie,
            0x643 => htval,
            0x644 => hip,
            0x645 => hvip,
            0x64a => htinst,
            0xe12 => hgeip,
            0x60a => henvcfg,
            0x61a => henvcfgh,
            0x680 => hgatp,
            0x6a8 => hcontext,
            0x605 => htimedelta,
            0x615 => htimedeltah,
            0x200 => vsstatus,
            0x204 => vsie,
            0x205 => vstvec,
            0x240 => vsscratch,
            0x241 => vsepc,
            0x242 => vscause,
            0x243 => vstval,
            0x244 => vsip,
            0x280 => vsatp,
            0xf11 => mvendorid,
            0xf12 => marchid,
            0xf13 => mimpid,
            0xf14 => mhartid,
            0xf15 => mconfigptr,
            0x300 => mstatus,
            0x301 => misa,
            0x302 => medeleg,
            0x303 => mideleg,
            0x304 => mie,
            0x305 => mtvec,
            0x306 => mcounteren,
            0x310 => mstatush,
            0x340 => mscratch,
            0x341 => mepc,
            0x342 => mcause,
            0x343 => mtval,
            0x344 => mip,
            0x34a => mtinst,
            0x34b => mtval2,
            0x30a => menvcfg,
            0x31a => menvcfgh,
            0x747 => mseccfg,
            0x757 => mseccfgh,
            0x3a0 .. 0x3f0 => pmpcfg(r - 0x3a0), // n = 0 [0x3a0] -> 63 [0x3ef]
            0xb00 => mcycle,
            0xb02 => minstret,
            0xb03 .. 0xb1f => mhpmcounter(r-0xb00), // n = 3 [0xb03] -> 31 [0xb1f]
            0xb80 => mcycleh,
            0xb82 => minstreth,
            0xb83 .. 0xba0 => mhpmcounterh(r-0xb80), // n = 3 [0xb83] -> 31 [0xb9f]
            0x320 => mcountinhibit,
            0x323 .. 0x340 => mhpmevent(r-0x320), // n = 3 [0x323] -> 31 [0x33f]
            0x7a0 => tselect,
            0x7a1 => tdata1,
            0x7a2 => tdata2,
            0x7a3 => tdata3,
            0x7a8 => mcontext,
            0x7b0 => dcsr,
            0x7b1 => dpc,
            0x7b2 => dscratch0,
            0x7b3 => dscratch1,            
            _ => None,
        }
    }
}

impl std::fmt::Display for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

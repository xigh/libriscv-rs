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

    hpmcounter(u8), // 0xc00 + n: 3 -> 31

    cycleh = 0xc80,
    timeh = 0xc81,
    instreth = 0xc82,

    hpmcounterh(u8), // 0xc80 + n: 3 -> 31

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
}

impl Csr {
    pub fn from_u16(r: u16) -> Option<Csr> {
        match r {
            0x001 => Csr::fflags,
            0x002 => Csr::frm,
            0x003 => Csr::fcr,
            0xc00 => Csr::cycle,
            0xc01 => Csr::time,
            0xc02 => Csr::instret,
            0xc80 => Csr::cycleh,
            0xc81 => Csr::timeh,
            0xc82 => Csr::instreth,
            _ => None,
        }
    }
}

impl std::fmt::Display for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

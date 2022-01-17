#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Gpr {
    zero = 0,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    fp,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
}

impl Gpr {
    pub fn from_u8(r: u8) -> Option<Gpr> {
        match r {
        0 => Some(Gpr::zero),
        1 => Some(Gpr::ra),
        2 => Some(Gpr::sp),
        3 => Some(Gpr::gp),
        4 => Some(Gpr::tp),
        5 => Some(Gpr::t0),
        6 => Some(Gpr::t1),
        7 => Some(Gpr::t2),
        8 => Some(Gpr::fp),
        9 => Some(Gpr::s1),
        10 => Some(Gpr::a0),
        11 => Some(Gpr::a1),
        12 => Some(Gpr::a2),
        13 => Some(Gpr::a3),
        14 => Some(Gpr::a4),
        15 => Some(Gpr::a5),
        16 => Some(Gpr::a6),
        17 => Some(Gpr::a7),
        18 => Some(Gpr::s2),
        19 => Some(Gpr::s3),
        20 => Some(Gpr::s4),
        21 => Some(Gpr::s5),
        22 => Some(Gpr::s6),
        23 => Some(Gpr::s7),
        24 => Some(Gpr::s8),
        25 => Some(Gpr::s9),
        26 => Some(Gpr::s10),
        27 => Some(Gpr::s11),
        28 => Some(Gpr::t3),
        29 => Some(Gpr::t4),
        30 => Some(Gpr::t5),
        31 => Some(Gpr::t6),
        _ => None,
        }
    }
}

impl std::fmt::Display for Gpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

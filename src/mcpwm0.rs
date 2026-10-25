#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    revision: Revision,
    _reserved1: [u8; 0x0c],
    tbctl: Tbctl,
    tbprd: Tbprd,
    tbprds: Tbprds,
    tbphs: Tbphs,
    tbsts: Tbsts,
    tbstsclr: Tbstsclr,
    tbctr: Tbctr,
    _reserved8: [u8; 0x04],
    cmpctl: Cmpctl,
    _reserved9: [u8; 0x0c],
    cmpc: Cmpc,
    cmpd: Cmpd,
    cmpcs: Cmpcs,
    cmpds: Cmpds,
    aqctl: Aqctl,
    _reserved14: [u8; 0x0c],
    socen: Socen,
    socsel: Socsel,
    socperiod: Socperiod,
    soccnt: Soccnt,
    socflag: Socflag,
    socclr: Socclr,
    _reserved20: [u8; 0x08],
    etsel: Etsel,
    etperiod: Etperiod,
    etcnt: Etcnt,
    _reserved23: [u8; 0x04],
    inten: Inten,
    intflag: Intflag,
    intclr: Intclr,
    intfrc: Intfrc,
    tzsel: Tzsel,
    _reserved28: [u8; 0x08],
    tzctl: Tzctl,
    tzcbcostflag: Tzcbcostflag,
    tzcbcostclr: Tzcbcostclr,
    _reserved31: [u8; 0x08],
    dbctl: Dbctl,
    _reserved32: [u8; 0x0c],
    dbfed: Dbfed,
    dbred: Dbred,
    dbfeds: Dbfeds,
    dbreds: Dbreds,
    _reserved36: [u8; 0x10],
    gldctl: Gldctl,
    gldoshtctl: Gldoshtctl,
    gldoshtsts: Gldoshtsts,
    _reserved39: [u8; 0x04],
    pwm1_cmpa: Pwm1Cmpa,
    pwm1_cmpas: Pwm1Cmpas,
    pwm1_cmpb: Pwm1Cmpb,
    pwm1_cmpbs: Pwm1Cmpbs,
    _reserved43: [u8; 0x10],
    pwm1_aqctla: Pwm1Aqctla,
    pwm1_aqctlas: Pwm1Aqctlas,
    pwm1_aqctlb: Pwm1Aqctlb,
    pwm1_aqctlbs: Pwm1Aqctlbs,
    pwm1_aqsfrc: Pwm1Aqsfrc,
    pwm1_aqotsfrc: Pwm1Aqotsfrc,
    _reserved49: [u8; 0x01c8],
    pwm2_cmpa: Pwm2Cmpa,
    pwm2_cmpas: Pwm2Cmpas,
    pwm2_cmpb: Pwm2Cmpb,
    pwm2_cmpbs: Pwm2Cmpbs,
    _reserved53: [u8; 0x10],
    pwm2_aqctla: Pwm2Aqctla,
    pwm2_aqctlas: Pwm2Aqctlas,
    pwm2_aqctlb: Pwm2Aqctlb,
    pwm2_aqctlbs: Pwm2Aqctlbs,
    pwm2_aqsfrc: Pwm2Aqsfrc,
    pwm2_aqotsfrc: Pwm2Aqotsfrc,
    _reserved59: [u8; 0x01c8],
    pwm3_cmpa: Pwm3Cmpa,
    pwm3_cmpas: Pwm3Cmpas,
    pwm3_cmpb: Pwm3Cmpb,
    pwm3_cmpbs: Pwm3Cmpbs,
    _reserved63: [u8; 0x10],
    pwm3_aqctla: Pwm3Aqctla,
    pwm3_aqctlas: Pwm3Aqctlas,
    pwm3_aqctlb: Pwm3Aqctlb,
    pwm3_aqctlbs: Pwm3Aqctlbs,
    pwm3_aqsfrc: Pwm3Aqsfrc,
    pwm3_aqotsfrc: Pwm3Aqotsfrc,
}
impl RegisterBlock {
    #[doc = "0x00 - IP revision id register"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x10 - Time base control register"]
    #[inline(always)]
    pub const fn tbctl(&self) -> &Tbctl {
        &self.tbctl
    }
    #[doc = "0x14 - Time base period register"]
    #[inline(always)]
    pub const fn tbprd(&self) -> &Tbprd {
        &self.tbprd
    }
    #[doc = "0x18 - Time base period shadow register"]
    #[inline(always)]
    pub const fn tbprds(&self) -> &Tbprds {
        &self.tbprds
    }
    #[doc = "0x1c - Time base phase offset register"]
    #[inline(always)]
    pub const fn tbphs(&self) -> &Tbphs {
        &self.tbphs
    }
    #[doc = "0x20 - Time base status register"]
    #[inline(always)]
    pub const fn tbsts(&self) -> &Tbsts {
        &self.tbsts
    }
    #[doc = "0x24 - Time base status clear register"]
    #[inline(always)]
    pub const fn tbstsclr(&self) -> &Tbstsclr {
        &self.tbstsclr
    }
    #[doc = "0x28 - Time base counter register"]
    #[inline(always)]
    pub const fn tbctr(&self) -> &Tbctr {
        &self.tbctr
    }
    #[doc = "0x30 - Counter compare control register"]
    #[inline(always)]
    pub const fn cmpctl(&self) -> &Cmpctl {
        &self.cmpctl
    }
    #[doc = "0x40 - Counter compare C register"]
    #[inline(always)]
    pub const fn cmpc(&self) -> &Cmpc {
        &self.cmpc
    }
    #[doc = "0x44 - Counter compare D register"]
    #[inline(always)]
    pub const fn cmpd(&self) -> &Cmpd {
        &self.cmpd
    }
    #[doc = "0x48 - Counter compare C shadow register"]
    #[inline(always)]
    pub const fn cmpcs(&self) -> &Cmpcs {
        &self.cmpcs
    }
    #[doc = "0x4c - Counter compare D shadow register"]
    #[inline(always)]
    pub const fn cmpds(&self) -> &Cmpds {
        &self.cmpds
    }
    #[doc = "0x50 - Action qualifier control register"]
    #[inline(always)]
    pub const fn aqctl(&self) -> &Aqctl {
        &self.aqctl
    }
    #[doc = "0x60 - Start of conversion enable"]
    #[inline(always)]
    pub const fn socen(&self) -> &Socen {
        &self.socen
    }
    #[doc = "0x64 - Start of conversion selection"]
    #[inline(always)]
    pub const fn socsel(&self) -> &Socsel {
        &self.socsel
    }
    #[doc = "0x68 - Start of conversion period"]
    #[inline(always)]
    pub const fn socperiod(&self) -> &Socperiod {
        &self.socperiod
    }
    #[doc = "0x6c - Start of conversion count"]
    #[inline(always)]
    pub const fn soccnt(&self) -> &Soccnt {
        &self.soccnt
    }
    #[doc = "0x70 - Start of conversion flag"]
    #[inline(always)]
    pub const fn socflag(&self) -> &Socflag {
        &self.socflag
    }
    #[doc = "0x74 - Start of conversion clear"]
    #[inline(always)]
    pub const fn socclr(&self) -> &Socclr {
        &self.socclr
    }
    #[doc = "0x80 - Event trigger selection"]
    #[inline(always)]
    pub const fn etsel(&self) -> &Etsel {
        &self.etsel
    }
    #[doc = "0x84 - Event trigger period"]
    #[inline(always)]
    pub const fn etperiod(&self) -> &Etperiod {
        &self.etperiod
    }
    #[doc = "0x88 - Event trigger count"]
    #[inline(always)]
    pub const fn etcnt(&self) -> &Etcnt {
        &self.etcnt
    }
    #[doc = "0x90 - Interrupt enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x94 - Interrupt flag"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x98 - Interrupt clear"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x9c - Interrupt force"]
    #[inline(always)]
    pub const fn intfrc(&self) -> &Intfrc {
        &self.intfrc
    }
    #[doc = "0xa0 - Trip Zone selection"]
    #[inline(always)]
    pub const fn tzsel(&self) -> &Tzsel {
        &self.tzsel
    }
    #[doc = "0xac - Trip Zone control"]
    #[inline(always)]
    pub const fn tzctl(&self) -> &Tzctl {
        &self.tzctl
    }
    #[doc = "0xb0 - Trip zone CBCOST flag"]
    #[inline(always)]
    pub const fn tzcbcostflag(&self) -> &Tzcbcostflag {
        &self.tzcbcostflag
    }
    #[doc = "0xb4 - Trip zone CBCOST flag clear"]
    #[inline(always)]
    pub const fn tzcbcostclr(&self) -> &Tzcbcostclr {
        &self.tzcbcostclr
    }
    #[doc = "0xc0 - Dead band control register"]
    #[inline(always)]
    pub const fn dbctl(&self) -> &Dbctl {
        &self.dbctl
    }
    #[doc = "0xd0 - Dead band fall edge delay"]
    #[inline(always)]
    pub const fn dbfed(&self) -> &Dbfed {
        &self.dbfed
    }
    #[doc = "0xd4 - Dead band rise edge delay"]
    #[inline(always)]
    pub const fn dbred(&self) -> &Dbred {
        &self.dbred
    }
    #[doc = "0xd8 - Dead band fall edge delay shadow register"]
    #[inline(always)]
    pub const fn dbfeds(&self) -> &Dbfeds {
        &self.dbfeds
    }
    #[doc = "0xdc - Dead band rise edge delay shadow register"]
    #[inline(always)]
    pub const fn dbreds(&self) -> &Dbreds {
        &self.dbreds
    }
    #[doc = "0xf0 - Global load control register"]
    #[inline(always)]
    pub const fn gldctl(&self) -> &Gldctl {
        &self.gldctl
    }
    #[doc = "0xf4 - Global load one shot control register"]
    #[inline(always)]
    pub const fn gldoshtctl(&self) -> &Gldoshtctl {
        &self.gldoshtctl
    }
    #[doc = "0xf8 - Global load one shot status register"]
    #[inline(always)]
    pub const fn gldoshtsts(&self) -> &Gldoshtsts {
        &self.gldoshtsts
    }
    #[doc = "0x100 - PWM1 counter compare A register"]
    #[inline(always)]
    pub const fn pwm1_cmpa(&self) -> &Pwm1Cmpa {
        &self.pwm1_cmpa
    }
    #[doc = "0x104 - PWM1 counter compare A shadow register"]
    #[inline(always)]
    pub const fn pwm1_cmpas(&self) -> &Pwm1Cmpas {
        &self.pwm1_cmpas
    }
    #[doc = "0x108 - PWM1 counter compare B register"]
    #[inline(always)]
    pub const fn pwm1_cmpb(&self) -> &Pwm1Cmpb {
        &self.pwm1_cmpb
    }
    #[doc = "0x10c - PWM1 counter compare B shadow register"]
    #[inline(always)]
    pub const fn pwm1_cmpbs(&self) -> &Pwm1Cmpbs {
        &self.pwm1_cmpbs
    }
    #[doc = "0x120 - PWM1 action qualifier A register"]
    #[inline(always)]
    pub const fn pwm1_aqctla(&self) -> &Pwm1Aqctla {
        &self.pwm1_aqctla
    }
    #[doc = "0x124 - PWM1 action qualifier A shadow register"]
    #[inline(always)]
    pub const fn pwm1_aqctlas(&self) -> &Pwm1Aqctlas {
        &self.pwm1_aqctlas
    }
    #[doc = "0x128 - PWM1 action qualifier B register"]
    #[inline(always)]
    pub const fn pwm1_aqctlb(&self) -> &Pwm1Aqctlb {
        &self.pwm1_aqctlb
    }
    #[doc = "0x12c - PWM1 action qualifier B shadow register"]
    #[inline(always)]
    pub const fn pwm1_aqctlbs(&self) -> &Pwm1Aqctlbs {
        &self.pwm1_aqctlbs
    }
    #[doc = "0x130 - PWM1 action qualifier software force"]
    #[inline(always)]
    pub const fn pwm1_aqsfrc(&self) -> &Pwm1Aqsfrc {
        &self.pwm1_aqsfrc
    }
    #[doc = "0x134 - PWM1 action qualifier one time software force"]
    #[inline(always)]
    pub const fn pwm1_aqotsfrc(&self) -> &Pwm1Aqotsfrc {
        &self.pwm1_aqotsfrc
    }
    #[doc = "0x300 - PWM2 counter compare A register"]
    #[inline(always)]
    pub const fn pwm2_cmpa(&self) -> &Pwm2Cmpa {
        &self.pwm2_cmpa
    }
    #[doc = "0x304 - PWM2 counter compare A shadow register"]
    #[inline(always)]
    pub const fn pwm2_cmpas(&self) -> &Pwm2Cmpas {
        &self.pwm2_cmpas
    }
    #[doc = "0x308 - PWM2 counter compare B register"]
    #[inline(always)]
    pub const fn pwm2_cmpb(&self) -> &Pwm2Cmpb {
        &self.pwm2_cmpb
    }
    #[doc = "0x30c - PWM2 counter compare B shadow register"]
    #[inline(always)]
    pub const fn pwm2_cmpbs(&self) -> &Pwm2Cmpbs {
        &self.pwm2_cmpbs
    }
    #[doc = "0x320 - PWM2 action qualifier A register"]
    #[inline(always)]
    pub const fn pwm2_aqctla(&self) -> &Pwm2Aqctla {
        &self.pwm2_aqctla
    }
    #[doc = "0x324 - PWM2 action qualifier A shadow register"]
    #[inline(always)]
    pub const fn pwm2_aqctlas(&self) -> &Pwm2Aqctlas {
        &self.pwm2_aqctlas
    }
    #[doc = "0x328 - PWM2 action qualifier B register"]
    #[inline(always)]
    pub const fn pwm2_aqctlb(&self) -> &Pwm2Aqctlb {
        &self.pwm2_aqctlb
    }
    #[doc = "0x32c - PWM2 action qualifier B shadow register"]
    #[inline(always)]
    pub const fn pwm2_aqctlbs(&self) -> &Pwm2Aqctlbs {
        &self.pwm2_aqctlbs
    }
    #[doc = "0x330 - PWM2 action qualifier software force"]
    #[inline(always)]
    pub const fn pwm2_aqsfrc(&self) -> &Pwm2Aqsfrc {
        &self.pwm2_aqsfrc
    }
    #[doc = "0x334 - PWM2 action qualifier one time software force"]
    #[inline(always)]
    pub const fn pwm2_aqotsfrc(&self) -> &Pwm2Aqotsfrc {
        &self.pwm2_aqotsfrc
    }
    #[doc = "0x500 - PWM3 counter compare A register"]
    #[inline(always)]
    pub const fn pwm3_cmpa(&self) -> &Pwm3Cmpa {
        &self.pwm3_cmpa
    }
    #[doc = "0x504 - PWM3 counter compare A shadow register"]
    #[inline(always)]
    pub const fn pwm3_cmpas(&self) -> &Pwm3Cmpas {
        &self.pwm3_cmpas
    }
    #[doc = "0x508 - PWM3 counter compare B register"]
    #[inline(always)]
    pub const fn pwm3_cmpb(&self) -> &Pwm3Cmpb {
        &self.pwm3_cmpb
    }
    #[doc = "0x50c - PWM3 counter compare B shadow register"]
    #[inline(always)]
    pub const fn pwm3_cmpbs(&self) -> &Pwm3Cmpbs {
        &self.pwm3_cmpbs
    }
    #[doc = "0x520 - PWM3 action qualifier A register"]
    #[inline(always)]
    pub const fn pwm3_aqctla(&self) -> &Pwm3Aqctla {
        &self.pwm3_aqctla
    }
    #[doc = "0x524 - PWM3 action qualifier A shadow register"]
    #[inline(always)]
    pub const fn pwm3_aqctlas(&self) -> &Pwm3Aqctlas {
        &self.pwm3_aqctlas
    }
    #[doc = "0x528 - PWM3 action qualifier B register"]
    #[inline(always)]
    pub const fn pwm3_aqctlb(&self) -> &Pwm3Aqctlb {
        &self.pwm3_aqctlb
    }
    #[doc = "0x52c - PWM3 action qualifier B shadow register"]
    #[inline(always)]
    pub const fn pwm3_aqctlbs(&self) -> &Pwm3Aqctlbs {
        &self.pwm3_aqctlbs
    }
    #[doc = "0x530 - PWM3 action qualifier software force"]
    #[inline(always)]
    pub const fn pwm3_aqsfrc(&self) -> &Pwm3Aqsfrc {
        &self.pwm3_aqsfrc
    }
    #[doc = "0x534 - PWM3 action qualifier one time software force"]
    #[inline(always)]
    pub const fn pwm3_aqotsfrc(&self) -> &Pwm3Aqotsfrc {
        &self.pwm3_aqotsfrc
    }
}
#[doc = "REVISION (rw) register accessor: IP revision id register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`] module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "IP revision id register"]
pub mod revision;
#[doc = "TBCTL (rw) register accessor: Time base control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl`] module"]
#[doc(alias = "TBCTL")]
pub type Tbctl = crate::Reg<tbctl::TbctlSpec>;
#[doc = "Time base control register"]
pub mod tbctl;
#[doc = "TBPRD (rw) register accessor: Time base period register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbprd`] module"]
#[doc(alias = "TBPRD")]
pub type Tbprd = crate::Reg<tbprd::TbprdSpec>;
#[doc = "Time base period register"]
pub mod tbprd;
#[doc = "TBPRDS (rw) register accessor: Time base period shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbprds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbprds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbprds`] module"]
#[doc(alias = "TBPRDS")]
pub type Tbprds = crate::Reg<tbprds::TbprdsSpec>;
#[doc = "Time base period shadow register"]
pub mod tbprds;
#[doc = "TBPHS (rw) register accessor: Time base phase offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbphs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbphs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbphs`] module"]
#[doc(alias = "TBPHS")]
pub type Tbphs = crate::Reg<tbphs::TbphsSpec>;
#[doc = "Time base phase offset register"]
pub mod tbphs;
#[doc = "TBSTS (rw) register accessor: Time base status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbsts`] module"]
#[doc(alias = "TBSTS")]
pub type Tbsts = crate::Reg<tbsts::TbstsSpec>;
#[doc = "Time base status register"]
pub mod tbsts;
#[doc = "TBSTSCLR (rw) register accessor: Time base status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbstsclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbstsclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbstsclr`] module"]
#[doc(alias = "TBSTSCLR")]
pub type Tbstsclr = crate::Reg<tbstsclr::TbstsclrSpec>;
#[doc = "Time base status clear register"]
pub mod tbstsclr;
#[doc = "TBCTR (rw) register accessor: Time base counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctr`] module"]
#[doc(alias = "TBCTR")]
pub type Tbctr = crate::Reg<tbctr::TbctrSpec>;
#[doc = "Time base counter register"]
pub mod tbctr;
#[doc = "CMPCTL (rw) register accessor: Counter compare control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctl`] module"]
#[doc(alias = "CMPCTL")]
pub type Cmpctl = crate::Reg<cmpctl::CmpctlSpec>;
#[doc = "Counter compare control register"]
pub mod cmpctl;
#[doc = "CMPC (rw) register accessor: Counter compare C register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpc`] module"]
#[doc(alias = "CMPC")]
pub type Cmpc = crate::Reg<cmpc::CmpcSpec>;
#[doc = "Counter compare C register"]
pub mod cmpc;
#[doc = "CMPD (rw) register accessor: Counter compare D register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpd`] module"]
#[doc(alias = "CMPD")]
pub type Cmpd = crate::Reg<cmpd::CmpdSpec>;
#[doc = "Counter compare D register"]
pub mod cmpd;
#[doc = "CMPCS (rw) register accessor: Counter compare C shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpcs`] module"]
#[doc(alias = "CMPCS")]
pub type Cmpcs = crate::Reg<cmpcs::CmpcsSpec>;
#[doc = "Counter compare C shadow register"]
pub mod cmpcs;
#[doc = "CMPDS (rw) register accessor: Counter compare D shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpds`] module"]
#[doc(alias = "CMPDS")]
pub type Cmpds = crate::Reg<cmpds::CmpdsSpec>;
#[doc = "Counter compare D shadow register"]
pub mod cmpds;
#[doc = "AQCTL (rw) register accessor: Action qualifier control register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqctl`] module"]
#[doc(alias = "AQCTL")]
pub type Aqctl = crate::Reg<aqctl::AqctlSpec>;
#[doc = "Action qualifier control register"]
pub mod aqctl;
#[doc = "SOCEN (rw) register accessor: Start of conversion enable\n\nYou can [`read`](crate::Reg::read) this register and get [`socen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socen`] module"]
#[doc(alias = "SOCEN")]
pub type Socen = crate::Reg<socen::SocenSpec>;
#[doc = "Start of conversion enable"]
pub mod socen;
#[doc = "SOCSEL (rw) register accessor: Start of conversion selection\n\nYou can [`read`](crate::Reg::read) this register and get [`socsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socsel`] module"]
#[doc(alias = "SOCSEL")]
pub type Socsel = crate::Reg<socsel::SocselSpec>;
#[doc = "Start of conversion selection"]
pub mod socsel;
#[doc = "SOCPERIOD (rw) register accessor: Start of conversion period\n\nYou can [`read`](crate::Reg::read) this register and get [`socperiod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socperiod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socperiod`] module"]
#[doc(alias = "SOCPERIOD")]
pub type Socperiod = crate::Reg<socperiod::SocperiodSpec>;
#[doc = "Start of conversion period"]
pub mod socperiod;
#[doc = "SOCCNT (rw) register accessor: Start of conversion count\n\nYou can [`read`](crate::Reg::read) this register and get [`soccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soccnt`] module"]
#[doc(alias = "SOCCNT")]
pub type Soccnt = crate::Reg<soccnt::SoccntSpec>;
#[doc = "Start of conversion count"]
pub mod soccnt;
#[doc = "SOCFLAG (rw) register accessor: Start of conversion flag\n\nYou can [`read`](crate::Reg::read) this register and get [`socflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socflag`] module"]
#[doc(alias = "SOCFLAG")]
pub type Socflag = crate::Reg<socflag::SocflagSpec>;
#[doc = "Start of conversion flag"]
pub mod socflag;
#[doc = "SOCCLR (rw) register accessor: Start of conversion clear\n\nYou can [`read`](crate::Reg::read) this register and get [`socclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socclr`] module"]
#[doc(alias = "SOCCLR")]
pub type Socclr = crate::Reg<socclr::SocclrSpec>;
#[doc = "Start of conversion clear"]
pub mod socclr;
#[doc = "ETSEL (rw) register accessor: Event trigger selection\n\nYou can [`read`](crate::Reg::read) this register and get [`etsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etsel`] module"]
#[doc(alias = "ETSEL")]
pub type Etsel = crate::Reg<etsel::EtselSpec>;
#[doc = "Event trigger selection"]
pub mod etsel;
#[doc = "ETPERIOD (rw) register accessor: Event trigger period\n\nYou can [`read`](crate::Reg::read) this register and get [`etperiod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etperiod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etperiod`] module"]
#[doc(alias = "ETPERIOD")]
pub type Etperiod = crate::Reg<etperiod::EtperiodSpec>;
#[doc = "Event trigger period"]
pub mod etperiod;
#[doc = "ETCNT (rw) register accessor: Event trigger count\n\nYou can [`read`](crate::Reg::read) this register and get [`etcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etcnt`] module"]
#[doc(alias = "ETCNT")]
pub type Etcnt = crate::Reg<etcnt::EtcntSpec>;
#[doc = "Event trigger count"]
pub mod etcnt;
#[doc = "INTEN (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable"]
pub mod inten;
#[doc = "INTFLAG (rw) register accessor: Interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt flag"]
pub mod intflag;
#[doc = "INTCLR (rw) register accessor: Interrupt clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`] module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Interrupt clear"]
pub mod intclr;
#[doc = "INTFRC (rw) register accessor: Interrupt force\n\nYou can [`read`](crate::Reg::read) this register and get [`intfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfrc`] module"]
#[doc(alias = "INTFRC")]
pub type Intfrc = crate::Reg<intfrc::IntfrcSpec>;
#[doc = "Interrupt force"]
pub mod intfrc;
#[doc = "TZSEL (rw) register accessor: Trip Zone selection\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsel`] module"]
#[doc(alias = "TZSEL")]
pub type Tzsel = crate::Reg<tzsel::TzselSpec>;
#[doc = "Trip Zone selection"]
pub mod tzsel;
#[doc = "TZCTL (rw) register accessor: Trip Zone control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzctl`] module"]
#[doc(alias = "TZCTL")]
pub type Tzctl = crate::Reg<tzctl::TzctlSpec>;
#[doc = "Trip Zone control"]
pub mod tzctl;
#[doc = "TZCBCOSTFLAG (rw) register accessor: Trip zone CBCOST flag\n\nYou can [`read`](crate::Reg::read) this register and get [`tzcbcostflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzcbcostflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzcbcostflag`] module"]
#[doc(alias = "TZCBCOSTFLAG")]
pub type Tzcbcostflag = crate::Reg<tzcbcostflag::TzcbcostflagSpec>;
#[doc = "Trip zone CBCOST flag"]
pub mod tzcbcostflag;
#[doc = "TZCBCOSTCLR (rw) register accessor: Trip zone CBCOST flag clear\n\nYou can [`read`](crate::Reg::read) this register and get [`tzcbcostclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzcbcostclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzcbcostclr`] module"]
#[doc(alias = "TZCBCOSTCLR")]
pub type Tzcbcostclr = crate::Reg<tzcbcostclr::TzcbcostclrSpec>;
#[doc = "Trip zone CBCOST flag clear"]
pub mod tzcbcostclr;
#[doc = "DBCTL (rw) register accessor: Dead band control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbctl`] module"]
#[doc(alias = "DBCTL")]
pub type Dbctl = crate::Reg<dbctl::DbctlSpec>;
#[doc = "Dead band control register"]
pub mod dbctl;
#[doc = "DBFED (rw) register accessor: Dead band fall edge delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbfed`] module"]
#[doc(alias = "DBFED")]
pub type Dbfed = crate::Reg<dbfed::DbfedSpec>;
#[doc = "Dead band fall edge delay"]
pub mod dbfed;
#[doc = "DBRED (rw) register accessor: Dead band rise edge delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dbred::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbred::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbred`] module"]
#[doc(alias = "DBRED")]
pub type Dbred = crate::Reg<dbred::DbredSpec>;
#[doc = "Dead band rise edge delay"]
pub mod dbred;
#[doc = "DBFEDS (rw) register accessor: Dead band fall edge delay shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfeds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfeds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbfeds`] module"]
#[doc(alias = "DBFEDS")]
pub type Dbfeds = crate::Reg<dbfeds::DbfedsSpec>;
#[doc = "Dead band fall edge delay shadow register"]
pub mod dbfeds;
#[doc = "DBREDS (rw) register accessor: Dead band rise edge delay shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbreds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbreds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbreds`] module"]
#[doc(alias = "DBREDS")]
pub type Dbreds = crate::Reg<dbreds::DbredsSpec>;
#[doc = "Dead band rise edge delay shadow register"]
pub mod dbreds;
#[doc = "GLDCTL (rw) register accessor: Global load control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gldctl`] module"]
#[doc(alias = "GLDCTL")]
pub type Gldctl = crate::Reg<gldctl::GldctlSpec>;
#[doc = "Global load control register"]
pub mod gldctl;
#[doc = "GLDOSHTCTL (rw) register accessor: Global load one shot control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldoshtctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldoshtctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gldoshtctl`] module"]
#[doc(alias = "GLDOSHTCTL")]
pub type Gldoshtctl = crate::Reg<gldoshtctl::GldoshtctlSpec>;
#[doc = "Global load one shot control register"]
pub mod gldoshtctl;
#[doc = "GLDOSHTSTS (rw) register accessor: Global load one shot status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gldoshtsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gldoshtsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gldoshtsts`] module"]
#[doc(alias = "GLDOSHTSTS")]
pub type Gldoshtsts = crate::Reg<gldoshtsts::GldoshtstsSpec>;
#[doc = "Global load one shot status register"]
pub mod gldoshtsts;
#[doc = "PWM1_CMPA (rw) register accessor: PWM1 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_cmpa`] module"]
#[doc(alias = "PWM1_CMPA")]
pub type Pwm1Cmpa = crate::Reg<pwm1_cmpa::Pwm1CmpaSpec>;
#[doc = "PWM1 counter compare A register"]
pub mod pwm1_cmpa;
#[doc = "PWM1_CMPAS (rw) register accessor: PWM1 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_cmpas`] module"]
#[doc(alias = "PWM1_CMPAS")]
pub type Pwm1Cmpas = crate::Reg<pwm1_cmpas::Pwm1CmpasSpec>;
#[doc = "PWM1 counter compare A shadow register"]
pub mod pwm1_cmpas;
#[doc = "PWM1_CMPB (rw) register accessor: PWM1 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_cmpb`] module"]
#[doc(alias = "PWM1_CMPB")]
pub type Pwm1Cmpb = crate::Reg<pwm1_cmpb::Pwm1CmpbSpec>;
#[doc = "PWM1 counter compare B register"]
pub mod pwm1_cmpb;
#[doc = "PWM1_CMPBS (rw) register accessor: PWM1 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_cmpbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_cmpbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_cmpbs`] module"]
#[doc(alias = "PWM1_CMPBS")]
pub type Pwm1Cmpbs = crate::Reg<pwm1_cmpbs::Pwm1CmpbsSpec>;
#[doc = "PWM1 counter compare B shadow register"]
pub mod pwm1_cmpbs;
#[doc = "PWM1_AQCTLA (rw) register accessor: PWM1 action qualifier A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqctla`] module"]
#[doc(alias = "PWM1_AQCTLA")]
pub type Pwm1Aqctla = crate::Reg<pwm1_aqctla::Pwm1AqctlaSpec>;
#[doc = "PWM1 action qualifier A register"]
pub mod pwm1_aqctla;
#[doc = "PWM1_AQCTLAS (rw) register accessor: PWM1 action qualifier A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqctlas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqctlas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqctlas`] module"]
#[doc(alias = "PWM1_AQCTLAS")]
pub type Pwm1Aqctlas = crate::Reg<pwm1_aqctlas::Pwm1AqctlasSpec>;
#[doc = "PWM1 action qualifier A shadow register"]
pub mod pwm1_aqctlas;
#[doc = "PWM1_AQCTLB (rw) register accessor: PWM1 action qualifier B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqctlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqctlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqctlb`] module"]
#[doc(alias = "PWM1_AQCTLB")]
pub type Pwm1Aqctlb = crate::Reg<pwm1_aqctlb::Pwm1AqctlbSpec>;
#[doc = "PWM1 action qualifier B register"]
pub mod pwm1_aqctlb;
#[doc = "PWM1_AQCTLBS (rw) register accessor: PWM1 action qualifier B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqctlbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqctlbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqctlbs`] module"]
#[doc(alias = "PWM1_AQCTLBS")]
pub type Pwm1Aqctlbs = crate::Reg<pwm1_aqctlbs::Pwm1AqctlbsSpec>;
#[doc = "PWM1 action qualifier B shadow register"]
pub mod pwm1_aqctlbs;
#[doc = "PWM1_AQSFRC (rw) register accessor: PWM1 action qualifier software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqsfrc`] module"]
#[doc(alias = "PWM1_AQSFRC")]
pub type Pwm1Aqsfrc = crate::Reg<pwm1_aqsfrc::Pwm1AqsfrcSpec>;
#[doc = "PWM1 action qualifier software force"]
pub mod pwm1_aqsfrc;
#[doc = "PWM1_AQOTSFRC (rw) register accessor: PWM1 action qualifier one time software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_aqotsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_aqotsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_aqotsfrc`] module"]
#[doc(alias = "PWM1_AQOTSFRC")]
pub type Pwm1Aqotsfrc = crate::Reg<pwm1_aqotsfrc::Pwm1AqotsfrcSpec>;
#[doc = "PWM1 action qualifier one time software force"]
pub mod pwm1_aqotsfrc;
#[doc = "PWM2_CMPA (rw) register accessor: PWM2 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_cmpa`] module"]
#[doc(alias = "PWM2_CMPA")]
pub type Pwm2Cmpa = crate::Reg<pwm2_cmpa::Pwm2CmpaSpec>;
#[doc = "PWM2 counter compare A register"]
pub mod pwm2_cmpa;
#[doc = "PWM2_CMPAS (rw) register accessor: PWM2 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_cmpas`] module"]
#[doc(alias = "PWM2_CMPAS")]
pub type Pwm2Cmpas = crate::Reg<pwm2_cmpas::Pwm2CmpasSpec>;
#[doc = "PWM2 counter compare A shadow register"]
pub mod pwm2_cmpas;
#[doc = "PWM2_CMPB (rw) register accessor: PWM2 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_cmpb`] module"]
#[doc(alias = "PWM2_CMPB")]
pub type Pwm2Cmpb = crate::Reg<pwm2_cmpb::Pwm2CmpbSpec>;
#[doc = "PWM2 counter compare B register"]
pub mod pwm2_cmpb;
#[doc = "PWM2_CMPBS (rw) register accessor: PWM2 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_cmpbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_cmpbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_cmpbs`] module"]
#[doc(alias = "PWM2_CMPBS")]
pub type Pwm2Cmpbs = crate::Reg<pwm2_cmpbs::Pwm2CmpbsSpec>;
#[doc = "PWM2 counter compare B shadow register"]
pub mod pwm2_cmpbs;
#[doc = "PWM2_AQCTLA (rw) register accessor: PWM2 action qualifier A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqctla`] module"]
#[doc(alias = "PWM2_AQCTLA")]
pub type Pwm2Aqctla = crate::Reg<pwm2_aqctla::Pwm2AqctlaSpec>;
#[doc = "PWM2 action qualifier A register"]
pub mod pwm2_aqctla;
#[doc = "PWM2_AQCTLAS (rw) register accessor: PWM2 action qualifier A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqctlas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqctlas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqctlas`] module"]
#[doc(alias = "PWM2_AQCTLAS")]
pub type Pwm2Aqctlas = crate::Reg<pwm2_aqctlas::Pwm2AqctlasSpec>;
#[doc = "PWM2 action qualifier A shadow register"]
pub mod pwm2_aqctlas;
#[doc = "PWM2_AQCTLB (rw) register accessor: PWM2 action qualifier B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqctlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqctlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqctlb`] module"]
#[doc(alias = "PWM2_AQCTLB")]
pub type Pwm2Aqctlb = crate::Reg<pwm2_aqctlb::Pwm2AqctlbSpec>;
#[doc = "PWM2 action qualifier B register"]
pub mod pwm2_aqctlb;
#[doc = "PWM2_AQCTLBS (rw) register accessor: PWM2 action qualifier B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqctlbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqctlbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqctlbs`] module"]
#[doc(alias = "PWM2_AQCTLBS")]
pub type Pwm2Aqctlbs = crate::Reg<pwm2_aqctlbs::Pwm2AqctlbsSpec>;
#[doc = "PWM2 action qualifier B shadow register"]
pub mod pwm2_aqctlbs;
#[doc = "PWM2_AQSFRC (rw) register accessor: PWM2 action qualifier software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqsfrc`] module"]
#[doc(alias = "PWM2_AQSFRC")]
pub type Pwm2Aqsfrc = crate::Reg<pwm2_aqsfrc::Pwm2AqsfrcSpec>;
#[doc = "PWM2 action qualifier software force"]
pub mod pwm2_aqsfrc;
#[doc = "PWM2_AQOTSFRC (rw) register accessor: PWM2 action qualifier one time software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_aqotsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_aqotsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_aqotsfrc`] module"]
#[doc(alias = "PWM2_AQOTSFRC")]
pub type Pwm2Aqotsfrc = crate::Reg<pwm2_aqotsfrc::Pwm2AqotsfrcSpec>;
#[doc = "PWM2 action qualifier one time software force"]
pub mod pwm2_aqotsfrc;
#[doc = "PWM3_CMPA (rw) register accessor: PWM3 counter compare A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_cmpa`] module"]
#[doc(alias = "PWM3_CMPA")]
pub type Pwm3Cmpa = crate::Reg<pwm3_cmpa::Pwm3CmpaSpec>;
#[doc = "PWM3 counter compare A register"]
pub mod pwm3_cmpa;
#[doc = "PWM3_CMPAS (rw) register accessor: PWM3 counter compare A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_cmpas`] module"]
#[doc(alias = "PWM3_CMPAS")]
pub type Pwm3Cmpas = crate::Reg<pwm3_cmpas::Pwm3CmpasSpec>;
#[doc = "PWM3 counter compare A shadow register"]
pub mod pwm3_cmpas;
#[doc = "PWM3_CMPB (rw) register accessor: PWM3 counter compare B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_cmpb`] module"]
#[doc(alias = "PWM3_CMPB")]
pub type Pwm3Cmpb = crate::Reg<pwm3_cmpb::Pwm3CmpbSpec>;
#[doc = "PWM3 counter compare B register"]
pub mod pwm3_cmpb;
#[doc = "PWM3_CMPBS (rw) register accessor: PWM3 counter compare B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_cmpbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_cmpbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_cmpbs`] module"]
#[doc(alias = "PWM3_CMPBS")]
pub type Pwm3Cmpbs = crate::Reg<pwm3_cmpbs::Pwm3CmpbsSpec>;
#[doc = "PWM3 counter compare B shadow register"]
pub mod pwm3_cmpbs;
#[doc = "PWM3_AQCTLA (rw) register accessor: PWM3 action qualifier A register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqctla`] module"]
#[doc(alias = "PWM3_AQCTLA")]
pub type Pwm3Aqctla = crate::Reg<pwm3_aqctla::Pwm3AqctlaSpec>;
#[doc = "PWM3 action qualifier A register"]
pub mod pwm3_aqctla;
#[doc = "PWM3_AQCTLAS (rw) register accessor: PWM3 action qualifier A shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqctlas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqctlas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqctlas`] module"]
#[doc(alias = "PWM3_AQCTLAS")]
pub type Pwm3Aqctlas = crate::Reg<pwm3_aqctlas::Pwm3AqctlasSpec>;
#[doc = "PWM3 action qualifier A shadow register"]
pub mod pwm3_aqctlas;
#[doc = "PWM3_AQCTLB (rw) register accessor: PWM3 action qualifier B register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqctlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqctlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqctlb`] module"]
#[doc(alias = "PWM3_AQCTLB")]
pub type Pwm3Aqctlb = crate::Reg<pwm3_aqctlb::Pwm3AqctlbSpec>;
#[doc = "PWM3 action qualifier B register"]
pub mod pwm3_aqctlb;
#[doc = "PWM3_AQCTLBS (rw) register accessor: PWM3 action qualifier B shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqctlbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqctlbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqctlbs`] module"]
#[doc(alias = "PWM3_AQCTLBS")]
pub type Pwm3Aqctlbs = crate::Reg<pwm3_aqctlbs::Pwm3AqctlbsSpec>;
#[doc = "PWM3 action qualifier B shadow register"]
pub mod pwm3_aqctlbs;
#[doc = "PWM3_AQSFRC (rw) register accessor: PWM3 action qualifier software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqsfrc`] module"]
#[doc(alias = "PWM3_AQSFRC")]
pub type Pwm3Aqsfrc = crate::Reg<pwm3_aqsfrc::Pwm3AqsfrcSpec>;
#[doc = "PWM3 action qualifier software force"]
pub mod pwm3_aqsfrc;
#[doc = "PWM3_AQOTSFRC (rw) register accessor: PWM3 action qualifier one time software force\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_aqotsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_aqotsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_aqotsfrc`] module"]
#[doc(alias = "PWM3_AQOTSFRC")]
pub type Pwm3Aqotsfrc = crate::Reg<pwm3_aqotsfrc::Pwm3AqotsfrcSpec>;
#[doc = "PWM3 action qualifier one time software force"]
pub mod pwm3_aqotsfrc;

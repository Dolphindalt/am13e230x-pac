#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "COUNTERREGS"]
#[doc(alias = "COUNTERREGS")]
pub struct Counterregs {
    ctr: Ctr,
    ctrctl: Ctrctl,
    load: Load,
    _reserved3: [u8; 0x04],
    cc_01: [Cc01; 2],
    cc_23: [Cc23; 2],
    cc_45: [Cc45; 2],
    _reserved6: [u8; 0x08],
    ccctl_01: [Ccctl01; 2],
    ccctl_23: [Ccctl23; 2],
    ccctl_45: [Ccctl45; 2],
    _reserved9: [u8; 0x08],
    octl_01: [Octl01; 2],
    octl_23: [Octl23; 2],
    _reserved11: [u8; 0x10],
    ccact_01: [Ccact01; 2],
    ccact_23: [Ccact23; 2],
    ifctl_01: [Ifctl01; 2],
    ifctl_23: [Ifctl23; 2],
    _reserved15: [u8; 0x10],
    pl: Pl,
    dbctl: Dbctl,
    _reserved17: [u8; 0x08],
    tsel: Tsel,
    rc: Rc,
    rcld: Rcld,
    qdir: Qdir,
    _reserved21: [u8; 0x10],
    fctl: Fctl,
    fifctl: Fifctl,
}
impl Counterregs {
    #[doc = "0x00 - Counter Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x04 - Counter Control Register"]
    #[inline(always)]
    pub const fn ctrctl(&self) -> &Ctrctl {
        &self.ctrctl
    }
    #[doc = "0x08 - Load Register"]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    #[doc = "0x10..0x18 - Capture or Compare Register 0/1"]
    #[inline(always)]
    pub const fn cc_01(&self, n: usize) -> &Cc01 {
        &self.cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Capture or Compare Register 0/1"]
    #[inline(always)]
    pub fn cc_01_iter(&self) -> impl Iterator<Item = &Cc01> {
        self.cc_01.iter()
    }
    #[doc = "0x18..0x20 - Capture or Compare Register 2/3"]
    #[inline(always)]
    pub const fn cc_23(&self, n: usize) -> &Cc23 {
        &self.cc_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Capture or Compare Register 2/3"]
    #[inline(always)]
    pub fn cc_23_iter(&self) -> impl Iterator<Item = &Cc23> {
        self.cc_23.iter()
    }
    #[doc = "0x20..0x28 - The CC_45 register are a registers which can be used as compare to the current CTR to create an events CC4U, CC4D, CC5U and CC5D."]
    #[inline(always)]
    pub const fn cc_45(&self, n: usize) -> &Cc45 {
        &self.cc_45[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - The CC_45 register are a registers which can be used as compare to the current CTR to create an events CC4U, CC4D, CC5U and CC5D."]
    #[inline(always)]
    pub fn cc_45_iter(&self) -> impl Iterator<Item = &Cc45> {
        self.cc_45.iter()
    }
    #[doc = "0x30..0x38 - Capture or Compare Control Registers 0/1"]
    #[inline(always)]
    pub const fn ccctl_01(&self, n: usize) -> &Ccctl01 {
        &self.ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - Capture or Compare Control Registers 0/1"]
    #[inline(always)]
    pub fn ccctl_01_iter(&self) -> impl Iterator<Item = &Ccctl01> {
        self.ccctl_01.iter()
    }
    #[doc = "0x38..0x40 - Capture or Compare Control Registers 2/3"]
    #[inline(always)]
    pub const fn ccctl_23(&self, n: usize) -> &Ccctl23 {
        &self.ccctl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x40 - Capture or Compare Control Registers 2/3"]
    #[inline(always)]
    pub fn ccctl_23_iter(&self) -> impl Iterator<Item = &Ccctl23> {
        self.ccctl_23.iter()
    }
    #[doc = "0x40..0x48 - Capture or Compare Control Registers 4/5"]
    #[inline(always)]
    pub const fn ccctl_45(&self, n: usize) -> &Ccctl45 {
        &self.ccctl_45[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - Capture or Compare Control Registers 4/5"]
    #[inline(always)]
    pub fn ccctl_45_iter(&self) -> impl Iterator<Item = &Ccctl45> {
        self.ccctl_45.iter()
    }
    #[doc = "0x50..0x58 - CCP Output Control Registers 0/1"]
    #[inline(always)]
    pub const fn octl_01(&self, n: usize) -> &Octl01 {
        &self.octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - CCP Output Control Registers 0/1"]
    #[inline(always)]
    pub fn octl_01_iter(&self) -> impl Iterator<Item = &Octl01> {
        self.octl_01.iter()
    }
    #[doc = "0x58..0x60 - CCP Output Control Registers 2/3"]
    #[inline(always)]
    pub const fn octl_23(&self, n: usize) -> &Octl23 {
        &self.octl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x60 - CCP Output Control Registers 2/3"]
    #[inline(always)]
    pub fn octl_23_iter(&self) -> impl Iterator<Item = &Octl23> {
        self.octl_23.iter()
    }
    #[doc = "0x70..0x78 - Capture or Compare Action Registers 0/1"]
    #[inline(always)]
    pub const fn ccact_01(&self, n: usize) -> &Ccact01 {
        &self.ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x78 - Capture or Compare Action Registers 0/1"]
    #[inline(always)]
    pub fn ccact_01_iter(&self) -> impl Iterator<Item = &Ccact01> {
        self.ccact_01.iter()
    }
    #[doc = "0x78..0x80 - Capture or Compare Action Registers 2/3"]
    #[inline(always)]
    pub const fn ccact_23(&self, n: usize) -> &Ccact23 {
        &self.ccact_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x80 - Capture or Compare Action Registers 2/3"]
    #[inline(always)]
    pub fn ccact_23_iter(&self) -> impl Iterator<Item = &Ccact23> {
        self.ccact_23.iter()
    }
    #[doc = "0x80..0x88 - Input Filter Control Register 0/1"]
    #[inline(always)]
    pub const fn ifctl_01(&self, n: usize) -> &Ifctl01 {
        &self.ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Input Filter Control Register 0/1"]
    #[inline(always)]
    pub fn ifctl_01_iter(&self) -> impl Iterator<Item = &Ifctl01> {
        self.ifctl_01.iter()
    }
    #[doc = "0x88..0x90 - Input Filter Control Register 2/3"]
    #[inline(always)]
    pub const fn ifctl_23(&self, n: usize) -> &Ifctl23 {
        &self.ifctl_23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x90 - Input Filter Control Register 2/3"]
    #[inline(always)]
    pub fn ifctl_23_iter(&self) -> impl Iterator<Item = &Ifctl23> {
        self.ifctl_23.iter()
    }
    #[doc = "0xa0 - Phase Load Register"]
    #[inline(always)]
    pub const fn pl(&self) -> &Pl {
        &self.pl
    }
    #[doc = "0xa4 - Dead Band insertion control register"]
    #[inline(always)]
    pub const fn dbctl(&self) -> &Dbctl {
        &self.dbctl
    }
    #[doc = "0xb0 - Trigger Select Register"]
    #[inline(always)]
    pub const fn tsel(&self) -> &Tsel {
        &self.tsel
    }
    #[doc = "0xb4 - Repeat counter Register"]
    #[inline(always)]
    pub const fn rc(&self) -> &Rc {
        &self.rc
    }
    #[doc = "0xb8 - Repeat counter load Register"]
    #[inline(always)]
    pub const fn rcld(&self) -> &Rcld {
        &self.rcld
    }
    #[doc = "0xbc - QEI Count Direction Register"]
    #[inline(always)]
    pub const fn qdir(&self) -> &Qdir {
        &self.qdir
    }
    #[doc = "0xd0 - Fault Control Register"]
    #[inline(always)]
    pub const fn fctl(&self) -> &Fctl {
        &self.fctl
    }
    #[doc = "0xd4 - Fault input Filter control register"]
    #[inline(always)]
    pub const fn fifctl(&self) -> &Fifctl {
        &self.fifctl
    }
}
#[doc = "CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`] module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Counter Register"]
pub mod ctr;
#[doc = "CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrctl`] module"]
#[doc(alias = "CTRCTL")]
pub type Ctrctl = crate::Reg<ctrctl::CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod ctrctl;
#[doc = "LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`] module"]
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "Load Register"]
pub mod load;
#[doc = "CC_01 (rw) register accessor: Capture or Compare Register 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_01`] module"]
#[doc(alias = "CC_01")]
pub type Cc01 = crate::Reg<cc_01::Cc01Spec>;
#[doc = "Capture or Compare Register 0/1"]
pub mod cc_01;
#[doc = "CC_23 (rw) register accessor: Capture or Compare Register 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_23`] module"]
#[doc(alias = "CC_23")]
pub type Cc23 = crate::Reg<cc_23::Cc23Spec>;
#[doc = "Capture or Compare Register 2/3"]
pub mod cc_23;
#[doc = "CC_45 (rw) register accessor: The CC_45 register are a registers which can be used as compare to the current CTR to create an events CC4U, CC4D, CC5U and CC5D.\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_45`] module"]
#[doc(alias = "CC_45")]
pub type Cc45 = crate::Reg<cc_45::Cc45Spec>;
#[doc = "The CC_45 register are a registers which can be used as compare to the current CTR to create an events CC4U, CC4D, CC5U and CC5D."]
pub mod cc_45;
#[doc = "CCCTL_01 (rw) register accessor: Capture or Compare Control Registers 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccctl_01`] module"]
#[doc(alias = "CCCTL_01")]
pub type Ccctl01 = crate::Reg<ccctl_01::Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers 0/1"]
pub mod ccctl_01;
#[doc = "CCCTL_23 (rw) register accessor: Capture or Compare Control Registers 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccctl_23`] module"]
#[doc(alias = "CCCTL_23")]
pub type Ccctl23 = crate::Reg<ccctl_23::Ccctl23Spec>;
#[doc = "Capture or Compare Control Registers 2/3"]
pub mod ccctl_23;
#[doc = "CCCTL_45 (rw) register accessor: Capture or Compare Control Registers 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccctl_45`] module"]
#[doc(alias = "CCCTL_45")]
pub type Ccctl45 = crate::Reg<ccctl_45::Ccctl45Spec>;
#[doc = "Capture or Compare Control Registers 4/5"]
pub mod ccctl_45;
#[doc = "OCTL_01 (rw) register accessor: CCP Output Control Registers 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl_01`] module"]
#[doc(alias = "OCTL_01")]
pub type Octl01 = crate::Reg<octl_01::Octl01Spec>;
#[doc = "CCP Output Control Registers 0/1"]
pub mod octl_01;
#[doc = "OCTL_23 (rw) register accessor: CCP Output Control Registers 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl_23`] module"]
#[doc(alias = "OCTL_23")]
pub type Octl23 = crate::Reg<octl_23::Octl23Spec>;
#[doc = "CCP Output Control Registers 2/3"]
pub mod octl_23;
#[doc = "CCACT_01 (rw) register accessor: Capture or Compare Action Registers 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccact_01`] module"]
#[doc(alias = "CCACT_01")]
pub type Ccact01 = crate::Reg<ccact_01::Ccact01Spec>;
#[doc = "Capture or Compare Action Registers 0/1"]
pub mod ccact_01;
#[doc = "CCACT_23 (rw) register accessor: Capture or Compare Action Registers 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccact_23`] module"]
#[doc(alias = "CCACT_23")]
pub type Ccact23 = crate::Reg<ccact_23::Ccact23Spec>;
#[doc = "Capture or Compare Action Registers 2/3"]
pub mod ccact_23;
#[doc = "IFCTL_01 (rw) register accessor: Input Filter Control Register 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifctl_01`] module"]
#[doc(alias = "IFCTL_01")]
pub type Ifctl01 = crate::Reg<ifctl_01::Ifctl01Spec>;
#[doc = "Input Filter Control Register 0/1"]
pub mod ifctl_01;
#[doc = "IFCTL_23 (rw) register accessor: Input Filter Control Register 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifctl_23`] module"]
#[doc(alias = "IFCTL_23")]
pub type Ifctl23 = crate::Reg<ifctl_23::Ifctl23Spec>;
#[doc = "Input Filter Control Register 2/3"]
pub mod ifctl_23;
#[doc = "PL (rw) register accessor: Phase Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl`] module"]
#[doc(alias = "PL")]
pub type Pl = crate::Reg<pl::PlSpec>;
#[doc = "Phase Load Register"]
pub mod pl;
#[doc = "DBCTL (rw) register accessor: Dead Band insertion control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbctl`] module"]
#[doc(alias = "DBCTL")]
pub type Dbctl = crate::Reg<dbctl::DbctlSpec>;
#[doc = "Dead Band insertion control register"]
pub mod dbctl;
#[doc = "TSEL (rw) register accessor: Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsel`] module"]
#[doc(alias = "TSEL")]
pub type Tsel = crate::Reg<tsel::TselSpec>;
#[doc = "Trigger Select Register"]
pub mod tsel;
#[doc = "RC (r) register accessor: Repeat counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc`] module"]
#[doc(alias = "RC")]
pub type Rc = crate::Reg<rc::RcSpec>;
#[doc = "Repeat counter Register"]
pub mod rc;
#[doc = "RCLD (rw) register accessor: Repeat counter load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcld`] module"]
#[doc(alias = "RCLD")]
pub type Rcld = crate::Reg<rcld::RcldSpec>;
#[doc = "Repeat counter load Register"]
pub mod rcld;
#[doc = "QDIR (r) register accessor: QEI Count Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qdir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdir`] module"]
#[doc(alias = "QDIR")]
pub type Qdir = crate::Reg<qdir::QdirSpec>;
#[doc = "QEI Count Direction Register"]
pub mod qdir;
#[doc = "FCTL (rw) register accessor: Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl`] module"]
#[doc(alias = "FCTL")]
pub type Fctl = crate::Reg<fctl::FctlSpec>;
#[doc = "Fault Control Register"]
pub mod fctl;
#[doc = "FIFCTL (rw) register accessor: Fault input Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifctl`] module"]
#[doc(alias = "FIFCTL")]
pub type Fifctl = crate::Reg<fifctl::FifctlSpec>;
#[doc = "Fault input Filter control register"]
pub mod fifctl;

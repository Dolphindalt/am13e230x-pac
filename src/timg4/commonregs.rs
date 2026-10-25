#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "COMMONREGS"]
#[doc(alias = "COMMONREGS")]
pub struct Commonregs {
    ccpd: Ccpd,
    odis: Odis,
    cclkctl: Cclkctl,
    cps: Cps,
    cpsv: Cpsv,
    cttrigctl: Cttrigctl,
    _reserved6: [u8; 0x04],
    cttrig: Cttrig,
    fsctl: Fsctl,
    gctl: Gctl,
}
impl Commonregs {
    #[doc = "0x00 - CCP Direction"]
    #[inline(always)]
    pub const fn ccpd(&self) -> &Ccpd {
        &self.ccpd
    }
    #[doc = "0x04 - Output Disable"]
    #[inline(always)]
    pub const fn odis(&self) -> &Odis {
        &self.odis
    }
    #[doc = "0x08 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn cclkctl(&self) -> &Cclkctl {
        &self.cclkctl
    }
    #[doc = "0x0c - Clock Prescale Register"]
    #[inline(always)]
    pub const fn cps(&self) -> &Cps {
        &self.cps
    }
    #[doc = "0x10 - Clock prescale count status register"]
    #[inline(always)]
    pub const fn cpsv(&self) -> &Cpsv {
        &self.cpsv
    }
    #[doc = "0x14 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn cttrigctl(&self) -> &Cttrigctl {
        &self.cttrigctl
    }
    #[doc = "0x1c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn cttrig(&self) -> &Cttrig {
        &self.cttrig
    }
    #[doc = "0x20 - Fault Source Control"]
    #[inline(always)]
    pub const fn fsctl(&self) -> &Fsctl {
        &self.fsctl
    }
    #[doc = "0x24 - Global control register"]
    #[inline(always)]
    pub const fn gctl(&self) -> &Gctl {
        &self.gctl
    }
}
#[doc = "CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccpd`] module"]
#[doc(alias = "CCPD")]
pub type Ccpd = crate::Reg<ccpd::CcpdSpec>;
#[doc = "CCP Direction"]
pub mod ccpd;
#[doc = "ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odis`] module"]
#[doc(alias = "ODIS")]
pub type Odis = crate::Reg<odis::OdisSpec>;
#[doc = "Output Disable"]
pub mod odis;
#[doc = "CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclkctl`] module"]
#[doc(alias = "CCLKCTL")]
pub type Cclkctl = crate::Reg<cclkctl::CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod cclkctl;
#[doc = "CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cps`] module"]
#[doc(alias = "CPS")]
pub type Cps = crate::Reg<cps::CpsSpec>;
#[doc = "Clock Prescale Register"]
pub mod cps;
#[doc = "CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsv`] module"]
#[doc(alias = "CPSV")]
pub type Cpsv = crate::Reg<cpsv::CpsvSpec>;
#[doc = "Clock prescale count status register"]
pub mod cpsv;
#[doc = "CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrigctl`] module"]
#[doc(alias = "CTTRIGCTL")]
pub type Cttrigctl = crate::Reg<cttrigctl::CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod cttrigctl;
#[doc = "CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrig`] module"]
#[doc(alias = "CTTRIG")]
pub type Cttrig = crate::Reg<cttrig::CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod cttrig;
#[doc = "FSCTL (rw) register accessor: Fault Source Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsctl`] module"]
#[doc(alias = "FSCTL")]
pub type Fsctl = crate::Reg<fsctl::FsctlSpec>;
#[doc = "Fault Source Control"]
pub mod fsctl;
#[doc = "GCTL (rw) register accessor: Global control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctl`] module"]
#[doc(alias = "GCTL")]
pub type Gctl = crate::Reg<gctl::GctlSpec>;
#[doc = "Global control register"]
pub mod gctl;

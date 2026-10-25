#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    compctl: Compctl,
    comphysctl: Comphysctl,
    compsts: Compsts,
    compstsclr: Compstsclr,
    compdachctl: Compdachctl,
    _reserved5: [u8; 0x02],
    dachvals: Dachvals,
    dachvala: Dachvala,
    _reserved7: [u8; 0x14],
    daclvals: Daclvals,
    daclvala: Daclvala,
    _reserved9: [u8; 0x04],
    ctriplfilctl: Ctriplfilctl,
    ctriplfilclkctl: Ctriplfilclkctl,
    ctriphfilctl: Ctriphfilctl,
    ctriphfilclkctl: Ctriphfilclkctl,
    complock: Complock,
    _reserved14: [u8; 0x12],
    compdaclctl: Compdaclctl,
    _reserved15: [u8; 0x24],
    ctriplfilclkctl2: Ctriplfilclkctl2,
    _reserved16: [u8; 0x02],
    ctriphfilclkctl2: Ctriphfilclkctl2,
}
impl RegisterBlock {
    #[doc = "0x00 - CMPSS Comparator Control Register"]
    #[inline(always)]
    pub const fn compctl(&self) -> &Compctl {
        &self.compctl
    }
    #[doc = "0x02 - CMPSS Comparator Hysteresis Control Register"]
    #[inline(always)]
    pub const fn comphysctl(&self) -> &Comphysctl {
        &self.comphysctl
    }
    #[doc = "0x04 - CMPSS Comparator Status Register"]
    #[inline(always)]
    pub const fn compsts(&self) -> &Compsts {
        &self.compsts
    }
    #[doc = "0x06 - CMPSS Comparator Status Clear Register"]
    #[inline(always)]
    pub const fn compstsclr(&self) -> &Compstsclr {
        &self.compstsclr
    }
    #[doc = "0x08 - CMPSS High DAC Control Register"]
    #[inline(always)]
    pub const fn compdachctl(&self) -> &Compdachctl {
        &self.compdachctl
    }
    #[doc = "0x0c - CMPSS High DAC Value Shadow Register"]
    #[inline(always)]
    pub const fn dachvals(&self) -> &Dachvals {
        &self.dachvals
    }
    #[doc = "0x0e - CMPSS High DAC Value Active Register"]
    #[inline(always)]
    pub const fn dachvala(&self) -> &Dachvala {
        &self.dachvala
    }
    #[doc = "0x24 - CMPSS Low DAC Value Shadow Register"]
    #[inline(always)]
    pub const fn daclvals(&self) -> &Daclvals {
        &self.daclvals
    }
    #[doc = "0x26 - CMPSS Low DAC Value Active Register"]
    #[inline(always)]
    pub const fn daclvala(&self) -> &Daclvala {
        &self.daclvala
    }
    #[doc = "0x2c - CTRIPL Filter Control Register"]
    #[inline(always)]
    pub const fn ctriplfilctl(&self) -> &Ctriplfilctl {
        &self.ctriplfilctl
    }
    #[doc = "0x2e - CTRIPL Filter Clock Control Register"]
    #[inline(always)]
    pub const fn ctriplfilclkctl(&self) -> &Ctriplfilclkctl {
        &self.ctriplfilclkctl
    }
    #[doc = "0x30 - CTRIPH Filter Control Register"]
    #[inline(always)]
    pub const fn ctriphfilctl(&self) -> &Ctriphfilctl {
        &self.ctriphfilctl
    }
    #[doc = "0x32 - CTRIPH Filter Clock Control Register"]
    #[inline(always)]
    pub const fn ctriphfilclkctl(&self) -> &Ctriphfilclkctl {
        &self.ctriphfilclkctl
    }
    #[doc = "0x34 - CMPSS Lock Register"]
    #[inline(always)]
    pub const fn complock(&self) -> &Complock {
        &self.complock
    }
    #[doc = "0x48 - CMPSS Low DAC Control Register"]
    #[inline(always)]
    pub const fn compdaclctl(&self) -> &Compdaclctl {
        &self.compdaclctl
    }
    #[doc = "0x6e - CTRIPL Filter Clock Control Register 2"]
    #[inline(always)]
    pub const fn ctriplfilclkctl2(&self) -> &Ctriplfilclkctl2 {
        &self.ctriplfilclkctl2
    }
    #[doc = "0x72 - CTRIPH Filter Clock Control Register 2"]
    #[inline(always)]
    pub const fn ctriphfilclkctl2(&self) -> &Ctriphfilclkctl2 {
        &self.ctriphfilclkctl2
    }
}
#[doc = "COMPCTL (rw) register accessor: CMPSS Comparator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compctl`] module"]
#[doc(alias = "COMPCTL")]
pub type Compctl = crate::Reg<compctl::CompctlSpec>;
#[doc = "CMPSS Comparator Control Register"]
pub mod compctl;
#[doc = "COMPHYSCTL (rw) register accessor: CMPSS Comparator Hysteresis Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comphysctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comphysctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comphysctl`] module"]
#[doc(alias = "COMPHYSCTL")]
pub type Comphysctl = crate::Reg<comphysctl::ComphysctlSpec>;
#[doc = "CMPSS Comparator Hysteresis Control Register"]
pub mod comphysctl;
#[doc = "COMPSTS (rw) register accessor: CMPSS Comparator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compsts`] module"]
#[doc(alias = "COMPSTS")]
pub type Compsts = crate::Reg<compsts::CompstsSpec>;
#[doc = "CMPSS Comparator Status Register"]
pub mod compsts;
#[doc = "COMPSTSCLR (rw) register accessor: CMPSS Comparator Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compstsclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compstsclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compstsclr`] module"]
#[doc(alias = "COMPSTSCLR")]
pub type Compstsclr = crate::Reg<compstsclr::CompstsclrSpec>;
#[doc = "CMPSS Comparator Status Clear Register"]
pub mod compstsclr;
#[doc = "COMPDACHCTL (rw) register accessor: CMPSS High DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compdachctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compdachctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compdachctl`] module"]
#[doc(alias = "COMPDACHCTL")]
pub type Compdachctl = crate::Reg<compdachctl::CompdachctlSpec>;
#[doc = "CMPSS High DAC Control Register"]
pub mod compdachctl;
#[doc = "DACHVALS (rw) register accessor: CMPSS High DAC Value Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dachvals::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dachvals::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dachvals`] module"]
#[doc(alias = "DACHVALS")]
pub type Dachvals = crate::Reg<dachvals::DachvalsSpec>;
#[doc = "CMPSS High DAC Value Shadow Register"]
pub mod dachvals;
#[doc = "DACHVALA (rw) register accessor: CMPSS High DAC Value Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dachvala::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dachvala::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dachvala`] module"]
#[doc(alias = "DACHVALA")]
pub type Dachvala = crate::Reg<dachvala::DachvalaSpec>;
#[doc = "CMPSS High DAC Value Active Register"]
pub mod dachvala;
#[doc = "DACLVALS (rw) register accessor: CMPSS Low DAC Value Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daclvals::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daclvals::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daclvals`] module"]
#[doc(alias = "DACLVALS")]
pub type Daclvals = crate::Reg<daclvals::DaclvalsSpec>;
#[doc = "CMPSS Low DAC Value Shadow Register"]
pub mod daclvals;
#[doc = "DACLVALA (rw) register accessor: CMPSS Low DAC Value Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daclvala::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daclvala::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daclvala`] module"]
#[doc(alias = "DACLVALA")]
pub type Daclvala = crate::Reg<daclvala::DaclvalaSpec>;
#[doc = "CMPSS Low DAC Value Active Register"]
pub mod daclvala;
#[doc = "CTRIPLFILCTL (rw) register accessor: CTRIPL Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriplfilctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriplfilctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriplfilctl`] module"]
#[doc(alias = "CTRIPLFILCTL")]
pub type Ctriplfilctl = crate::Reg<ctriplfilctl::CtriplfilctlSpec>;
#[doc = "CTRIPL Filter Control Register"]
pub mod ctriplfilctl;
#[doc = "CTRIPLFILCLKCTL (rw) register accessor: CTRIPL Filter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriplfilclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriplfilclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriplfilclkctl`] module"]
#[doc(alias = "CTRIPLFILCLKCTL")]
pub type Ctriplfilclkctl = crate::Reg<ctriplfilclkctl::CtriplfilclkctlSpec>;
#[doc = "CTRIPL Filter Clock Control Register"]
pub mod ctriplfilclkctl;
#[doc = "CTRIPHFILCTL (rw) register accessor: CTRIPH Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriphfilctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriphfilctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriphfilctl`] module"]
#[doc(alias = "CTRIPHFILCTL")]
pub type Ctriphfilctl = crate::Reg<ctriphfilctl::CtriphfilctlSpec>;
#[doc = "CTRIPH Filter Control Register"]
pub mod ctriphfilctl;
#[doc = "CTRIPHFILCLKCTL (rw) register accessor: CTRIPH Filter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriphfilclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriphfilclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriphfilclkctl`] module"]
#[doc(alias = "CTRIPHFILCLKCTL")]
pub type Ctriphfilclkctl = crate::Reg<ctriphfilclkctl::CtriphfilclkctlSpec>;
#[doc = "CTRIPH Filter Clock Control Register"]
pub mod ctriphfilclkctl;
#[doc = "COMPLOCK (rw) register accessor: CMPSS Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`complock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`complock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@complock`] module"]
#[doc(alias = "COMPLOCK")]
pub type Complock = crate::Reg<complock::ComplockSpec>;
#[doc = "CMPSS Lock Register"]
pub mod complock;
#[doc = "COMPDACLCTL (rw) register accessor: CMPSS Low DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compdaclctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compdaclctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compdaclctl`] module"]
#[doc(alias = "COMPDACLCTL")]
pub type Compdaclctl = crate::Reg<compdaclctl::CompdaclctlSpec>;
#[doc = "CMPSS Low DAC Control Register"]
pub mod compdaclctl;
#[doc = "CTRIPLFILCLKCTL2 (rw) register accessor: CTRIPL Filter Clock Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriplfilclkctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriplfilclkctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriplfilclkctl2`] module"]
#[doc(alias = "CTRIPLFILCLKCTL2")]
pub type Ctriplfilclkctl2 = crate::Reg<ctriplfilclkctl2::Ctriplfilclkctl2Spec>;
#[doc = "CTRIPL Filter Clock Control Register 2"]
pub mod ctriplfilclkctl2;
#[doc = "CTRIPHFILCLKCTL2 (rw) register accessor: CTRIPH Filter Clock Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctriphfilclkctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctriphfilclkctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctriphfilclkctl2`] module"]
#[doc(alias = "CTRIPHFILCLKCTL2")]
pub type Ctriphfilclkctl2 = crate::Reg<ctriphfilclkctl2::Ctriphfilclkctl2Spec>;
#[doc = "CTRIPH Filter Clock Control Register 2"]
pub mod ctriphfilclkctl2;

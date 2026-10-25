#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    tsctr: Tsctr,
    ctrphs: Ctrphs,
    cap1: Cap1,
    cap2: Cap2,
    cap3: Cap3,
    cap4: Cap4,
    _reserved6: [u8; 0x0c],
    ecctl0: Ecctl0,
    ecctl1: Ecctl1,
    ecctl2: Ecctl2,
    eceint: Eceint,
    ecflg: Ecflg,
    ecclr: Ecclr,
    ecfrc: Ecfrc,
    _reserved13: [u8; 0x08],
    ecapsyncinsel: Ecapsyncinsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Time-Stamp Counter"]
    #[inline(always)]
    pub const fn tsctr(&self) -> &Tsctr {
        &self.tsctr
    }
    #[doc = "0x04 - Counter Phase Offset Value Register"]
    #[inline(always)]
    pub const fn ctrphs(&self) -> &Ctrphs {
        &self.ctrphs
    }
    #[doc = "0x08 - Capture 1 Register"]
    #[inline(always)]
    pub const fn cap1(&self) -> &Cap1 {
        &self.cap1
    }
    #[doc = "0x0c - Capture 2 Register"]
    #[inline(always)]
    pub const fn cap2(&self) -> &Cap2 {
        &self.cap2
    }
    #[doc = "0x10 - Capture 3 Register"]
    #[inline(always)]
    pub const fn cap3(&self) -> &Cap3 {
        &self.cap3
    }
    #[doc = "0x14 - Capture 4 Register"]
    #[inline(always)]
    pub const fn cap4(&self) -> &Cap4 {
        &self.cap4
    }
    #[doc = "0x24 - Capture Control Register 0"]
    #[inline(always)]
    pub const fn ecctl0(&self) -> &Ecctl0 {
        &self.ecctl0
    }
    #[doc = "0x28 - Capture Control Register 1"]
    #[inline(always)]
    pub const fn ecctl1(&self) -> &Ecctl1 {
        &self.ecctl1
    }
    #[doc = "0x2a - Capture Control Register 2"]
    #[inline(always)]
    pub const fn ecctl2(&self) -> &Ecctl2 {
        &self.ecctl2
    }
    #[doc = "0x2c - Capture Interrupt Enable Register"]
    #[inline(always)]
    pub const fn eceint(&self) -> &Eceint {
        &self.eceint
    }
    #[doc = "0x2e - Capture Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ecflg(&self) -> &Ecflg {
        &self.ecflg
    }
    #[doc = "0x30 - Capture Interrupt Clear Register"]
    #[inline(always)]
    pub const fn ecclr(&self) -> &Ecclr {
        &self.ecclr
    }
    #[doc = "0x32 - Capture Interrupt Force Register"]
    #[inline(always)]
    pub const fn ecfrc(&self) -> &Ecfrc {
        &self.ecfrc
    }
    #[doc = "0x3c - SYNC source select register"]
    #[inline(always)]
    pub const fn ecapsyncinsel(&self) -> &Ecapsyncinsel {
        &self.ecapsyncinsel
    }
}
#[doc = "TSCTR (rw) register accessor: Time-Stamp Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsctr`] module"]
#[doc(alias = "TSCTR")]
pub type Tsctr = crate::Reg<tsctr::TsctrSpec>;
#[doc = "Time-Stamp Counter"]
pub mod tsctr;
#[doc = "CTRPHS (rw) register accessor: Counter Phase Offset Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrphs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrphs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrphs`] module"]
#[doc(alias = "CTRPHS")]
pub type Ctrphs = crate::Reg<ctrphs::CtrphsSpec>;
#[doc = "Counter Phase Offset Value Register"]
pub mod ctrphs;
#[doc = "CAP1 (rw) register accessor: Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap1`] module"]
#[doc(alias = "CAP1")]
pub type Cap1 = crate::Reg<cap1::Cap1Spec>;
#[doc = "Capture 1 Register"]
pub mod cap1;
#[doc = "CAP2 (rw) register accessor: Capture 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap2`] module"]
#[doc(alias = "CAP2")]
pub type Cap2 = crate::Reg<cap2::Cap2Spec>;
#[doc = "Capture 2 Register"]
pub mod cap2;
#[doc = "CAP3 (rw) register accessor: Capture 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap3`] module"]
#[doc(alias = "CAP3")]
pub type Cap3 = crate::Reg<cap3::Cap3Spec>;
#[doc = "Capture 3 Register"]
pub mod cap3;
#[doc = "CAP4 (rw) register accessor: Capture 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap4`] module"]
#[doc(alias = "CAP4")]
pub type Cap4 = crate::Reg<cap4::Cap4Spec>;
#[doc = "Capture 4 Register"]
pub mod cap4;
#[doc = "ECCTL0 (rw) register accessor: Capture Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecctl0`] module"]
#[doc(alias = "ECCTL0")]
pub type Ecctl0 = crate::Reg<ecctl0::Ecctl0Spec>;
#[doc = "Capture Control Register 0"]
pub mod ecctl0;
#[doc = "ECCTL1 (rw) register accessor: Capture Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecctl1`] module"]
#[doc(alias = "ECCTL1")]
pub type Ecctl1 = crate::Reg<ecctl1::Ecctl1Spec>;
#[doc = "Capture Control Register 1"]
pub mod ecctl1;
#[doc = "ECCTL2 (rw) register accessor: Capture Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecctl2`] module"]
#[doc(alias = "ECCTL2")]
pub type Ecctl2 = crate::Reg<ecctl2::Ecctl2Spec>;
#[doc = "Capture Control Register 2"]
pub mod ecctl2;
#[doc = "ECEINT (rw) register accessor: Capture Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eceint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eceint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eceint`] module"]
#[doc(alias = "ECEINT")]
pub type Eceint = crate::Reg<eceint::EceintSpec>;
#[doc = "Capture Interrupt Enable Register"]
pub mod eceint;
#[doc = "ECFLG (rw) register accessor: Capture Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecflg`] module"]
#[doc(alias = "ECFLG")]
pub type Ecflg = crate::Reg<ecflg::EcflgSpec>;
#[doc = "Capture Interrupt Flag Register"]
pub mod ecflg;
#[doc = "ECCLR (rw) register accessor: Capture Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecclr`] module"]
#[doc(alias = "ECCLR")]
pub type Ecclr = crate::Reg<ecclr::EcclrSpec>;
#[doc = "Capture Interrupt Clear Register"]
pub mod ecclr;
#[doc = "ECFRC (rw) register accessor: Capture Interrupt Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecfrc`] module"]
#[doc(alias = "ECFRC")]
pub type Ecfrc = crate::Reg<ecfrc::EcfrcSpec>;
#[doc = "Capture Interrupt Force Register"]
pub mod ecfrc;
#[doc = "ECAPSYNCINSEL (rw) register accessor: SYNC source select register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecapsyncinsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecapsyncinsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecapsyncinsel`] module"]
#[doc(alias = "ECAPSYNCINSEL")]
pub type Ecapsyncinsel = crate::Reg<ecapsyncinsel::EcapsyncinselSpec>;
#[doc = "SYNC source select register"]
pub mod ecapsyncinsel;

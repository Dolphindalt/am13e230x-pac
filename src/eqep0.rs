#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    qposcnt: Qposcnt,
    qposinit: Qposinit,
    qposmax: Qposmax,
    qposcmp: Qposcmp,
    qposilat: Qposilat,
    qposslat: Qposslat,
    qposlat: Qposlat,
    qutmr: Qutmr,
    quprd: Quprd,
    qwdtmr: Qwdtmr,
    qwdprd: Qwdprd,
    qdecctl: Qdecctl,
    qepctl: Qepctl,
    qcapctl: Qcapctl,
    qposctl: Qposctl,
    qeint: Qeint,
    qflg: Qflg,
    qclr: Qclr,
    qfrc: Qfrc,
    qepsts: Qepsts,
    qctmr: Qctmr,
    qcprd: Qcprd,
    qctmrlat: Qctmrlat,
    qcprdlat: Qcprdlat,
    _reserved24: [u8; 0x1e],
    rev: Rev,
    qepstrobesel: Qepstrobesel,
    qmactrl: Qmactrl,
    qepsrcsel: Qepsrcsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Position Counter"]
    #[inline(always)]
    pub const fn qposcnt(&self) -> &Qposcnt {
        &self.qposcnt
    }
    #[doc = "0x04 - Position Counter Init"]
    #[inline(always)]
    pub const fn qposinit(&self) -> &Qposinit {
        &self.qposinit
    }
    #[doc = "0x08 - Maximum Position Count"]
    #[inline(always)]
    pub const fn qposmax(&self) -> &Qposmax {
        &self.qposmax
    }
    #[doc = "0x0c - Position Compare"]
    #[inline(always)]
    pub const fn qposcmp(&self) -> &Qposcmp {
        &self.qposcmp
    }
    #[doc = "0x10 - Index Position Latch"]
    #[inline(always)]
    pub const fn qposilat(&self) -> &Qposilat {
        &self.qposilat
    }
    #[doc = "0x14 - Strobe Position Latch"]
    #[inline(always)]
    pub const fn qposslat(&self) -> &Qposslat {
        &self.qposslat
    }
    #[doc = "0x18 - Position Latch"]
    #[inline(always)]
    pub const fn qposlat(&self) -> &Qposlat {
        &self.qposlat
    }
    #[doc = "0x1c - QEP Unit Timer"]
    #[inline(always)]
    pub const fn qutmr(&self) -> &Qutmr {
        &self.qutmr
    }
    #[doc = "0x20 - QEP Unit Period"]
    #[inline(always)]
    pub const fn quprd(&self) -> &Quprd {
        &self.quprd
    }
    #[doc = "0x24 - QEP Watchdog Timer"]
    #[inline(always)]
    pub const fn qwdtmr(&self) -> &Qwdtmr {
        &self.qwdtmr
    }
    #[doc = "0x26 - QEP Watchdog Period"]
    #[inline(always)]
    pub const fn qwdprd(&self) -> &Qwdprd {
        &self.qwdprd
    }
    #[doc = "0x28 - Quadrature Decoder Control"]
    #[inline(always)]
    pub const fn qdecctl(&self) -> &Qdecctl {
        &self.qdecctl
    }
    #[doc = "0x2a - QEP Control"]
    #[inline(always)]
    pub const fn qepctl(&self) -> &Qepctl {
        &self.qepctl
    }
    #[doc = "0x2c - Qaudrature Capture Control"]
    #[inline(always)]
    pub const fn qcapctl(&self) -> &Qcapctl {
        &self.qcapctl
    }
    #[doc = "0x2e - Position Compare Control"]
    #[inline(always)]
    pub const fn qposctl(&self) -> &Qposctl {
        &self.qposctl
    }
    #[doc = "0x30 - QEP Interrupt Control"]
    #[inline(always)]
    pub const fn qeint(&self) -> &Qeint {
        &self.qeint
    }
    #[doc = "0x32 - QEP Interrupt Flag"]
    #[inline(always)]
    pub const fn qflg(&self) -> &Qflg {
        &self.qflg
    }
    #[doc = "0x34 - QEP Interrupt Clear"]
    #[inline(always)]
    pub const fn qclr(&self) -> &Qclr {
        &self.qclr
    }
    #[doc = "0x36 - QEP Interrupt Force"]
    #[inline(always)]
    pub const fn qfrc(&self) -> &Qfrc {
        &self.qfrc
    }
    #[doc = "0x38 - QEP Status"]
    #[inline(always)]
    pub const fn qepsts(&self) -> &Qepsts {
        &self.qepsts
    }
    #[doc = "0x3a - QEP Capture Timer"]
    #[inline(always)]
    pub const fn qctmr(&self) -> &Qctmr {
        &self.qctmr
    }
    #[doc = "0x3c - QEP Capture Period"]
    #[inline(always)]
    pub const fn qcprd(&self) -> &Qcprd {
        &self.qcprd
    }
    #[doc = "0x3e - QEP Capture Latch"]
    #[inline(always)]
    pub const fn qctmrlat(&self) -> &Qctmrlat {
        &self.qctmrlat
    }
    #[doc = "0x40 - QEP Capture Period Latch"]
    #[inline(always)]
    pub const fn qcprdlat(&self) -> &Qcprdlat {
        &self.qcprdlat
    }
    #[doc = "0x60 - QEP Revision Number"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x64 - QEP Strobe select register"]
    #[inline(always)]
    pub const fn qepstrobesel(&self) -> &Qepstrobesel {
        &self.qepstrobesel
    }
    #[doc = "0x68 - QMA Control register"]
    #[inline(always)]
    pub const fn qmactrl(&self) -> &Qmactrl {
        &self.qmactrl
    }
    #[doc = "0x6c - QEP Source Select Register"]
    #[inline(always)]
    pub const fn qepsrcsel(&self) -> &Qepsrcsel {
        &self.qepsrcsel
    }
}
#[doc = "QPOSCNT (rw) register accessor: Position Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`qposcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposcnt`] module"]
#[doc(alias = "QPOSCNT")]
pub type Qposcnt = crate::Reg<qposcnt::QposcntSpec>;
#[doc = "Position Counter"]
pub mod qposcnt;
#[doc = "QPOSINIT (rw) register accessor: Position Counter Init\n\nYou can [`read`](crate::Reg::read) this register and get [`qposinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposinit`] module"]
#[doc(alias = "QPOSINIT")]
pub type Qposinit = crate::Reg<qposinit::QposinitSpec>;
#[doc = "Position Counter Init"]
pub mod qposinit;
#[doc = "QPOSMAX (rw) register accessor: Maximum Position Count\n\nYou can [`read`](crate::Reg::read) this register and get [`qposmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposmax`] module"]
#[doc(alias = "QPOSMAX")]
pub type Qposmax = crate::Reg<qposmax::QposmaxSpec>;
#[doc = "Maximum Position Count"]
pub mod qposmax;
#[doc = "QPOSCMP (rw) register accessor: Position Compare\n\nYou can [`read`](crate::Reg::read) this register and get [`qposcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposcmp`] module"]
#[doc(alias = "QPOSCMP")]
pub type Qposcmp = crate::Reg<qposcmp::QposcmpSpec>;
#[doc = "Position Compare"]
pub mod qposcmp;
#[doc = "QPOSILAT (rw) register accessor: Index Position Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qposilat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposilat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposilat`] module"]
#[doc(alias = "QPOSILAT")]
pub type Qposilat = crate::Reg<qposilat::QposilatSpec>;
#[doc = "Index Position Latch"]
pub mod qposilat;
#[doc = "QPOSSLAT (rw) register accessor: Strobe Position Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qposslat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposslat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposslat`] module"]
#[doc(alias = "QPOSSLAT")]
pub type Qposslat = crate::Reg<qposslat::QposslatSpec>;
#[doc = "Strobe Position Latch"]
pub mod qposslat;
#[doc = "QPOSLAT (rw) register accessor: Position Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qposlat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposlat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposlat`] module"]
#[doc(alias = "QPOSLAT")]
pub type Qposlat = crate::Reg<qposlat::QposlatSpec>;
#[doc = "Position Latch"]
pub mod qposlat;
#[doc = "QUTMR (rw) register accessor: QEP Unit Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qutmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qutmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qutmr`] module"]
#[doc(alias = "QUTMR")]
pub type Qutmr = crate::Reg<qutmr::QutmrSpec>;
#[doc = "QEP Unit Timer"]
pub mod qutmr;
#[doc = "QUPRD (rw) register accessor: QEP Unit Period\n\nYou can [`read`](crate::Reg::read) this register and get [`quprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quprd`] module"]
#[doc(alias = "QUPRD")]
pub type Quprd = crate::Reg<quprd::QuprdSpec>;
#[doc = "QEP Unit Period"]
pub mod quprd;
#[doc = "QWDTMR (rw) register accessor: QEP Watchdog Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qwdtmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwdtmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qwdtmr`] module"]
#[doc(alias = "QWDTMR")]
pub type Qwdtmr = crate::Reg<qwdtmr::QwdtmrSpec>;
#[doc = "QEP Watchdog Timer"]
pub mod qwdtmr;
#[doc = "QWDPRD (rw) register accessor: QEP Watchdog Period\n\nYou can [`read`](crate::Reg::read) this register and get [`qwdprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwdprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qwdprd`] module"]
#[doc(alias = "QWDPRD")]
pub type Qwdprd = crate::Reg<qwdprd::QwdprdSpec>;
#[doc = "QEP Watchdog Period"]
pub mod qwdprd;
#[doc = "QDECCTL (rw) register accessor: Quadrature Decoder Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qdecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdecctl`] module"]
#[doc(alias = "QDECCTL")]
pub type Qdecctl = crate::Reg<qdecctl::QdecctlSpec>;
#[doc = "Quadrature Decoder Control"]
pub mod qdecctl;
#[doc = "QEPCTL (rw) register accessor: QEP Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qepctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepctl`] module"]
#[doc(alias = "QEPCTL")]
pub type Qepctl = crate::Reg<qepctl::QepctlSpec>;
#[doc = "QEP Control"]
pub mod qepctl;
#[doc = "QCAPCTL (rw) register accessor: Qaudrature Capture Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qcapctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcapctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcapctl`] module"]
#[doc(alias = "QCAPCTL")]
pub type Qcapctl = crate::Reg<qcapctl::QcapctlSpec>;
#[doc = "Qaudrature Capture Control"]
pub mod qcapctl;
#[doc = "QPOSCTL (rw) register accessor: Position Compare Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qposctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qposctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposctl`] module"]
#[doc(alias = "QPOSCTL")]
pub type Qposctl = crate::Reg<qposctl::QposctlSpec>;
#[doc = "Position Compare Control"]
pub mod qposctl;
#[doc = "QEINT (rw) register accessor: QEP Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qeint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeint`] module"]
#[doc(alias = "QEINT")]
pub type Qeint = crate::Reg<qeint::QeintSpec>;
#[doc = "QEP Interrupt Control"]
pub mod qeint;
#[doc = "QFLG (rw) register accessor: QEP Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`qflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qflg`] module"]
#[doc(alias = "QFLG")]
pub type Qflg = crate::Reg<qflg::QflgSpec>;
#[doc = "QEP Interrupt Flag"]
pub mod qflg;
#[doc = "QCLR (rw) register accessor: QEP Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`qclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qclr`] module"]
#[doc(alias = "QCLR")]
pub type Qclr = crate::Reg<qclr::QclrSpec>;
#[doc = "QEP Interrupt Clear"]
pub mod qclr;
#[doc = "QFRC (rw) register accessor: QEP Interrupt Force\n\nYou can [`read`](crate::Reg::read) this register and get [`qfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qfrc`] module"]
#[doc(alias = "QFRC")]
pub type Qfrc = crate::Reg<qfrc::QfrcSpec>;
#[doc = "QEP Interrupt Force"]
pub mod qfrc;
#[doc = "QEPSTS (rw) register accessor: QEP Status\n\nYou can [`read`](crate::Reg::read) this register and get [`qepsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepsts`] module"]
#[doc(alias = "QEPSTS")]
pub type Qepsts = crate::Reg<qepsts::QepstsSpec>;
#[doc = "QEP Status"]
pub mod qepsts;
#[doc = "QCTMR (rw) register accessor: QEP Capture Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`qctmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qctmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctmr`] module"]
#[doc(alias = "QCTMR")]
pub type Qctmr = crate::Reg<qctmr::QctmrSpec>;
#[doc = "QEP Capture Timer"]
pub mod qctmr;
#[doc = "QCPRD (rw) register accessor: QEP Capture Period\n\nYou can [`read`](crate::Reg::read) this register and get [`qcprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcprd`] module"]
#[doc(alias = "QCPRD")]
pub type Qcprd = crate::Reg<qcprd::QcprdSpec>;
#[doc = "QEP Capture Period"]
pub mod qcprd;
#[doc = "QCTMRLAT (rw) register accessor: QEP Capture Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qctmrlat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qctmrlat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctmrlat`] module"]
#[doc(alias = "QCTMRLAT")]
pub type Qctmrlat = crate::Reg<qctmrlat::QctmrlatSpec>;
#[doc = "QEP Capture Latch"]
pub mod qctmrlat;
#[doc = "QCPRDLAT (rw) register accessor: QEP Capture Period Latch\n\nYou can [`read`](crate::Reg::read) this register and get [`qcprdlat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcprdlat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcprdlat`] module"]
#[doc(alias = "QCPRDLAT")]
pub type Qcprdlat = crate::Reg<qcprdlat::QcprdlatSpec>;
#[doc = "QEP Capture Period Latch"]
pub mod qcprdlat;
#[doc = "REV (rw) register accessor: QEP Revision Number\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`] module"]
#[doc(alias = "REV")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "QEP Revision Number"]
pub mod rev;
#[doc = "QEPSTROBESEL (rw) register accessor: QEP Strobe select register\n\nYou can [`read`](crate::Reg::read) this register and get [`qepstrobesel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepstrobesel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepstrobesel`] module"]
#[doc(alias = "QEPSTROBESEL")]
pub type Qepstrobesel = crate::Reg<qepstrobesel::QepstrobeselSpec>;
#[doc = "QEP Strobe select register"]
pub mod qepstrobesel;
#[doc = "QMACTRL (rw) register accessor: QMA Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`qmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qmactrl`] module"]
#[doc(alias = "QMACTRL")]
pub type Qmactrl = crate::Reg<qmactrl::QmactrlSpec>;
#[doc = "QMA Control register"]
pub mod qmactrl;
#[doc = "QEPSRCSEL (rw) register accessor: QEP Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qepsrcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qepsrcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepsrcsel`] module"]
#[doc(alias = "QEPSRCSEL")]
pub type Qepsrcsel = crate::Reg<qepsrcsel::QepsrcselSpec>;
#[doc = "QEP Source Select Register"]
pub mod qepsrcsel;

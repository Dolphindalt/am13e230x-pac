#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pgactl: Pgactl,
    _reserved1: [u8; 0x02],
    muxsel: Muxsel,
    _reserved2: [u8; 0x0a],
    pgatype: Pgatype,
    _reserved3: [u8; 0x02],
    pgalock: Pgalock,
}
impl RegisterBlock {
    #[doc = "0x00 - PGA Control Register"]
    #[inline(always)]
    pub const fn pgactl(&self) -> &Pgactl {
        &self.pgactl
    }
    #[doc = "0x04 - Mux Selection Register"]
    #[inline(always)]
    pub const fn muxsel(&self) -> &Muxsel {
        &self.muxsel
    }
    #[doc = "0x10 - PGA Type Register"]
    #[inline(always)]
    pub const fn pgatype(&self) -> &Pgatype {
        &self.pgatype
    }
    #[doc = "0x14 - PGA Lock Register"]
    #[inline(always)]
    pub const fn pgalock(&self) -> &Pgalock {
        &self.pgalock
    }
}
#[doc = "PGACTL (rw) register accessor: PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgactl`] module"]
#[doc(alias = "PGACTL")]
pub type Pgactl = crate::Reg<pgactl::PgactlSpec>;
#[doc = "PGA Control Register"]
pub mod pgactl;
#[doc = "MUXSEL (rw) register accessor: Mux Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsel`] module"]
#[doc(alias = "MUXSEL")]
pub type Muxsel = crate::Reg<muxsel::MuxselSpec>;
#[doc = "Mux Selection Register"]
pub mod muxsel;
#[doc = "PGATYPE (rw) register accessor: PGA Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgatype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgatype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgatype`] module"]
#[doc(alias = "PGATYPE")]
pub type Pgatype = crate::Reg<pgatype::PgatypeSpec>;
#[doc = "PGA Type Register"]
pub mod pgatype;
#[doc = "PGALOCK (rw) register accessor: PGA Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgalock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgalock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgalock`] module"]
#[doc(alias = "PGALOCK")]
pub type Pgalock = crate::Reg<pgalock::PgalockSpec>;
#[doc = "PGA Lock Register"]
pub mod pgalock;

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x6004],
    pincm: Pincm,
    _reserved1: [u8; 0x01f8],
    fupdate: Fupdate,
    _reserved2: [u8; 0x0274],
    cpu_connect_0: CpuConnect0,
    _reserved3: [u8; 0x037c],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved5: [u8; 0x0c],
    stat: Stat,
    _reserved6: [u8; 0x07e8],
    mcan_core: McanCore,
    _reserved7: [u8; 0x0104],
    ti_wrapper: TiWrapper,
}
impl RegisterBlock {
    #[doc = "0x6004..0x600c - PINCM"]
    #[inline(always)]
    pub const fn pincm(&self) -> &Pincm {
        &self.pincm
    }
    #[doc = "0x6204..0x620c - FUPDATE"]
    #[inline(always)]
    pub const fn fupdate(&self) -> &Fupdate {
        &self.fupdate
    }
    #[doc = "0x6480 - CPU Connect"]
    #[inline(always)]
    pub const fn cpu_connect_0(&self) -> &CpuConnect0 {
        &self.cpu_connect_0
    }
    #[doc = "0x6800 - Power enable"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x6804 - Reset Control"]
    #[inline(always)]
    pub const fn rstctl(&self) -> &Rstctl {
        &self.rstctl
    }
    #[doc = "0x6814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x7000..0x70fc - MCAN_CORE"]
    #[inline(always)]
    pub const fn mcan_core(&self) -> &McanCore {
        &self.mcan_core
    }
    #[doc = "0x7200..0x7910 - TI_WRAPPER"]
    #[inline(always)]
    pub const fn ti_wrapper(&self) -> &TiWrapper {
        &self.ti_wrapper
    }
}
#[doc = "PINCM"]
pub use self::pincm::Pincm;
#[doc = r"Cluster"]
#[doc = "PINCM"]
pub mod pincm;
#[doc = "FUPDATE"]
pub use self::fupdate::Fupdate;
#[doc = r"Cluster"]
#[doc = "FUPDATE"]
pub mod fupdate;
#[doc = "CPU_CONNECT_0 (rw) register accessor: CPU Connect\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_connect_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_connect_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_connect_0`] module"]
#[doc(alias = "CPU_CONNECT_0")]
pub type CpuConnect0 = crate::Reg<cpu_connect_0::CpuConnect0Spec>;
#[doc = "CPU Connect"]
pub mod cpu_connect_0;
#[doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`] module"]
#[doc(alias = "PWREN")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "Power enable"]
pub mod pwren;
#[doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`] module"]
#[doc(alias = "RSTCTL")]
pub type Rstctl = crate::Reg<rstctl::RstctlSpec>;
#[doc = "Reset Control"]
pub mod rstctl;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "MCAN_CORE"]
pub use self::mcan_core::McanCore;
#[doc = r"Cluster"]
#[doc = "MCAN_CORE"]
pub mod mcan_core;
#[doc = "TI_WRAPPER"]
pub use self::ti_wrapper::TiWrapper;
#[doc = r"Cluster"]
#[doc = "TI_WRAPPER"]
pub mod ti_wrapper;

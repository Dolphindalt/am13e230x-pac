#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    cpu_int: CpuInt,
    _reserved1: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved2: [u8; 0x18],
    desc: Desc,
    txd: Txd,
    txctl: Txctl,
    rxd: Rxd,
    rxctl: Rxctl,
    _reserved7: [u8; 0xf0],
    special_auth: SpecialAuth,
    _reserved8: [u8; 0x0c],
    app_auth: AppAuth,
}
impl RegisterBlock {
    #[doc = "0x1020..0x104c - CPU_INT"]
    #[inline(always)]
    pub const fn cpu_int(&self) -> &CpuInt {
        &self.cpu_int
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - Transmit data register"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x1104 - Transmit control register"]
    #[inline(always)]
    pub const fn txctl(&self) -> &Txctl {
        &self.txctl
    }
    #[doc = "0x1108 - Receive data register"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x110c - Receive control register"]
    #[inline(always)]
    pub const fn rxctl(&self) -> &Rxctl {
        &self.rxctl
    }
    #[doc = "0x1200 - Special enable authorization register"]
    #[inline(always)]
    pub const fn special_auth(&self) -> &SpecialAuth {
        &self.special_auth
    }
    #[doc = "0x1210 - Application CPU0 authorization register"]
    #[inline(always)]
    pub const fn app_auth(&self) -> &AppAuth {
        &self.app_auth
    }
}
#[doc = "CPU_INT"]
pub use self::cpu_int::CpuInt;
#[doc = r"Cluster"]
#[doc = "CPU_INT"]
pub mod cpu_int;
#[doc = "EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`] module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`] module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "TXD (r) register accessor: Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`] module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "Transmit data register"]
pub mod txd;
#[doc = "TXCTL (r) register accessor: Transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctl`] module"]
#[doc(alias = "TXCTL")]
pub type Txctl = crate::Reg<txctl::TxctlSpec>;
#[doc = "Transmit control register"]
pub mod txctl;
#[doc = "RXD (rw) register accessor: Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`] module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "Receive data register"]
pub mod rxd;
#[doc = "RXCTL (rw) register accessor: Receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctl`] module"]
#[doc(alias = "RXCTL")]
pub type Rxctl = crate::Reg<rxctl::RxctlSpec>;
#[doc = "Receive control register"]
pub mod rxctl;
#[doc = "SPECIAL_AUTH (r) register accessor: Special enable authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`special_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth`] module"]
#[doc(alias = "SPECIAL_AUTH")]
pub type SpecialAuth = crate::Reg<special_auth::SpecialAuthSpec>;
#[doc = "Special enable authorization register"]
pub mod special_auth;
#[doc = "APP_AUTH (r) register accessor: Application CPU0 authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth`] module"]
#[doc(alias = "APP_AUTH")]
pub type AppAuth = crate::Reg<app_auth::AppAuthSpec>;
#[doc = "Application CPU0 authorization register"]
pub mod app_auth;

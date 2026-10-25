#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    gprcm: Gprcm,
    _reserved1: [u8; 0x0800],
    pdbgctl: Pdbgctl,
    _reserved2: [u8; 0x04],
    cpu_int: CpuInt,
    _reserved3: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved4: [u8; 0x18],
    desc: Desc,
    wwdtctl0: Wwdtctl0,
    wwdtctl1: Wwdtctl1,
    wwdtcntrst: Wwdtcntrst,
    wwdtstat: Wwdtstat,
}
impl RegisterBlock {
    #[doc = "0x800..0x818 - GPRCM"]
    #[inline(always)]
    pub const fn gprcm(&self) -> &Gprcm {
        &self.gprcm
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
    }
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
    #[doc = "0x1100 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdtctl0(&self) -> &Wwdtctl0 {
        &self.wwdtctl0
    }
    #[doc = "0x1104 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdtctl1(&self) -> &Wwdtctl1 {
        &self.wwdtctl1
    }
    #[doc = "0x1108 - Window Watchdog Timer Counter Reset Register"]
    #[inline(always)]
    pub const fn wwdtcntrst(&self) -> &Wwdtcntrst {
        &self.wwdtcntrst
    }
    #[doc = "0x110c - Window Watchdog Timer Status Register"]
    #[inline(always)]
    pub const fn wwdtstat(&self) -> &Wwdtstat {
        &self.wwdtstat
    }
}
#[doc = "GPRCM"]
pub use self::gprcm::Gprcm;
#[doc = r"Cluster"]
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`] module"]
#[doc(alias = "PDBGCTL")]
pub type Pdbgctl = crate::Reg<pdbgctl::PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod pdbgctl;
#[doc = "CPU_INT"]
pub use self::cpu_int::CpuInt;
#[doc = r"Cluster"]
#[doc = "CPU_INT"]
pub mod cpu_int;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`] module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`] module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "WWDTCTL0 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtctl0`] module"]
#[doc(alias = "WWDTCTL0")]
pub type Wwdtctl0 = crate::Reg<wwdtctl0::Wwdtctl0Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdtctl0;
#[doc = "WWDTCTL1 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtctl1`] module"]
#[doc(alias = "WWDTCTL1")]
pub type Wwdtctl1 = crate::Reg<wwdtctl1::Wwdtctl1Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdtctl1;
#[doc = "WWDTCNTRST (rw) register accessor: Window Watchdog Timer Counter Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtcntrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtcntrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtcntrst`] module"]
#[doc(alias = "WWDTCNTRST")]
pub type Wwdtcntrst = crate::Reg<wwdtcntrst::WwdtcntrstSpec>;
#[doc = "Window Watchdog Timer Counter Reset Register"]
pub mod wwdtcntrst;
#[doc = "WWDTSTAT (r) register accessor: Window Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtstat`] module"]
#[doc(alias = "WWDTSTAT")]
pub type Wwdtstat = crate::Reg<wwdtstat::WwdtstatSpec>;
#[doc = "Window Watchdog Timer Status Register"]
pub mod wwdtstat;
